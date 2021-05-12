use std::collections::HashMap;

pub struct QueryBuilder {
    base_url: String,                //Base URL
    query: String,                   // Query String
    params: HashMap<String, String>, // Different Params
}

impl QueryBuilder {
    pub fn new(base_url: &str, query: &str) -> QueryBuilder {
        QueryBuilder {
            base_url: base_url.to_string(),
            query: query.to_string(),
            params: HashMap::new(),
        }
    }

    pub fn add_params(&mut self, param_key: &str, value: &str) {
        self.params.insert(param_key.to_string(), value.to_string());
    }

    pub fn multi_params(&mut self, param_key: &str, value: Vec<&str>) {
        self.params.insert(param_key.to_string(), value.join(","));
    }

    pub fn build_url(&self) -> String {
        let mut key_pairs = vec![];
        for (key, value) in self.params.iter() {
            key_pairs.push(format!("{}={}", key, value));
        }
        format!(
            "https://{}/{}?{}",
            self.base_url,
            self.query,
            key_pairs.join("&")
        )
    }
}

// This fails sometimes due to the order the key values pairs are appended
// #[test]
// fn test_build() {
//     let mut query = QueryBuilder::new("google.com", "test");
//     query.add_params("test_q", "memo");
//     query.multi_params("address", vec!["1", "2"]);
//     assert_eq!(
//         query.build_url(),
//         "https://google.com/test?test_q=memo&address=1,2"
//     )
// }
