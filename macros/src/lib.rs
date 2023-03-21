use std::{collections::BTreeMap, str::FromStr};

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, ToTokens};
use syn::{
    parse::Parser, parse_macro_input, parse_quote, punctuated::Punctuated, Data, DataStruct,
    DeriveInput, Expr, ExprLit, Field, Lit, LitStr, Meta, MetaList, MetaNameValue, Token,
};

#[proc_macro_attribute]
pub fn rq(attr: TokenStream, input: TokenStream) -> TokenStream {
    let DeriveInput { data: Data::Struct(DataStruct {fields, ..}), .. } = parse_macro_input!(input) else {
        panic!("Expected a struct");
    };

    let parser = Punctuated::<MetaNameValue, Token![,]>::parse_separated_nonempty;
    let substructs_names_docs = parser.parse(attr).unwrap();
    let substructs_names_docs = substructs_names_docs
        .iter()
        .map(|meta| {
            let name = meta.path.get_ident().unwrap();
            let Expr::Lit(ExprLit { lit: Lit::Str(doc_str), .. }) = &meta.value else {
                panic!("Expected a string literal");
            };
            let doc = quote::quote!(#[doc = #doc_str]);

            (name, doc)
        })
        .collect::<Vec<_>>();

    assert!(
        !substructs_names_docs.is_empty(),
        "Expected exactly two substructs"
    );

    let substructs_fields =
        fields
            .into_iter()
            .fold(BTreeMap::new(), |mut substructs_fields, mut field| {
                let rq_attrs = field
                    .attrs
                    .iter()
                    .enumerate()
                    .filter_map(|(index, attr)| {
                        attr.meta
                            .require_list()
                            .ok()
                            .and_then(|meta| meta.path.is_ident("rq").then_some((index, meta)))
                    })
                    .collect::<Vec<_>>();

                if rq_attrs.is_empty() {
                    return substructs_fields;
                }

                assert_eq!(
                    rq_attrs.len(),
                    1,
                    "Expected exactly one #[rq(...)] attribute"
                );

                let (rq_attr_ind, rq_attr) = &rq_attrs[0];

                let on_substructs = rq_attr.parse_args::<MetaList>().unwrap();
                assert!(on_substructs.path.is_ident("on"), "Expected #[rq(on(...))]");

                let on_substructs_names_req = on_substructs
                    .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                    .unwrap()
                    .into_iter()
                    .map(|meta| match meta {
                        Meta::Path(ident) => (ident.get_ident().cloned().unwrap(), false),
                        Meta::List(metalist) => {
                            let ident = metalist.path.get_ident().cloned().unwrap();

                            assert_eq!(metalist.tokens.to_string().as_str(), "req");

                            (ident, true)
                        }
                        Meta::NameValue(_) => {
                            panic!("Expected a struct name or a StructName(req)")
                        }
                    })
                    .collect::<Vec<_>>();

                field.attrs.remove(*rq_attr_ind);
                field.attrs.push(parse_quote!(#[get = "pub"]));

                if let Some(all_req) =
                    on_substructs_names_req
                        .iter()
                        .find_map(|(substruct_name, req)| {
                            (substruct_name.to_string().as_str() == "all").then_some(*req)
                        })
                {
                    substructs_names_docs
                        .iter()
                        .for_each(|(&ref substruct_name, _)| {
                            let mut field = field.clone();

                            if all_req {
                                fix_req_option(&mut field);
                            }

                            substructs_fields
                                .entry(substruct_name.clone())
                                .or_insert(Vec::new())
                                .push((field, all_req));
                        });

                    return substructs_fields;
                }

                on_substructs_names_req
                    .into_iter()
                    .for_each(|(substruct_name, req)| {
                        let mut field = field.clone();

                        if req {
                            fix_req_option(&mut field);
                        }

                        substructs_fields
                            .entry(substruct_name)
                            .or_insert(Vec::new())
                            .push((field, req));
                    });

                substructs_fields
            });

    let substructs = substructs_names_docs.iter().map(|(substruct_name, doc)| {
        let actual_substruct_name = format_ident!("{substruct_name}RequestBody");

        let fields = substructs_fields.get(&substruct_name).unwrap();

        let fields_tokens = fields.iter().map(|(f, _)| quote::quote!(#f));

        let required_fields = fields
            .iter()
            .filter_map(|(f, req)| (*req).then_some(f))
            .collect::<Vec<_>>();
        let required_fields_names = required_fields
            .iter()
            .map(|f| f.ident.as_ref().unwrap())
            .collect::<Vec<_>>();
        let non_required_fields = fields
            .iter()
            .filter_map(|(f, req)| (!(*req)).then_some(f))
            .collect::<Vec<_>>();

        let init_func_args = required_fields.iter().map(|f| {
            let ident = f.ident.as_ref().unwrap();
            let ty = &f.ty;

            quote::quote!(#ident: #ty)
        });
        let init_default_vals =
            (required_fields.len() < fields.len()).then_some(quote::quote!(, ..Default::default()));

        let init_func = quote::quote! {
            pub fn init(#(#init_func_args),*) -> Self {
                Self {
                    #(#required_fields_names),*
                    #init_default_vals
                }
            }
        };

        let with_functions = non_required_fields.iter().map(|f| {
            let mut f: Field = (*f).clone();
            fix_req_option(&mut f);

            let ident = f.ident.as_ref().unwrap();
            let fn_name = format_ident!("with_{}", ident);
            let ty = &f.ty;

            quote::quote! {
                pub fn #fn_name(mut self, #ident: #ty) -> Self {
                    self.#ident = Some(#ident);

                    self
                }
            }
        });

        let to_json_req_fields = required_fields.iter().map(|f| {
            let ident = f.ident.as_ref().unwrap();
            let ident_lit_str = LitStr::new(&ident.to_string(), Span::call_site());

            quote::quote! {
                res.insert(
                    #ident_lit_str.to_string(),
                    serde_json::value::to_value(self.#ident)?,
                );
            }
        });
        let to_json_non_req_fields = non_required_fields.iter().map(|f| {
            let ident = f.ident.as_ref().unwrap();
            let ident_lit_str = LitStr::new(&ident.to_string(), Span::call_site());

            quote::quote! {
                if let Some(#ident) = self.#ident {
                    res.insert(
                        #ident_lit_str.to_string(),
                        serde_json::value::to_value(#ident)?,
                    );
                }
            }
        });
        let to_json = quote::quote! {
            pub fn to_json(self) -> Result<serde_json::Value, serde_json::Error> {
                let mut res = serde_json::Map::new();

                #(#to_json_req_fields)*

                #(#to_json_non_req_fields)*

                Ok(serde_json::Value::Object(res))
            }
        };

        quote::quote! {
            #doc
            #[derive(Debug, Default, getset::Getters)]
            pub struct #actual_substruct_name {
                #(#fields_tokens),*
            }

            impl #actual_substruct_name {
                #init_func

                #to_json

                #(#with_functions)*
            }
        }
    });

    (quote::quote! {
        #(#substructs)*
    })
    .into()
}

fn fix_req_option(field: &mut Field) {
    let ty_str = field.ty.to_token_stream().to_string().replace(' ', "");

    if ty_str.starts_with("Option<") {
        let ty = proc_macro2::TokenStream::from_str(&ty_str[7..ty_str.len() - 1]).unwrap();

        field.ty = parse_quote!(#ty);
    }
}
