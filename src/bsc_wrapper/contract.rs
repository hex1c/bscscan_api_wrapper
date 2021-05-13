use super::{accounts::Response, BscChainApi};
use crate::error::{CustomErrors, ErrorCause};
use ethabi::Contract;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Code {
    #[serde(rename = "SourceCode")]
    sourcecode: String,
    #[serde(rename = "ContractName")]
    contract_name: String,
    #[serde(rename = "CompilerVersion")]
    compile_version: String,
    #[serde(rename = "OptimizationUsed")]
    optimization_used: String,
    #[serde(rename = "Runs")]
    runs: String,
    #[serde(rename = "ConstructorArguments")]
    constructor_arguments: String,
    #[serde(rename = "EVMVersion")]
    evm_version: String,
    #[serde(rename = "Library")]
    library: String,
    #[serde(rename = "LicenseType")]
    license_type: String,
    #[serde(rename = "Proxy")]
    proxy: String,
    #[serde(rename = "Implementation")]
    implementation: String,
    #[serde(rename = "SwarmSource")]
    swarm_source: String,
}

impl Code {
    pub fn sourcecode(&self) -> String {
        self.sourcecode.clone()
    }

    pub fn contract_name(&self) -> String {
        self.contract_name.clone()
    }

    pub fn compile_version(&self) -> String {
        self.compile_version.clone()
    }

    pub fn optimization_used(&self) -> String {
        self.optimization_used.clone()
    }

    pub fn runs(&self) -> String {
        self.runs.clone()
    }

    pub fn constructor_arguments(&self) -> String {
        self.constructor_arguments.clone()
    }

    pub fn evm_version(&self) -> String {
        self.evm_version.clone()
    }

    pub fn library(&self) -> String {
        self.library.clone()
    }

    pub fn license_type(&self) -> String {
        self.license_type.clone()
    }

    pub fn proxy(&self) -> String {
        self.proxy.clone()
    }

    pub fn implementation(&self) -> String {
        self.implementation.clone()
    }

    pub fn swarm_source(&self) -> String {
        self.swarm_source.clone()
    }
}

impl Response<Vec<Code>> {
    pub async fn parse_str(response: String) -> Response<Vec<Code>> {
        serde_json::from_str::<Response<Vec<Code>>>(&response).unwrap()
    }
}

impl Response<Contract> {
    pub async fn parse_str(response: String) -> Result<Response<Contract>, CustomErrors> {
        let m = serde_json::from_str::<Response<String>>(&response).unwrap();
        let r = Contract::load(m.result().as_bytes())
            .map_err(|_| CustomErrors::new(ErrorCause::AbiParsingError))?;

        Ok(Response::<Contract>::new(m.status(), m.message(), r))
    }
}

impl<'a> BscChainApi<'a> {
    pub async fn get_abi(
        &mut self,
        smart_contarct_address: &str,
    ) -> Result<Response<Contract>, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "contract");
        self.query.add_params("action", "getabi");
        self.query.add_params("address", smart_contarct_address);

        Ok(Response::<Contract>::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await?)
    }

    pub async fn get_source_code(
        &mut self,
        smart_contarct_address: &str,
    ) -> Result<Response<Vec<Code>>, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "contract");
        self.query.add_params("action", "getsourcecode");
        self.query.add_params("address", smart_contarct_address);

        Ok(Response::<Vec<Code>>::parse_str(
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
//     async fn abi_get() {
//         let mut m = create_success();
//         assert_eq!(
//             m.get_abi("0xBCfCcbde45cE874adCB698cC183deBcF17952812")
//                 .await
//                 .unwrap()
//                 .status(),
//             "1".to_string()
//         );
//     }

//     #[actix_rt::test]
//     async fn get_source_code() {
//         let mut m = create_success();
//         assert_eq!(
//             m.get_source_code("0xBCfCcbde45cE874adCB698cC183deBcF17952812")
//                 .await
//                 .unwrap()
//                 .result()[0]
//                 .contract_name(),
//             "PancakeFactory".to_string()
//         );
//     }
// }
