use super::{accounts::NormalResponse, BscChainApi};
use crate::error::CustomErrors;
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
    pub async fn sourcecode(&self) -> String {
        self.sourcecode.clone()
    }

    pub async fn contract_name(&self) -> String {
        self.contract_name.clone()
    }

    pub async fn compile_version(&self) -> String {
        self.compile_version.clone()
    }

    pub async fn optimization_used(&self) -> String {
        self.optimization_used.clone()
    }

    pub async fn runs(&self) -> String {
        self.runs.clone()
    }

    pub async fn constructor_arguments(&self) -> String {
        self.constructor_arguments.clone()
    }

    pub async fn evm_version(&self) -> String {
        self.evm_version.clone()
    }

    pub async fn library(&self) -> String {
        self.library.clone()
    }

    pub async fn license_type(&self) -> String {
        self.license_type.clone()
    }

    pub async fn proxy(&self) -> String {
        self.proxy.clone()
    }

    pub async fn implementation(&self) -> String {
        self.implementation.clone()
    }

    pub async fn swarm_source(&self) -> String {
        self.swarm_source.clone()
    }
}

#[derive(Debug, Deserialize)]
pub struct SourceCode {
    status: String,
    message: String,
    result: Vec<Code>,
}

impl SourceCode {
    pub async fn parse_str(response: String) -> SourceCode {
        serde_json::from_str::<SourceCode>(&response).unwrap()
    }

    pub async fn status(&self) -> String {
        self.status.clone()
    }

    pub async fn message(&self) -> String {
        self.message.clone()
    }
}

impl<'a> BscChainApi<'a> {
    pub async fn get_abi(
        &mut self,
        smart_contarct_address: &str,
    ) -> Result<NormalResponse, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "contract");
        self.query.add_params("action", "getabi");
        self.query.add_params("address", smart_contarct_address);

        Ok(NormalResponse::parse_str(
            self.client
                .get(self.query.build_url())
                .send()
                .await?
                .text()
                .await?,
        )
        .await)
    }

    pub async fn get_source_code(
        &mut self,
        smart_contarct_address: &str,
    ) -> Result<SourceCode, CustomErrors> {
        self.query.add_params("apikey", self.api_key);
        self.query.add_params("module", "contract");
        self.query.add_params("action", "getsourcecode");
        self.query.add_params("address", smart_contarct_address);

        Ok(SourceCode::parse_str(
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
//     // use actix_rt;

//     fn create_success<'a>() -> BscChainApi<'a> {
//         BscChainApi::new("YOUR API KEY HERE")
//     }

    // #[actix_rt::test]
    // async fn abi_get() {
    //     let mut m = create_success();
    //     assert_eq!(
    //         m.get_abi("0xBCfCcbde45cE874adCB698cC183deBcF17952812")
    //             .await
    //             .unwrap()
    //             .status()
    //             .await,
    //         "1".to_string()
    //     );
    // }

    // #[actix_rt::test]
    // async fn get_source_code() {
    //     let mut m = create_success();
    //     assert_eq!(
    //         m.get_source_code("0xBCfCcbde45cE874adCB698cC183deBcF17952812")
    //             .await
    //             .unwrap()
    //             .result[0]
    //             .contract_name()
    //             .await,
    //         "PancakeFactory".to_string()
    //     );
    // }
// }
