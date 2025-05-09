# UK Property Tax Calculator: MCP Server Implementation

This repository demonstrates a practical Model Context Protocol (MCP) server implementation using Rust and the `rmcp` SDK. Unlike simple "counter" or "echo" examples, this server provides real utility by calculating UK Stamp Duty Land Tax (SDLT) for property purchases.

## Overview

This project showcases how to build a functional MCP server that:
1. Takes numerical input (property value)
2. Performs multi-tiered tax calculations based on UK tax bands
3. Returns formatted results to clients

This implementation serves as an educational example for developers looking to build their first production-ready MCP server.

## What is MCP?

MCP is an open protocol designed for structured communication between AI agents and tools/services. It enables AI systems to call external tools while maintaining a standardized communication format.

## Prerequisites

- Rust toolchain (cargo, rustc)
- Basic understanding of Rust syntax
- Familiarity with async programming concepts

## Understanding the Code

### Core Components

#### 1. Calculator Service

```rust
#[derive(Clone)]
pub struct Calculator;

impl Calculator {
    fn new() -> Self {
        Self
    }
}
```

This defines our main service struct that will handle SDLT calculations. It doesn't need to store any state in this case.

#### 2. Input Structure

```rust
#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct IpValidateRequest {
    pub property_value: f64,
}
```

This structure defines the expected input format. The `schemars::JsonSchema` derive enables automatic JSON schema generation for MCP clients to understand the input requirements.

#### 3. Tool Implementation

```rust
#[tool(tool_box)]
impl Calculator {
    // Tool methods here
}
```

The `#[tool(tool_box)]` attribute marks this implementation block as containing tools that will be exposed via MCP. Each method decorated with `#[tool(...)]` becomes available as a callable function.

#### 4. SDLT Calculation Logic

```rust
#[tool(description = "Calculate UK SDLT - property tax")]
async fn calculate_sdlt(
    &self,
    #[tool(aggr)] IpValidateRequest { property_value }: IpValidateRequest,
) -> Result<CallToolResult, Error> {
    // Tax calculation logic
}
```

This is where the business logic lives. The method:
- Takes a property value input
- Calculates tax based on UK SDLT bands
- Returns formatted results
- Handles potential errors

#### 5. Server Handler Implementation

```rust
#[tool(tool_box)]
impl ServerHandler for Calculator {
    fn get_info(&self) -> ServerInfo {
        // Server information
    }
}
```

This implements the required `ServerHandler` trait that provides metadata about your server to clients.

#### 6. Main Function

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let service = Calculator::new()
        .serve(stdio())
        .await
        .inspect_err(|e| eprintln!("{e}"))?;
    service.waiting().await?;
    Ok(())
}
```

The entry point that:
1. Creates a new Calculator instance
2. Serves it over standard I/O using the MCP protocol
3. Waits for the service to complete

## Key Learning Points

### 1. Input Validation
The code validates the input property value and provides a helpful message if missing.

### 2. Real Business Logic
Unlike toy examples, this implements actual business logic for calculating UK property tax:
- Multiple tax bands with different rates
- Proper handling of edge cases
- Formatted output with currency symbols

### 3. Error Handling
The implementation includes proper error handling patterns using Rust's Result type.

### 4. Tool Documentation
The `#[tool(description = "...")]` attribute provides documentation for clients about what the tool does.

### 5. Protocol Configuration
The ServerHandler implementation configures the MCP protocol version, capabilities, and instructions.

## Running the Server

1. Build the project:
```bash
cargo build --release
```

2. Run the server:
```bash
cargo run --release
```

3. Connect with an MCP client to interact with the calculator.

## Common Gotchas

- Don't forget the `#[tool(tool_box)]` attribute on both your tool implementation and ServerHandler implementation, or your tools won't be exposed properly
- Make sure to handle all potential error cases in your business logic
- Remember that MCP uses async functions, so you'll need to understand Rust's async/await patterns

## Extending the Server

Here are some ways you could extend this project:
- Add more property-related calculators (mortgage payments, rental yield, etc.)
- Implement rate lookups from an external API for up-to-date tax information
- Add validation for edge cases like non-residential properties or first-time buyers

## Resources

- [MCP Specification](https://github.com/machine-chat/machine-chat-protocol)
- [rmcp Documentation](https://docs.rs/rmcp/latest/rmcp/)
- [UK SDLT Official Rates](https://www.gov.uk/stamp-duty-land-tax/residential-property-rates)

## License

[MIT](LICENSE)
