mod accounts;
mod token;
mod transactions;
mod contract;

use crate::query_handler::QueryBuilder;
use reqwest::Client;

pub struct BscChainApi<'a> {
    api_key: &'a str,
    query: QueryBuilder,
    client: Client,
}

impl<'a> BscChainApi<'a> {
    pub fn new(api_key: &str) -> BscChainApi {
        BscChainApi {
            api_key,
            query: QueryBuilder::new("api.bscscan.com", "api"),
            client: Client::builder().https_only(true).build().unwrap(),
        }
    }
}
