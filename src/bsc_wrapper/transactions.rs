use crate::{bsc_wrapper::BscChainApi, error::CustomErrors, wei_convertor::wei_convertor};
use serde::Deserialize;
use serde_json;

#[derive(Debug, Deserialize)]
pub struct Transactions {
    status: String,
    message: String,
    result: Vec<Txs>,
}

#[derive(Debug, Deserialize)]
struct Status {
    status: String,
}

#[derive(Debug, Deserialize)]
pub struct TXStatus {
    status: String,
    message: String,
    result: Status,
}

impl TXStatus {
    pub async fn parse_str(response: String) -> TXStatus {
        serde_json::from_str::<TXStatus>(&response).unwrap()
    }

    pub async fn status(&self) -> String {
        self.status.clone()
    }

    pub async fn message(&self) -> String {
        self.message.clone()
    }

    pub async fn transaction_status(&self) -> bool {
        self.result.status == "1"
    }
}

#[derive(Debug, Deserialize)]
pub struct Txs {
    #[serde(rename = "blockNumber")]
    block_no: String,
    #[serde(rename = "timeStamp")]
    timestamp: String,
    #[serde(default)]
    hash: String,
    #[serde(default)]
    nonce: String,
    #[serde(default)]
    #[serde(rename = "blockHash")]
    block_hash: String,
    #[serde(default)]
    #[serde(rename = "transactionIndex")]
    transaction_index: String,
    from: String,
    to: String,
    value: String,
    gas: String,
    #[serde(default)]
    #[serde(rename = "gasPrice")]
    gas_price: String,
    #[serde(rename = "isError")]
    is_error: String,
    #[serde(default)]
    txreceipt_status: String,
    input: String,
    #[serde(rename = "contractAddress")]
    contract_address: String,
    #[serde(default)]
    #[serde(rename = "cumulativeGasUsed")]
    cumulative_gas_used: String,
    #[serde(rename = "gasUsed")]
    gas_used: String,
    #[serde(default)]
    confirmations: String,
    #[serde(default)]
    #[serde(rename = "traceId")]
    trace_id: String,
    #[serde(default)]
    #[serde(rename = "errCode")]
    err_code: String,
    #[serde(default)]
    #[serde(rename = "type")]
    _type: String,
}

impl Txs {
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
        if self.nonce.is_empty() {
            self.gas.parse::<u16>().unwrap_or(0)
        } else {
            0
        }
    }

    pub async fn block_hash(&self) -> String {
        self.block_hash.clone()
    }

    pub async fn transaction_index(&self) -> u16 {
        if self.transaction_index.is_empty() {
            self.transaction_index.parse::<u16>().unwrap()
        } else {
            0
        }
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

    pub async fn gas(&self) -> u32 {
        self.gas.parse::<u32>().unwrap_or(0)
    }

    pub async fn gas_price(&self) -> f64 {
        wei_convertor(&self.gas_price)
    }

    pub async fn is_error(&self) -> String {
        self.is_error.clone()
    }

    pub async fn txreceipt_status(&self) -> String {
        self.txreceipt_status.clone()
    }

    pub async fn input(&self) -> String {
        self.input.clone()
    }

    pub async fn contract_address(&self) -> String {
        self.contract_address.clone()
    }

    pub async fn gas_used(&self) -> f64 {
        wei_convertor(&self.gas_used)
    }

    pub async fn cumulative_gas_used(&self) -> f64 {
        wei_convertor(&self.cumulative_gas_used)
    }

    pub async fn confirmations(&self) -> u32 {
        self.confirmations.parse::<u32>().unwrap_or(0)
    }

    pub async fn trace_id(&self) -> String {
        self.trace_id.clone()
    }

    pub async fn err_code(&self) -> String {
        self.err_code.clone()
    }

    pub async fn _type(&self) -> String {
        self._type.clone()
    }
}

impl Transactions {
    pub async fn parse_str(response: String) -> Transactions {
        serde_json::from_str::<Transactions>(&response).unwrap()
    }

    pub async fn status(&self) -> String {
        self.status.clone()
    }

    pub async fn message(&self) -> String {
        self.message.clone()
    }
}

impl<'a> BscChainApi<'a> {
    pub async fn transactions(
        &mut self,
        address: &str,
        start_block: u32,
        end_block: u32,
        asc_sort: bool,
    ) -> Result<Transactions, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "txlist");
        self.query.add_params("address", address);
        self.query
            .add_params("startblock", &start_block.to_string());
        self.query.add_params("endblock", &end_block.to_string());
        self.query
            .add_params("sort", if asc_sort { "asc" } else { "dsc" });

        Ok(Transactions::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn transactions_max_results(
        &mut self,
        address: &str,
        start_block: u32,
        end_block: u32,
        asc_sort: bool,
        page: u16,
        offset: u32,
    ) -> Result<Transactions, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "txlist");
        self.query.add_params("address", address);
        self.query.add_params("page", &page.to_string());
        self.query.add_params("offset", &offset.to_string());
        self.query
            .add_params("startblock", &start_block.to_string());
        self.query.add_params("endblock", &end_block.to_string());
        self.query
            .add_params("sort", if asc_sort { "asc" } else { "dsc" });

        Ok(Transactions::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn internal_transaction(
        &mut self,
        address: &str,
        start_block: u32,
        end_block: u32,
        asc_sort: bool,
    ) -> Result<Transactions, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "txlistinternal");
        self.query.add_params("address", address);
        self.query
            .add_params("startblock", &start_block.to_string());
        self.query.add_params("endblock", &end_block.to_string());
        self.query
            .add_params("sort", if asc_sort { "asc" } else { "dsc" });
        Ok(Transactions::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn internal_transaction_max(
        &mut self,
        address: &str,
        start_block: u32,
        end_block: u32,
        asc_sort: bool,
        page: u16,
        offset: u32,
    ) -> Result<Transactions, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "txlistinternal");
        self.query.add_params("address", address);
        self.query
            .add_params("startblock", &start_block.to_string());
        self.query.add_params("endblock", &end_block.to_string());
        self.query
            .add_params("sort", if asc_sort { "asc" } else { "dsc" });
        self.query.add_params("page", &page.to_string());
        self.query.add_params("offset", &offset.to_string());

        Ok(Transactions::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn get_internal_transaction_by_tx(
        &mut self,
        tx: &str,
    ) -> Result<Transactions, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "txlistinternal");
        self.query.add_params("txhash", tx);

        Ok(Transactions::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn internal_transaction_general_pagination(
        &mut self,
        start_block: u32,
        end_block: u32,
        asc_sort: bool,
        page: u16,
        offset: u32,
    ) -> Result<Transactions, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "txlistinternal");
        self.query
            .add_params("startblock", &start_block.to_string());
        self.query.add_params("endblock", &end_block.to_string());
        self.query
            .add_params("sort", if asc_sort { "asc" } else { "dsc" });
        self.query.add_params("page", &page.to_string());
        self.query.add_params("offset", &offset.to_string());

        Ok(Transactions::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn check_tx_status(&mut self, txhash: &str) -> Result<bool, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "transaction");
        self.query.add_params("action", "gettxreceiptstatus");
        self.query.add_params("txhash", txhash);

        Ok(TXStatus::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await
        .transaction_status()
        .await)
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use actix_rt;

// fn create_success<'a>() -> BscChainApi<'a> {
//     BscChainApi::new("YOUR API KEY HERE")
// }

//     #[actix_rt::test]
//     async fn transaction_pagination() {
//         let mut api = create_success();
//         let m = api
//             .transactions_max_results(
//                 "0x0000000000000000000000000000000000001004",
//                 0,
//                 9999999,
//                 true,
//                 1,
//                 10,
//             )
//             .await;
//         assert_eq!(
//             m.as_ref().unwrap().result[2].to().await,
//             "0x0000000000000000000000000000000000001004".to_string()
//         );
//     }

//     #[actix_rt::test]
//     async fn internal_transaction() {
//         let mut api = create_success();
//         let m = api
//             .internal_transaction(
//                 "0x0000000000000000000000000000000000001004",
//                 0,
//                 9999999,
//                 true,
//             )
//             .await;
//         assert_eq!(
//             m.as_ref().unwrap().result[2].from().await,
//             "0x0000000000000000000000000000000000001004".to_string()
//         );
//     }

//     #[actix_rt::test]
//     async fn internal_transaction_general_pagination() {
//         let mut api = create_success();
//         let m = api
//             .internal_transaction_general_pagination(0, 9999999, true, 1, 10)
//             .await;
//         assert_eq!(
//             m.as_ref().unwrap().result[2].from().await,
//             "0x0000000000000000000000000000000000001004".to_string()
//         );
//     }

//     #[actix_rt::test]
//     async fn internal_transaction_tx() {
//         let mut api = create_success();
//         let m = api
//             .get_internal_transaction_by_tx(
//                 "0x4d74a6fc84d57f18b8e1dfa07ee517c4feb296d16a8353ee41adc03669982028",
//             )
//             .await;
//         assert_eq!(
//             m.as_ref().unwrap().result[0].from().await,
//             "0x0000000000000000000000000000000000001004".to_string()
//         );
//     }

// #[actix_rt::test]
// async fn internal_transaction_tx() {
//     let mut api = create_success();
//     let m = api
//         .check_tx_status("0xe9975702518c79caf81d5da65dea689dcac701fcdd063f848d4f03c85392fd00")
//         .await;
//     assert_eq!(m.unwrap(), true);
// }

// }
