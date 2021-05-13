use crate::{error::CustomErrors, wei_convertor::wei_convertor};

use super::{accounts::Response, BscChainApi};
use serde::Deserialize;
// #[derive(Debug, Deserialize)]
// pub struct Response<Vec<TokenTx>> {
//     status: String,
//     message: String,
//     result: Vec<TokenTx>,
// }

#[derive(Debug, Deserialize)]
pub struct TokenTx {
    #[serde(rename = "blockNumber")]
    block_no: String,
    #[serde(rename = "timeStamp")]
    timestamp: String,
    hash: String,
    nonce: String,
    #[serde(rename = "blockHash")]
    block_hash: String,
    from: String,
    to: String,
    #[serde(default)]
    value: String,
    #[serde(rename = "contractAddress")]
    contract_address: String,
    #[serde(default)]
    #[serde(rename = "tokenID")]
    token_id: String,
    #[serde(rename = "tokenName")]
    token_name: String,
    #[serde(rename = "tokenSymbol")]
    token_symbol: String,
    #[serde(rename = "tokenDecimal")]
    token_decimal: String,
    gas: String,
    #[serde(rename = "gasPrice")]
    gas_price: String,
    #[serde(default)]
    #[serde(rename = "transactionIndex")]
    transaction_index: String,
    #[serde(rename = "cumulativeGasUsed")]
    cumulative_gas_used: String,
    #[serde(rename = "gasUsed")]
    gas_used: String,
    confirmations: String,
}

impl TokenTx {
    pub fn block_no(&self) -> u32 {
        self.block_no.parse::<u32>().unwrap_or(0)
    }

    pub fn timestamp(&self) -> i32 {
        self.timestamp.parse::<i32>().unwrap_or(0)
    }

    pub fn hash(&self) -> String {
        self.hash.clone()
    }

    pub fn nonce(&self) -> u16 {
        self.nonce.parse::<u16>().unwrap_or(0)
    }

    pub fn block_hash(&self) -> String {
        self.block_hash.clone()
    }

    pub fn from(&self) -> String {
        self.from.clone()
    }

    pub fn to(&self) -> String {
        self.to.clone()
    }

    pub fn value(&self) -> f64 {
        if self.value.is_empty() {
            wei_convertor(&self.value)
        } else {
            0.0
        }
    }

    pub fn contract_address(&self) -> String {
        self.contract_address.clone()
    }

    pub fn token_id(&self) -> String {
        self.token_id.clone()
    }

    pub fn token_symbol(&self) -> String {
        self.token_symbol.clone()
    }

    pub fn token_decimal(&self) -> u16 {
        self.token_decimal.parse::<u16>().unwrap_or(0)
    }

    pub fn token_name(&self) -> String {
        self.token_name.clone()
    }

    pub fn gas(&self) -> u32 {
        self.gas.parse::<u32>().unwrap_or(0)
    }

    pub fn gas_price(&self) -> f64 {
        wei_convertor(&self.gas_price)
    }

    pub fn gas_used(&self) -> f64 {
        wei_convertor(&self.gas_used)
    }

    pub fn transaction_index(&self) -> u16 {
        if self.transaction_index.is_empty() {
            self.transaction_index.parse::<u16>().unwrap()
        } else {
            0
        }
    }

    pub fn cumulative_gas_used(&self) -> f64 {
        wei_convertor(&self.cumulative_gas_used)
    }

    pub fn confirmations(&self) -> u32 {
        self.confirmations.parse::<u32>().unwrap_or(0)
    }
}

impl Response<Vec<TokenTx>> {
    pub async fn parse_str(response: String) -> Response<Vec<TokenTx>> {
        serde_json::from_str::<Response<Vec<TokenTx>>>(&response).unwrap()
    }
}

impl<'a> BscChainApi<'a> {
    pub async fn bep_20_transactions(
        &mut self,
        address: &str,
        sort: bool,
    ) -> Result<Response<Vec<TokenTx>>, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "tokentx");
        self.query.add_params("address", address);
        self.query.add_params("startblock", "0");
        self.query.add_params("endblock", "latest");
        self.query
            .add_params("sort", if sort { "asc" } else { "dsc" });

        Ok(Response::<Vec<TokenTx>>::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn bep_20_transactions_pagination(
        &mut self,
        address: &str,
        sort: bool,
        page: u16,
        offest: u32,
    ) -> Result<Response<Vec<TokenTx>>, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "tokentx");
        self.query.add_params("address", address);
        self.query.add_params("page", &page.to_string());
        self.query.add_params("offset", &offest.to_string());
        self.query
            .add_params("sort", if sort { "asc" } else { "dsc" });

        Ok(Response::<Vec<TokenTx>>::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn bep_20_token_transaction_pagination(
        &mut self,
        address: &str,
        contract_address: &str,
        sort: bool,
        page: u16,
        offest: u32,
    ) -> Result<Response<Vec<TokenTx>>, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "tokentx");
        self.query.add_params("address", address);
        self.query.add_params("contractaddress", contract_address);

        self.query.add_params("page", &page.to_string());
        self.query.add_params("offset", &offest.to_string());
        self.query
            .add_params("sort", if sort { "asc" } else { "dsc" });

        Ok(Response::<Vec<TokenTx>>::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn erc_721_transactions_pagination(
        &mut self,
        address: &str,
        sort: bool,
        page: u16,
        offest: u32,
    ) -> Result<Response<Vec<TokenTx>>, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "tokennfttx");
        self.query.add_params("address", address);

        self.query.add_params("page", &page.to_string());
        self.query.add_params("offset", &offest.to_string());
        self.query
            .add_params("sort", if sort { "asc" } else { "dsc" });

        Ok(Response::<Vec<TokenTx>>::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn erc_721_transactions(
        &mut self,
        address: &str,
        sort: bool,
    ) -> Result<Response<Vec<TokenTx>>, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "tokennfttx");
        self.query.add_params("address", address);

        self.query.add_params("startblock", "0");
        self.query.add_params("endblock", "latest");
        self.query
            .add_params("sort", if sort { "asc" } else { "dsc" });

        Ok(Response::<Vec<TokenTx>>::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn erc_721_token_transactions_pagination(
        &mut self,
        address: &str,
        contract_address: &str,
        sort: bool,
        page: u16,
        offest: u32,
    ) -> Result<Response<Vec<TokenTx>>, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "tokennfttx");
        self.query.add_params("address", address);
        self.query.add_params("contractaddress", contract_address);

        self.query.add_params("page", &page.to_string());
        self.query.add_params("offset", &offest.to_string());
        self.query
            .add_params("sort", if sort { "asc" } else { "dsc" });

        Ok(Response::<Vec<TokenTx>>::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     fn create_success<'a>() -> BscChainApi<'a> {
//         BscChainApi::new("91FMGW3IAKTNBCQ4FH1IITRG92411S8DRZ")
//     }

//     #[actix_rt::test]
//     async fn bep_20_transactions() {
//         let mut api = create_success();
//         let m = api
//             .bep_20_transactions("0x7bb89460599dbf32ee3aa50798bbceae2a5f7f6a", true)
//             .await
//             .unwrap();
//         assert_eq!(m.result()[0].token_symbol(), "BUNNY")
//     }

//     #[actix_rt::test]
//     async fn bep20_transactions_pag() {
//         let mut api = create_success();
//         let m = api
//             .bep_20_transactions_pagination(
//                 "0x7bb89460599dbf32ee3aa50798bbceae2a5f7f6a",
//                 true,
//                 1,
//                 10,
//             )
//             .await
//             .unwrap();
//         assert_eq!(m.result()[0].token_symbol(), "BUNNY")
//     }

//     #[actix_rt::test]
//     async fn bep20_transactions_token() {
//         let mut api = create_success();
//         let m = api
//             .bep_20_token_transaction_pagination(
//                 "0x7bb89460599dbf32ee3aa50798bbceae2a5f7f6a",
//                 "0xc9849e6fdb743d08faee3e34dd2d1bc69ea11a51",
//                 true,
//                 1,
//                 10,
//             )
//             .await
//             .unwrap();
//         assert_eq!(m.result()[0].transaction_index(), 0);
//     }

//     #[actix_rt::test]
//     async fn erc_721_transactions_pagination() {
//         let mut api = create_success();
//         let m = api
//             .erc_721_transactions_pagination(
//                 "0xcd4ee0a77e09afa8d5a6518f7cf8539bef684e6c",
//                 true,
//                 1,
//                 10,
//             )
//             .await
//             .unwrap();
//         assert_eq!(m.result()[0].token_symbol(), "PLT")
//     }

//     #[actix_rt::test]
//     async fn erc_721_specific_token() {
//         let mut api = create_success();
//         let m = api
//             .erc_721_token_transactions_pagination(
//                 "0xcd4ee0a77e09afa8d5a6518f7cf8539bef684e6c",
//                 "0x5e74094cd416f55179dbd0e45b1a8ed030e396a1",
//                 true,
//                 1,
//                 10,
//             )
//             .await
//             .unwrap();
//         assert_eq!(m.result()[0].token_symbol(), "PLT")
//     }
// }
