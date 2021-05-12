use crate::{error::CustomErrors, wei_convertor::wei_convertor};

use super::BscChainApi;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct TokenTransactions {
    pub status: String,
    pub message: String,
    pub result: Vec<TokenTx>,
}

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
    pub async fn block_no(&self) -> u32 {
        self.block_no.parse::<u32>().unwrap_or(0)
    }

    pub async fn timestamp(&self) -> i32 {
        self.timestamp.parse::<i32>().unwrap_or(0)
    }

    pub async fn hash(&self) -> String {
        self.hash.clone()
    }

    pub async fn nonce(&self) -> u16 {
        self.nonce.parse::<u16>().unwrap_or(0)
    }

    pub async fn block_hash(&self) -> String {
        self.block_hash.clone()
    }

    pub async fn from(&self) -> String {
        self.from.clone()
    }

    pub async fn to(&self) -> String {
        self.to.clone()
    }

    pub async fn value(&self) -> f64 {
        if self.value.is_empty() {
            wei_convertor(&self.value)
        } else {
            0.0
        }
    }

    pub async fn contract_address(&self) -> String {
        self.contract_address.clone()
    }

    pub async fn token_id(&self) -> String {
        self.token_id.clone()
    }

    pub async fn token_symbol(&self) -> String {
        self.token_symbol.clone()
    }

    pub async fn token_decimal(&self) -> u16 {
        self.token_decimal.parse::<u16>().unwrap_or(0)
    }

    pub async fn token_name(&self) -> String {
        self.token_name.clone()
    }

    pub async fn gas(&self) -> u32 {
        self.gas.parse::<u32>().unwrap_or(0)
    }

    pub async fn gas_price(&self) -> f64 {
        wei_convertor(&self.gas_price)
    }

    pub async fn gas_used(&self) -> f64 {
        wei_convertor(&self.gas_used)
    }

    pub async fn transaction_index(&self) -> u16 {
        if self.transaction_index.is_empty() {
            self.transaction_index.parse::<u16>().unwrap()
        } else {
            0
        }
    }

    pub async fn cumulative_gas_used(&self) -> f64 {
        wei_convertor(&self.cumulative_gas_used)
    }

    pub async fn confirmations(&self) -> u32 {
        self.confirmations.parse::<u32>().unwrap_or(0)
    }
}

impl TokenTransactions {
    pub async fn parse_str(response: String) -> TokenTransactions {
        serde_json::from_str::<TokenTransactions>(&response).unwrap()
    }

    pub async fn status(&self) -> String {
        self.status.clone()
    }

    pub async fn message(&self) -> String {
        self.message.clone()
    }
}

impl<'a> BscChainApi<'a> {
    pub async fn bep20_transactions(
        &mut self,
        address: &str,
        start_block: u32,
        end_block: u32,
        sort: bool,
    ) -> Result<TokenTransactions, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "tokentx");
        self.query.add_params("address", address);
        self.query
            .add_params("startblock", &start_block.to_string());
        self.query.add_params("endblock", &end_block.to_string());
        self.query
            .add_params("sort", if sort { "asc" } else { "dsc" });

        Ok(TokenTransactions::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn bep20_transactions_pagination(
        &mut self,
        address: &str,
        sort: bool,
        page: u16,
        offest: u32,
    ) -> Result<TokenTransactions, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "tokentx");
        self.query.add_params("address", address);
        self.query.add_params("page", &page.to_string());
        self.query.add_params("end", &offest.to_string());
        self.query
            .add_params("sort", if sort { "asc" } else { "dsc" });

        Ok(TokenTransactions::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn bep20_specific_token_transactions(
        &mut self,
        address: &str,
        contract_address: &str,
        sort: bool,
        page: u16,
        offest: u32,
    ) -> Result<TokenTransactions, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "tokentx");
        self.query.add_params("address", address);
        self.query.add_params("contractaddress", contract_address);

        self.query.add_params("page", &page.to_string());
        self.query.add_params("offset", &offest.to_string());
        self.query
            .add_params("sort", if sort { "asc" } else { "dsc" });

        Ok(TokenTransactions::parse_str(
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
    ) -> Result<TokenTransactions, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "tokennfttx");
        self.query.add_params("address", address);

        self.query.add_params("page", &page.to_string());
        self.query.add_params("offset", &offest.to_string());
        self.query
            .add_params("sort", if sort { "asc" } else { "dsc" });

        Ok(TokenTransactions::parse_str(
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
        start_block: u32,
        end_block: u32,
    ) -> Result<TokenTransactions, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "tokennfttx");
        self.query.add_params("address", address);

        self.query
            .add_params("startblock", &start_block.to_string());
        self.query.add_params("endblock", &end_block.to_string());
        self.query
            .add_params("sort", if sort { "asc" } else { "dsc" });

        Ok(TokenTransactions::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn erc_721_specific_token_transactions(
        &mut self,
        address: &str,
        contract_address: &str,
        sort: bool,
        page: u16,
        offest: u32,
    ) -> Result<TokenTransactions, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "tokennfttx");
        self.query.add_params("address", address);
        self.query.add_params("contractaddress", contract_address);

        self.query.add_params("page", &page.to_string());
        self.query.add_params("offset", &offest.to_string());
        self.query
            .add_params("sort", if sort { "asc" } else { "dsc" });

        Ok(TokenTransactions::parse_str(
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
//     use actix_rt;

//     fn create_success<'a>() -> BscChainApi<'a> {
//         BscChainApi::new("YOUR API KEY HERE")
//     }

//     #[actix_rt::test]
//     async fn bep20_transactions() {
//         let mut api = create_success();
//         let m = api
//             .bep20_transactions(
//                 "0x7bb89460599dbf32ee3aa50798bbceae2a5f7f6a",
//                 0,
//                 2500000,
//                 true,
//             )
//             .await
//             .unwrap();
//         assert_eq!(m.result[0].token_symbol().await, "BUNNY")
//     }

//     #[actix_rt::test]
//     async fn bep20_transactions_pag() {
//         let mut api = create_success();
//         let m = api
//             .bep20_transactions_pagination(
//                 "0x7bb89460599dbf32ee3aa50798bbceae2a5f7f6a",
//                 true,
//                 1,
//                 10,
//             )
//             .await
//             .unwrap();
//         assert_eq!(m.result[0].token_symbol().await, "BUNNY")
//     }

//     #[actix_rt::test]
//     async fn bep20_transactions_token() {
//         let mut api = create_success();
//         let m = api
//             .bep20_specific_token_transactions(
//                 "0x7bb89460599dbf32ee3aa50798bbceae2a5f7f6a",
//                 "0xc9849e6fdb743d08faee3e34dd2d1bc69ea11a51",
//                 true,
//                 1,
//                 10,
//             )
//             .await
//             .unwrap();
//         assert_eq!(m.result[0].transaction_index().await, 2);
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
//         assert_eq!(m.result[0].token_symbol().await, "PLT")
//     }

//     #[actix_rt::test]
//     async fn erc_721_specific_token() {
//         let mut api = create_success();
//         let m = api
//             .erc_721_specific_token_transactions(
//                 "0xcd4ee0a77e09afa8d5a6518f7cf8539bef684e6c",
//                 "0x5e74094cd416f55179dbd0e45b1a8ed030e396a1",
//                 true,
//                 1,
//                 10,
//             )
//             .await
//             .unwrap();
//         assert_eq!(m.result[0].token_symbol().await, "PLT")
//     }
// }
