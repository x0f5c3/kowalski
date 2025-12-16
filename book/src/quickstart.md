# Quick Start

This guide will help you get started with Kowalski quickly.

## Using the CLI

### Basic Chat

Start an interactive chat session:

```bash
kowalski-cli
```

Once in the CLI, you can:
- Type messages to chat with the AI agent
- Use `/help` to see available commands
- Type `exit`, `quit`, or `bye` to exit

### Command-Line Tools

Use Kowalski's tools directly from the command line:

```bash
# Web search
kowalski tool search "rust async programming"

# Code analysis
kowalski tool code ./src/main.rs

# Analyze a PDF
kowalski academic --file research.pdf
```

## Using the Rust API

Here's a simple example of using Kowalski in your Rust code:

```rust
use kowalski_core::{Agent, BaseAgent, Config};
use kowalski_core::agent::Agent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let config = Config::default();
    
    // Create an agent
    let mut agent = BaseAgent::new(
        config.clone(),
        "Demo Agent",
        "A helpful assistant"
    ).await?;
    
    // Start a conversation
    let conv_id = agent.start_conversation("llama3.2");
    
    // Send a message
    agent.add_message(&conv_id, "user", "Hello, world!").await;
    
    // Get a response
    let response = agent.generate_response(&conv_id, "").await?;
    println!("Agent: {}", response);
    
    Ok(())
}
```

## Running Specific Agents

### Academic Agent

Analyze academic papers and PDFs:

```bash
cargo run --release --bin kowalski-academic-agent -- --file paper.pdf
```

**Features:**
- PDF analysis
- Citation extraction
- Academic summarization

### Code Agent

Analyze code files:

```bash
cargo run --release --bin kowalski-code-agent -- --file main.rs
```

**Features:**
- Code metrics
- Suggestions
- Multi-language support (Java, Python, Rust)

### Data Agent

Analyze CSV and data files:

```bash
cargo run --release

# Then in the CLI:
create data
chat data-agent
# Ask: "Could you investigate content and give me insight about domain_structure.csv file?"
```

**Features:**
- CSV analysis
- Statistics
- Summaries

### Web Agent

Perform web research:

```bash
cargo run --release --bin kowalski-web-agent -- --query "latest AI news"
```

**Features:**
- Web search
- Web scraping
- Summarization

## Next Steps

- Learn about [Architecture](./architecture.md) to understand how Kowalski works
- Explore [Memory Architecture](./memory_architecture.md) for advanced features
- Read [Creating Agents](./creating_agents.md) to build custom agents
- Check out [Use Cases](./use_cases.md) for real-world examples
