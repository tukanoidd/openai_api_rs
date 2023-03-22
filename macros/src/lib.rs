use std::{collections::BTreeMap, str::FromStr};

use miette::IntoDiagnostic;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, ToTokens};
use syn::{
    parse::Parser, parse_quote, punctuated::Punctuated, Data, DataStruct, DeriveInput, Expr,
    ExprLit, Field, Lit, LitStr, Meta, MetaList, Token,
};

struct SubstructData {
    doc: TokenStream2,
    url: LitStr,
    compatible_models: Vec<LitStr>,
}

impl Default for SubstructData {
    fn default() -> Self {
        Self {
            doc: TokenStream2::new(),
            url: LitStr::new("", Span::call_site()),
            compatible_models: Vec::new(),
        }
    }
}

#[proc_macro_attribute]
pub fn rq(attr: TokenStream, input: TokenStream) -> TokenStream {
    rq_impl(attr, input).unwrap()
}

fn rq_impl(attr: TokenStream, input: TokenStream) -> miette::Result<TokenStream> {
    let DeriveInput { data: Data::Struct(DataStruct {fields, ..}), .. } = syn::parse(input).into_diagnostic()? else {
        panic!("Expected a struct");
    };

    let parser = Punctuated::<MetaList, Token![,]>::parse_separated_nonempty;
    let substructs_names_docs = parser.parse(attr).into_diagnostic()?;
    let substructs_names_docs = substructs_names_docs
        .iter()
        .map(|meta| {
            let name = meta.path.get_ident().expect("Expected an identifier");

            let tags = meta.parse_args_with(parser).unwrap();
            let data = tags.iter().fold(SubstructData::default(), |mut data, tag| {
                if tag.path.is_ident("doc") {
                    let Expr::Lit(ExprLit { lit: Lit::Str(doc_str), .. }) = syn::parse2(tag.tokens.clone()).expect("Couldn't parse the doc") else {
                        panic!("Expected a string literal");
                    };

                    data.doc = quote::quote!(#[doc = #doc_str]);
                } else if tag.path.is_ident("url") {
                    data.url = tag.parse_args::<LitStr>().expect("Couldn't parse the url");
                } else if tag.path.is_ident("compatible_models") {
                    let models = tag
                        .parse_args_with(Punctuated::<LitStr, Token![,]>::parse_terminated)
                        .expect("Couldn't parse the compatible_models");

                    data.compatible_models = models.into_iter().collect();
                } else {
                    panic!("Expected on of these tags: ['doc', 'url', 'compatible_models']");
                }

                data
            });

            assert!(!data.doc.is_empty());
            assert!(!data.url.to_token_stream().to_string().is_empty());
            assert!(!data.compatible_models.is_empty());

            (name, data)
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

                let on_substructs = rq_attr
                    .parse_args::<MetaList>()
                    .expect("Couldn't parse the #[rq(...)] attribute");
                assert!(on_substructs.path.is_ident("on"), "Expected #[rq(on(...))]");

                let on_substructs_names_req = on_substructs
                    .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                    .expect("Couldn't parse the #[rq(on(...))] attribute")
                    .into_iter()
                    .map(|meta| match meta {
                        Meta::Path(ident) => (
                            ident
                                .get_ident()
                                .cloned()
                                .expect("Couldn't parse the variant name"),
                            false,
                        ),
                        Meta::List(metalist) => {
                            let ident = metalist
                                .path
                                .get_ident()
                                .cloned()
                                .expect("Expected a struct name or a StructName(req)");

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
                            let field = field.clone();

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
                        let field = field.clone();

                        substructs_fields
                            .entry(substruct_name)
                            .or_insert(Vec::new())
                            .push((field, req));
                    });

                substructs_fields
            });

    let substructs = substructs_names_docs.iter().map(|(substruct_name, SubstructData { doc, url, compatible_models })| {
        let actual_substruct_name = format_ident!("{substruct_name}Request");

        let fields = substructs_fields.get(&substruct_name).expect("Couldn't find the substruct fields");

        let fields_tokens = fields.iter().map(|(f, _)| quote::quote!(#f));

        let required_fields = fields
            .iter()
            .filter_map(|(f, req)| (*req).then_some(f))
            .collect::<Vec<_>>();
        let required_fields_names = required_fields
            .iter()
            .map(|f| f.ident.as_ref().expect("Expected a named field"))
            .collect::<Vec<_>>();
        let non_required_fields = fields
            .iter()
            .filter_map(|(f, req)| (!(*req)).then_some(f))
            .collect::<Vec<_>>();

        let init_func_args = required_fields.iter().map(|f| {
            let ident = f.ident.as_ref().expect("Expected a named field");
            let ty = &f.ty;

            quote::quote!(#ident: #ty)
        });
        let init_default_vals = non_required_fields.iter().map(|f| {
            let name = f.ident.as_ref().expect("Expected a named field");

            quote::quote! { #name: Default::default() }
        });

        let init_func = quote::quote! {
            pub fn init(model: &'model Model<'client>, #(#init_func_args),*) -> Self {
                Self {
                    model
                    #(,#required_fields_names)*
                    #(,#init_default_vals)*
                }
            }
        };

        let with_functions = non_required_fields.iter().map(|f| {
            let mut f: Field = (*f).clone();
            fix_req_option(&mut f).expect("Failed to fix the option stripping");

            let ident = f.ident.as_ref().expect("Expected a named field");
            let fn_name = format_ident!("with_{}", ident);
            let ty = &f.ty;

            quote::quote! {
                pub fn #fn_name(mut self, #ident: #ty) -> Self {
                    self.#ident = Some(#ident);

                    self
                }
            }
        });

        let to_json_req_fields = required_fields.iter().fold(quote::quote! {
            res.insert(
                "model".to_string(),
                serde_json::value::to_value(self.model.id().clone())?,
            );
        }, |res_tokens, f| {
            let ident = f.ident.as_ref().expect("Expected a named field");
            let ident_lit_str = LitStr::new(&ident.to_string(), Span::call_site());

            quote::quote! {
                #res_tokens

                res.insert(
                    #ident_lit_str.to_string(),
                    serde_json::value::to_value(self.#ident.clone())?,
                );
            }
        });
        let to_json_non_req_fields = non_required_fields.iter().map(|f| {
            let ident = f.ident.as_ref().expect("Expected a named field");
            let ident_lit_str = LitStr::new(&ident.to_string(), Span::call_site());

            quote::quote! {
                if let Some(#ident) = self.#ident.clone() {
                    res.insert(
                        #ident_lit_str.to_string(),
                        serde_json::value::to_value(#ident)?,
                    );
                }
            }
        });
        let to_json = quote::quote! {
            fn to_json(&self) -> serde_json::Result<serde_json::Value> {
                let mut res = serde_json::Map::<String, serde_json::Value>::new();

                #to_json_req_fields

                #(#to_json_non_req_fields)*

                Ok(serde_json::Value::Object(res))
            }
        };

        let model_error = format_ident!("NotCompatibleWith{}", substruct_name);
        let response = format_ident!("{}Response", substruct_name);

        quote::quote! {
            #doc
            #[derive(Debug, getset::Getters)]
            pub struct #actual_substruct_name<'model, 'client> {
                /// Required.
                ///
                /// ID of the model to use. You can use the [`crate::client::Client::list_models`] or
                /// [`crate::client::Client::list_models_blocking`] to see all of your available models,
                /// or see the [Model overview](https://platform.openai.com/docs/models/overview) for
                /// descriptions of them.
                model: &'model Model<'client>,

                #(#fields_tokens),*
            }

            impl<'model, 'client> #actual_substruct_name<'model, 'client> {
                #init_func

                #(#with_functions)*
            }

            impl<'model, 'client> crate::request::Request<'model, 'client, #response> for #actual_substruct_name<'model, 'client> {
                const URL: &'static str = #url;

                const COMPATIBLE_MODELS: &'static [&'static str] = &[
                    #(#compatible_models),*
                ];

                fn model(&self) -> &'model Model<'client> {
                    &self.model
                }

                fn model_error() -> crate::error::ModelError {
                    crate::error::ModelError::#model_error
                }

                #to_json
            }
        }
    });

    Ok((quote::quote! {
        #(#substructs)*
    })
    .into())
}

fn fix_req_option(field: &mut Field) -> miette::Result<()> {
    let ty_str = field.ty.to_token_stream().to_string().replace(' ', "");

    if ty_str.starts_with("Option<") {
        let ty = TokenStream2::from_str(&ty_str[7..ty_str.len() - 1]).map_err(|e| {
            miette::miette!("Failed to turn stripped type into a TokenStream: {}", e)
        })?;

        field.ty = syn::parse2(ty).into_diagnostic()?;
    }

    Ok(())
}
