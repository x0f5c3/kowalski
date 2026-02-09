# Creating Agents

Learn how to create custom agents in Kowalski.

## Agent Basics

An agent in Kowalski is composed of:
1. **Configuration**: Model, tools, and settings
2. **Template**: System prompt and behavior
3. **Tools**: Capabilities the agent can use
4. **Memory**: Episodic and semantic memory for context

## Using TemplateAgent

The `TemplateAgent` is the recommended way to create agents:

```rust
use kowalski_core::template::default::DefaultTemplate;
use kowalski_core::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create agent with default template
    let builder = DefaultTemplate::create_agent(
        Vec::new(), // tools
        Some("You are a helpful assistant specialized in Rust programming.".to_string()),
        Some(0.7), // temperature
    ).await?;
    
    let mut agent = builder.build().await?;
    
    // Use the agent
    let conv_id = agent.start_conversation("llama3.2");
    agent.add_message(&conv_id, "user", "Explain ownership in Rust").await;
    let response = agent.generate_response(&conv_id, "").await?;
    
    println!("{}", response);
    Ok(())
}
```

## Adding Tools to Agents

Tools extend agent capabilities:

```rust
use kowalski_tools::web::{WebSearchTool, WebScrapeTool};
use kowalski_core::template::TemplateAgent;

async fn create_web_agent() -> Result<TemplateAgent<DefaultTemplate>, Box<dyn std::error::Error>> {
    let config = Config::default();
    let template = DefaultTemplate::new(
        "Web Agent",
        "You are a web research assistant."
    );
    
    let mut agent = TemplateAgent::new(config.clone(), template).await?;
    
    // Register tools
    agent.register_tool(Box::new(WebSearchTool::new(config.clone())));
    agent.register_tool(Box::new(WebScrapeTool::new(config.clone())));
    
    Ok(agent)
}
```

## Custom Templates

Create custom templates for specialized behavior:

```rust
use kowalski_core::template::Template;

pub struct ResearchTemplate {
    name: String,
    description: String,
}

impl ResearchTemplate {
    pub fn new() -> Self {
        Self {
            name: "Research Assistant".to_string(),
            description: "A specialized research agent".to_string(),
        }
    }
}

impl Template for ResearchTemplate {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn description(&self) -> &str {
        &self.description
    }
    
    fn system_prompt(&self) -> String {
        r#"You are an expert research assistant.
        
Your capabilities include:
- Academic literature search
- Citation extraction and analysis
- Summarization of research papers
- Identifying research gaps

Always provide citations and be thorough in your analysis."#.to_string()
    }
    
    fn additional_instructions(&self) -> Option<String> {
        Some("When analyzing papers, focus on methodology and results.".to_string())
    }
}
```

## Example: Complete Custom Agent

Here's a complete example of a custom code review agent:

```rust
use kowalski_core::{
    Agent,
    config::Config,
    template::{TemplateAgent, Template},
};
use kowalski_tools::code::CodeAnalysisTool;

pub struct CodeReviewTemplate;

impl Template for CodeReviewTemplate {
    fn name(&self) -> &str {
        "Code Reviewer"
    }
    
    fn description(&self) -> &str {
        "Expert code reviewer for Rust, Python, and Java"
    }
    
    fn system_prompt(&self) -> String {
        "You are an expert code reviewer. Analyze code for:
        - Correctness and bugs
        - Performance issues
        - Security vulnerabilities
        - Code style and best practices
        - Documentation quality
        
        Provide specific, actionable feedback.".to_string()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::default();
    let template = CodeReviewTemplate;
    
    let mut agent = TemplateAgent::new(config.clone(), template).await?;
    
    // Add code analysis tool
    agent.register_tool(Box::new(CodeAnalysisTool::new(config.clone())));
    
    // Use the agent
    let conv_id = agent.start_conversation("llama3.2");
    
    let code = r#"
    fn divide(a: i32, b: i32) -> i32 {
        a / b
    }
    "#;
    
    agent.add_message(
        &conv_id,
        "user",
        &format!("Review this Rust code:\n```rust\n{}\n```", code)
    ).await;
    
    let review = agent.generate_response(&conv_id, "").await?;
    println!("Code Review:\n{}", review);
    
    Ok(())
}
```

## Agent Best Practices

1. **Clear System Prompts**: Define the agent's role and capabilities clearly
2. **Tool Selection**: Only add tools relevant to the agent's purpose
3. **Memory Management**: Configure appropriate memory settings for your use case
4. **Error Handling**: Always handle potential errors from LLM calls and tool invocations
5. **Testing**: Test agents with diverse inputs to ensure robust behavior

## Next Steps

- Learn about [Using Tools](./tools_guide.md)
- Explore [Memory Architecture](./memory_architecture.md)
- Read the [Memory System Design](./article_memory.md) article
