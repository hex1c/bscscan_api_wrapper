use crate::{bsc_wrapper::BscChainApi, error::CustomErrors, wei_convertor::wei_convertor};
use serde::Deserialize;
use serde_json;

use super::accounts::Response;

// #[derive(Debug, Deserialize)]
// pub struct Transactions {
//     status: String,
//     message: String,
//     result: Vec<Txs>,
// }

impl Response<Vec<Txs>> {
    pub async fn parse_str(response: String) -> Response<Vec<Txs>> {
        serde_json::from_str::<Response<Vec<Txs>>>(&response).unwrap()
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
    pub fn block_no(&self) -> u32 {
        self.block_no.parse::<u32>().unwrap_or(0)
    }

    pub fn timestamp(&self) -> i32 {
        self.timestamp.parse::<i32>().unwrap_or(0)
    }

    pub fn hash(&self) -> Option<String> {
        if self.hash.is_empty() {
            None
        } else {
            Some(self.hash.clone())
        }
    }

    pub fn nonce(&self) -> Option<u16> {
        if self.nonce.is_empty() {
            None
        } else {
            Some(self.nonce.parse::<u16>().unwrap_or(0))
        }
    }

    pub fn block_hash(&self) -> Option<String> {
        if self.block_hash.is_empty() {
            None
        } else {
            Some(self.block_hash.clone())
        }
    }

    pub fn transaction_index(&self) -> Option<u16> {
        if self.transaction_index.is_empty() {
            None
        } else {
            Some(self.transaction_index.parse::<u16>().unwrap_or(0))
        }
    }

    pub fn from(&self) -> String {
        self.from.clone()
    }

    pub fn to(&self) -> String {
        self.to.clone()
    }

    pub fn value(&self) -> Option<f64> {
        if self.value.is_empty() {
            Some(wei_convertor(&self.value))
        } else {
            None
        }
    }

    pub fn gas(&self) -> u32 {
        self.gas.parse::<u32>().unwrap_or(0)
    }

    pub fn gas_price(&self) -> Option<f64> {
        if self.gas_price.is_empty() {
            None
        } else {
            Some(wei_convertor(&self.gas_price))
        }
    }

    pub fn is_error(&self) -> String {
        self.is_error.clone()
    }

    pub fn txreceipt_status(&self) -> Option<String> {
        if self.txreceipt_status.is_empty() {
            None
        } else {
            Some(self.txreceipt_status.clone())
        }
    }

    pub fn input(&self) -> String {
        self.input.clone()
    }

    pub fn contract_address(&self) -> String {
        self.contract_address.clone()
    }

    pub fn gas_used(&self) -> f64 {
        wei_convertor(&self.gas_used)
    }

    pub fn cumulative_gas_used(&self) -> Option<f64> {
        if self.cumulative_gas_used.is_empty() {
            None
        } else {
            Some(wei_convertor(&self.cumulative_gas_used))
        }
    }

    pub fn confirmations(&self) -> Option<u32> {
        if self.confirmations.is_empty() {
            None
        } else {
            Some(self.confirmations.parse::<u32>().unwrap_or(0))
        }
    }

    pub fn trace_id(&self) -> Option<String> {
        if self.trace_id.is_empty() {
            None
        } else {
            Some(self.trace_id.clone())
        }
    }

    pub fn err_code(&self) -> Option<String> {
        if self.err_code.is_empty() {
            None
        } else {
            Some(self.err_code.clone())
        }
    }

    pub fn _type(&self) -> Option<String> {
        if self._type.is_empty() {
            None
        } else {
            Some(self._type.clone())
        }
    }
}

#[derive(Debug, Deserialize)]
struct Status {
    status: String,
}

impl Response<Status> {
    pub async fn parse_str(response: String) -> Response<Status> {
        serde_json::from_str::<Response<Status>>(&response).unwrap()
    }

    pub fn transaction_status(&self) -> bool {
        self.result().status == "1"
    }
}

impl<'a> BscChainApi<'a> {
    pub async fn transactions(
        &mut self,
        address: &str,
        start_block: u32,
        end_block: u32,
        asc_sort: bool,
    ) -> Result<Response<Vec<Txs>>, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "txlist");
        self.query.add_params("address", address);
        self.query
            .add_params("startblock", &start_block.to_string());
        self.query.add_params("endblock", &end_block.to_string());
        self.query
            .add_params("sort", if asc_sort { "asc" } else { "dsc" });

        Ok(Response::<Vec<Txs>>::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn transactions_pagination(
        &mut self,
        address: &str,
        asc_sort: bool,
        page: u16,
        offset: u32,
    ) -> Result<Response<Vec<Txs>>, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "txlist");
        self.query.add_params("address", address);
        self.query.add_params("page", &page.to_string());
        self.query.add_params("offset", &offset.to_string());
        self.query.add_params("startblock", "0");
        self.query.add_params("endblock", "latest");
        self.query
            .add_params("sort", if asc_sort { "asc" } else { "dsc" });

        Ok(Response::<Vec<Txs>>::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn internal_transactions(
        &mut self,
        address: &str,
        asc_sort: bool,
    ) -> Result<Response<Vec<Txs>>, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "txlistinternal");
        self.query.add_params("address", address);
        self.query.add_params("startblock", "0");
        self.query.add_params("endblock", "latest");
        self.query
            .add_params("sort", if asc_sort { "asc" } else { "dsc" });
        Ok(Response::<Vec<Txs>>::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn internal_transaction_pagination(
        &mut self,
        address: &str,
        asc_sort: bool,
        page: u16,
        offset: u32,
    ) -> Result<Response<Vec<Txs>>, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "txlistinternal");
        self.query.add_params("address", address);
        self.query.add_params("startblock", "0");
        self.query.add_params("endblock", "latest");
        self.query
            .add_params("sort", if asc_sort { "asc" } else { "dsc" });
        self.query.add_params("page", &page.to_string());
        self.query.add_params("offset", &offset.to_string());

        Ok(Response::<Vec<Txs>>::parse_str(
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
    ) -> Result<Response<Vec<Txs>>, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "txlistinternal");
        self.query.add_params("txhash", tx);

        Ok(Response::<Vec<Txs>>::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn general_internal_transaction(
        &mut self,
        start_block: u32,
        end_block: u32,
        asc_sort: bool,
        page: u16,
        offset: u32,
    ) -> Result<Response<Vec<Txs>>, CustomErrors> {
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

        Ok(Response::<Vec<Txs>>::parse_str(
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

        Ok(Response::<Status>::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await
        .transaction_status())
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use actix_rt;

//     fn create_success<'a>() -> BscChainApi<'a> {
//         BscChainApi::new("91FMGW3IAKTNBCQ4FH1IITRG92411S8DRZ")
//     }

//     #[actix_rt::test]
//     async fn transaction_pagination() {
//         let mut api = create_success();
//         let m = api
//             .transactions_pagination("0x0000000000000000000000000000000000001004", true, 1, 10)
//             .await;
//         assert_eq!(
//             m.as_ref().unwrap().result()[2].to(),
//             "0x0000000000000000000000000000000000001004".to_string()
//         );
//     }

//     #[actix_rt::test]
//     async fn internal_transactions() {
//         let mut api = create_success();
//         let m = api
//             .internal_transactions("0x0000000000000000000000000000000000001004", true)
//             .await;
//         assert_eq!(
//             m.as_ref().unwrap().result()[2].from(),
//             "0x0000000000000000000000000000000000001004".to_string()
//         );
//     }

//     #[actix_rt::test]
//     async fn internal_transaction_pagination() {
//         let mut api = create_success();
//         let m = api
//             .internal_transaction_pagination(
//                 "0x0000000000000000000000000000000000001004",
//                 true,
//                 1,
//                 10,
//             )
//             .await;
//         assert_eq!(
//             m.as_ref().unwrap().result()[2].from(),
//             "0x0000000000000000000000000000000000001004".to_string()
//         );
//     }

//     #[actix_rt::test]
//     async fn internal_transaction_general_pagination() {
//         let mut api = create_success();
//         let m = api
//             .general_internal_transaction(0, 9999999, true, 1, 10)
//             .await;
//         assert_eq!(
//             m.as_ref().unwrap().result()[2].from(),
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
//             m.as_ref().unwrap().result()[0].from(),
//             "0x0000000000000000000000000000000000001004".to_string()
//         );
//     }

//     #[actix_rt::test]
//     async fn transaction_status() {
//         let mut api = create_success();
//         let m = api
//             .check_tx_status("0xe9975702518c79caf81d5da65dea689dcac701fcdd063f848d4f03c85392fd00")
//             .await;
//         assert_eq!(m.unwrap(), true);
//     }
// }
