# Using Tools

Tools extend agent capabilities by providing access to external functionality.

## Available Tools

Kowalski provides several built-in tool categories:

### Web Tools

- **WebSearchTool**: Search the web for information
- **WebScrapeTool**: Extract content from web pages

### Code Tools

- **CodeAnalysisTool**: Analyze code for Java, Python, and Rust
- Support for metrics, style checks, and suggestions

### Data Tools

- **CSVTool**: Process and analyze CSV files
- Statistical analysis and data insights

### Document Tools

- **PDFTool**: Extract text and analyze PDF documents
- Academic paper processing

## Tool Interface

All tools implement the `Tool` trait:

```rust
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> Vec<ToolParameter>;
    async fn execute(&mut self, input: ToolInput) -> Result<ToolOutput, KowalskiError>;
}
```

## Using Tools in Agents

### Registering Tools

```rust
use kowalski_core::template::default::DefaultTemplate;
use kowalski_tools::web::WebSearchTool;

// Create agent with tools using DefaultTemplate
let tools = vec![Box::new(WebSearchTool::new(config.clone()))];
let builder = DefaultTemplate::create_agent(tools, None, None).await?;
let mut agent = builder.build().await?;
```

### Tool Invocation

Tools are invoked automatically by the LLM when needed:

```rust
// User asks a question requiring web search
agent.add_message(&conv_id, "user", "What's the latest news on Rust 2024?").await;

// Agent will automatically:
// 1. Recognize it needs to search
// 2. Invoke WebSearchTool
// 3. Process results
// 4. Generate response with findings
let response = agent.generate_response(&conv_id, "").await?;
```

### Direct Tool Execution

You can also execute tools directly:

```rust
use kowalski_tools::web::WebSearchTool;
use kowalski_core::tools::{ToolInput, ToolOutput};
use serde_json::json;

let mut tool = WebSearchTool::new(config);
let input = ToolInput::new(
    "web_search".to_string(),
    "Rust async programming".to_string(),
    json!({
        "query": "Rust async programming"
    })
);

let output = tool.execute(input).await?;
println!("Search results: {:?}", output.result);
```

## Creating Custom Tools

Implement the `Tool` trait to create custom tools:

```rust
use kowalski_core::tools::{Tool, ToolParameter, ToolInput, ToolOutput, ParameterType};
use kowalski_core::error::KowalskiError;
use async_trait::async_trait;
use serde_json::json;

pub struct CalculatorTool;

#[async_trait]
impl Tool for CalculatorTool {
    fn name(&self) -> &str {
        "calculator"
    }
    
    fn description(&self) -> &str {
        "Performs basic arithmetic operations"
    }
    
    fn parameters(&self) -> Vec<ToolParameter> {
        vec![
            ToolParameter {
                name: "operation".to_string(),
                description: "The operation to perform (add, subtract, multiply, divide)".to_string(),
                required: true,
                default_value: None,
                parameter_type: ParameterType::String,
            },
            ToolParameter {
                name: "a".to_string(),
                description: "First operand".to_string(),
                required: true,
                default_value: None,
                parameter_type: ParameterType::Number,
            },
            ToolParameter {
                name: "b".to_string(),
                description: "Second operand".to_string(),
                required: true,
                default_value: None,
                parameter_type: ParameterType::Number,
            },
        ]
    }
    
    async fn execute(&mut self, input: ToolInput) -> Result<ToolOutput, KowalskiError> {
        let params = input.parameters;
        let operation = params["operation"].as_str()
            .ok_or_else(|| KowalskiError::ToolError("Missing operation".to_string()))?;
        
        let a = params["a"].as_f64()
            .ok_or_else(|| KowalskiError::ToolError("Invalid number a".to_string()))?;
        
        let b = params["b"].as_f64()
            .ok_or_else(|| KowalskiError::ToolError("Invalid number b".to_string()))?;
        
        let result = match operation {
            "add" => a + b,
            "subtract" => a - b,
            "multiply" => a * b,
            "divide" => {
                if b == 0.0 {
                    return Err(KowalskiError::ToolError("Division by zero".to_string()));
                }
                a / b
            }
            _ => return Err(KowalskiError::ToolError("Unknown operation".to_string())),
        };
        
        Ok(ToolOutput::new(json!({"result": result}), None))
    }
}
```

### Using the Custom Tool

```rust
let mut agent = TemplateAgent::new(config, template).await?;
agent.register_tool(Box::new(CalculatorTool));

let conv_id = agent.start_conversation("llama3.2");
agent.add_message(&conv_id, "user", "What is 15 + 27?").await;
let response = agent.generate_response(&conv_id, "").await?;
```

## Tool Design Best Practices

1. **Clear Descriptions**: Provide detailed descriptions so the LLM knows when to use the tool
2. **Parameter Validation**: Validate all parameters before execution
3. **Error Handling**: Return meaningful errors that can help debug issues
4. **Idempotency**: Tools should be safe to call multiple times with the same inputs
5. **Performance**: Keep tool execution fast; use async for I/O operations
6. **Schema Definition**: Use JSON Schema for clear parameter definitions

## Tool Categories

### Information Retrieval
- Web search
- Database queries
- API calls

### Content Processing
- Text analysis
- Image processing
- Data transformation

### Code Operations
- Code analysis
- Refactoring
- Testing

### System Operations
- File operations
- Process management
- System monitoring

## Next Steps

- Explore [Configuration](./configuration.md) for tool settings
- Read [Tooling System](./article_tooling.md) for architecture details
- Check out [Use Cases](./use_cases.md) for real-world examples
