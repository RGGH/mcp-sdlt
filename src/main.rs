use rmcp::{
    Error, ServerHandler, ServiceExt,
    model::{
        CallToolResult, Content, Implementation, ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    schemars, tool,
    transport::stdio,
};

#[derive(Clone)]
pub struct Calculator;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct IpValidateRequest {
    pub property_value: f64,
}

#[tool(tool_box)]
impl Calculator {
    fn new() -> Self {
        Self
    }

    // phase 1, just to get the idea...
    // #[tool(description = "Calculate UK SDLT - property tax")]
    // async fn calculate_sdlt(&self, #[tool(aggr)] IpValidateRequest { property_value }: IpValidateRequest )->Result<CallToolResult,Error>{
    //     let result = match Some(property_value) {
    //         Some(val) => val * 10,
    //         None => 1
    //     };
    //     Ok(CallToolResult::success(vec![Content::text(result.to_string())]))
    // }

    /// Tax Band	Normal Rate
    /// less than £125k	0%
    /// £125k to £250k	2%
    /// £250k to £925k	5%
    /// £925k to £1.5m	10%
    /// rest over £1.5m	12%

    #[tool(description = "Calculate UK SDLT - property tax")]
    async fn calculate_sdlt(
        &self,
        #[tool(aggr)] IpValidateRequest { property_value }: IpValidateRequest,
    ) -> Result<CallToolResult, Error> {
        let property_value = match Some(property_value) {
            Some(val) => val,
            None => {
                return Ok(CallToolResult::success(vec![Content::text(
                    "Property value is missing.".to_string(),
                )]));
            }
        };

        let mut tax = 0.0;

        if property_value > 1_500_000.0 {
            tax += (property_value - 1_500_000.0) * 0.12;
        }
        if property_value > 925_000.0 {
            let upper = property_value.min(1_500_000.0);
            tax += (upper - 925_000.0) * 0.10;
        }
        if property_value > 250_000.0 {
            let upper = property_value.min(925_000.0);
            tax += (upper - 250_000.0) * 0.05;
        }
        if property_value > 125_000.0 {
            let upper = property_value.min(250_000.0);
            tax += (upper - 125_000.0) * 0.02;
        }
        // No tax for the first £125,000

        Ok(CallToolResult::success(vec![Content::text(format!(
            "SDLT for £{property_value:.2} is £{tax:.2}"
        ))]))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = Calculator::new()
        .serve(stdio())
        .await
        .inspect_err(|e| eprintln!("{e}"))?;
    service.waiting().await?;
    Ok(())
}

#[tool(tool_box)] // I forgot this line initially, you see no tools if you don't add it!  i.e response = tools:[]
impl ServerHandler for Calculator {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "This server provides a calculator tool
                to work out UK SDLT"
                    .to_string(),
            ),
        }
    }
}


