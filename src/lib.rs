use reqwest::header::HeaderMap;

pub mod client;
pub mod completion;
pub mod error;
pub mod model;

trait APIKeysAccess {
    fn get_api_key(&self) -> &String;
    fn get_org_id(&self) -> &Option<String>;

    fn common_headers(&self) -> HeaderMap {
        let mut header_map = HeaderMap::new();

        self.auth_header(&mut header_map);
        self.org_header(&mut header_map);

        header_map
    }

    fn auth_header(&self, header_map: &mut HeaderMap) {
        header_map.insert(
            "Authorization",
            format!("Bearer {}", self.get_api_key()).parse().unwrap(),
        );
    }

    fn org_header(&self, header_map: &mut HeaderMap) {
        if let Some(org) = &self.get_org_id() {
            header_map.insert("OpenAI-Organization", org.parse().unwrap());
        }
    }
}
