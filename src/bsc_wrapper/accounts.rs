use serde::Deserialize;
use serde_json;

use super::BscChainApi;
use crate::{error::CustomErrors, wei_convertor::wei_convertor};

#[derive(Debug, Deserialize)]
pub struct Balance {
    account: String,
    balance: String,
}

impl Balance {
    pub fn get_balace(&self) -> f64 {
        wei_convertor(&self.balance)
    }

    pub fn account(&self) -> String {
        self.account.clone()
    }
}


#[derive(Debug, Deserialize)]
pub struct Response<T> {
    status: String,
    message: String,
    result: T,
}

impl<T> Response<T> {
    pub fn new(status: String, message: String, result: T) -> Response<T> {
        Response {
            status,
            message,
            result,
        }
    }

    pub fn status(&self) -> String {
        self.status.clone()
    }

    pub fn message(&self) -> String {
        self.message.clone()
    }

    pub fn result(&self) -> &T {
        &self.result
    }
}

impl Response<String> {
    pub async fn parse_str(response: String) -> Response<String> {
        serde_json::from_str::<Response<String>>(&response).unwrap()
    }

    pub fn get_balace(&self) -> f64 {
        wei_convertor(&self.result)
    }
}

impl Response<Vec<Balance>> {
    pub async fn parse_str(response: String) -> Response<Vec<Balance>> {
        serde_json::from_str::<Response<Vec<Balance>>>(&response).unwrap()
    }
}

impl<'a> BscChainApi<'a> {
    // This function gives the BNB balance of the account
    pub async fn balance(&mut self, address: &str) -> Result<Response<String>, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "balance");
        self.query.add_params("tag", "latest");
        self.query.add_params("address", address);

        Ok(Response::<String>::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    //It fetches the BNB balances of the provided addresses
    pub async fn multi_balance(
        &mut self,
        accounts: Vec<&str>,
    ) -> Result<Response<Vec<Balance>>, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "balancemulti");
        self.query.add_params("tag", "latest");
        self.query.multi_params("address", accounts);

        Ok(Response::<Vec<Balance>>::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn token_balance(
        &mut self,
        address: &str,
        token_contract_address: &str,
    ) -> Result<Response<String>, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "tokenbalance");
        self.query.add_params("tag", "latest");
        self.query.add_params("address", address);
        self.query
            .add_params("contractaddress", token_contract_address);

        Ok(Response::<String>::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn bnb_balace_by_block_no(
        &mut self,
        address: &str,
        block_no: u32,
    ) -> Result<Response<String>, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "account");
        self.query.add_params("action", "balancehistory");
        self.query.add_params("address", address);
        self.query.add_params("blockno", &block_no.to_string());

        Ok(Response::<String>::parse_str(
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
//     async fn test_balance() {
//         let mut api = create_success();
//         let m = api
//             .balance("0xdB13FCA9C10805c39683FF8b2642648C573d1437")
//             .await;
//         assert_eq!(m.unwrap().get_balace(), 0.19146994786881893)
//     }

//     // Tests may fail due to the randomness of the order of the accounts fetched
//     #[actix_rt::test]
//     async fn test_multi_balace() {
//         let mut api = create_success();
//         let accs = vec!["0x0000000000000000000000000000000000001004"];
//         let m = api.multi_balance(accs).await;
//         assert_eq!(
//             m.as_ref().unwrap().result[0].account(),
//             "0x0000000000000000000000000000000000001004".to_string()
//         );
//     }

//     // For Testing Purpose twt token is used
//     #[actix_rt::test]
//     async fn test_token_balance() {
//         let mut api = create_success();
//         let m = api
//             .token_balance(
//                 "0x0000000000000000000000000000000000001004",
//                 "0x4b0f1812e5df2a09796481ff14017e6005508003",
//             )
//             .await;
//         assert_eq!(m.as_ref().unwrap().get_balace(), 816484077.8674881);
//     }

// // Could not test as it is a pro features
// #[actix_rt::test]
// async fn bnb_by_block() {
//     let mut api = create_success();
//     let m = api
//         .bnb_balace_by_block_no("0x0000000000000000000000000000000000001004", 99999)
//         .await;
//     assert_eq!(m.as_ref().unwrap().result, "0".to_string());
// }
// }
