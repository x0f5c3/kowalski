# Architecture Overview

Kowalski is built on a modular, extensible architecture designed for building sophisticated AI agent systems.

## Core Design Principles

1. **Modularity**: Each component is independent and can be used standalone
2. **Extensibility**: Easy to add new tools, agents, and capabilities
3. **Async-First**: Built on Tokio for high-performance async operations
4. **Type Safety**: Leverages Rust's type system for robust code
5. **Federation Ready**: Designed for multi-agent collaboration

## System Architecture

![Architecture Diagram](img/architecture_v01.png)

## Module Breakdown

### kowalski-core

The foundational layer providing:
- Agent abstractions and traits
- Conversation management
- Role-based access control
- Configuration management
- Error handling
- Memory system (episodic, semantic, procedural)
- Template system for agent creation

### kowalski-tools

Pluggable tools for various capabilities:
- **Code Tools**: Analysis for Java, Python, Rust
- **Data Tools**: CSV processing, statistics
- **Web Tools**: Search, scraping, content extraction
- **Document Tools**: PDF processing, text extraction
- **And more...**

Each tool implements a common trait interface, making them easily swappable and composable.

### kowalski-federation

**(Work in Progress)**

Multi-agent orchestration layer providing:
- Agent registry and discovery
- Task delegation and distribution
- Message passing protocols
- Coordination strategies

### Specialized Agents

Pre-built agents for specific domains:

- **kowalski-academic-agent**: Academic research and paper analysis
- **kowalski-code-agent**: Code analysis and suggestions
- **kowalski-data-agent**: Data processing and insights
- **kowalski-web-agent**: Web research and content gathering

Each agent is built using the core abstractions and can be customized or extended.

### kowalski-cli

Command-line interface providing:
- Interactive chat sessions
- Tool invocation
- Agent management
- Configuration

## Agent Lifecycle

```
┌─────────────┐
│   Create    │  Initialize agent with config and tools
│   Agent     │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Start     │  Begin conversation with specific model
│Conversation │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Message   │  Add user/assistant messages
│  Exchange   │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Generate   │  Call LLM and potentially use tools
│  Response   │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Memory    │  Store interactions for future recall
│   Storage   │
└─────────────┘
```

## Tool Integration

Tools are registered with agents and can be invoked during conversations:

1. Agent receives a user message
2. LLM determines if a tool is needed
3. Tool is invoked with parameters
4. Tool result is added to context
5. LLM generates final response

## Memory System

Kowalski implements a multi-tiered memory architecture:

- **Episodic Memory**: Conversation history and interactions
- **Semantic Memory**: Long-term knowledge and facts (via vector DB)
- **Procedural Memory**: Skills, tools, and how to use them

See [Memory Architecture](./memory_architecture.md) for details.

## Configuration

All components are configured via TOML files:

```toml
[ollama]
host = "localhost"
port = 11434
model = "llama3.2"

[qdrant]
url = "http://localhost:6333"
collection = "kowalski_memory"

[memory]
episodic_path = "./target/episodic_db"
```

## Next Steps

- Deep dive into [Key Technologies](./key_technology.md)
- Learn about [Memory Architecture](./memory_architecture.md)
- Start [Creating Agents](./creating_agents.md)
