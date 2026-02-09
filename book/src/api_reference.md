# API Reference

This page provides a reference to key APIs in Kowalski.

## Core Traits

### Agent Trait

The core `Agent` trait defines the interface for all agents:

```rust
#[async_trait]
pub trait Agent: Send + Sync {
    fn start_conversation(&mut self, model: &str) -> String;
    async fn add_message(&mut self, conversation_id: &str, role: &str, content: &str);
    async fn generate_response(&mut self, conversation_id: &str, user_input: &str) 
        -> Result<String, KowalskiError>;
    async fn chat_with_tools(&mut self, conversation_id: &str, input: &str) 
        -> Result<String, KowalskiError>;
}
```

### Tool Trait

The `Tool` trait for implementing custom tools:

```rust
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> Vec<ToolParameter>;
    async fn execute(&mut self, input: ToolInput) -> Result<ToolOutput, KowalskiError>;
}
```

### Template Trait

The `Template` trait for creating agent templates:

```rust
pub trait Template: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn system_prompt(&self) -> String;
    fn additional_instructions(&self) -> Option<String> {
        None
    }
}
```

### Memory Traits

Memory provider interface:

```rust
#[async_trait]
pub trait MemoryProvider: Send + Sync {
    async fn add(&mut self, memory: MemoryUnit) -> Result<(), KowalskiError>;
    async fn retrieve(&self, query: &str, limit: usize) 
        -> Result<Vec<MemoryUnit>, KowalskiError>;
    async fn clear(&mut self) -> Result<(), KowalskiError>;
}
```

Recall interface:

```rust
#[async_trait]
pub trait Recall: Send + Sync {
    async fn recall(&self, context: &str, limit: usize) 
        -> Result<Vec<MemoryUnit>, KowalskiError>;
}
```

## Core Types

### Config

Configuration structure:

```rust
pub struct Config {
    pub ollama: OllamaConfig,
    pub qdrant: QdrantConfig,
    pub memory: MemoryConfig,
}

pub struct OllamaConfig {
    pub host: String,
    pub port: u16,
    pub model: String,
}

pub struct QdrantConfig {
    pub url: String,
    pub collection: String,
}

pub struct MemoryConfig {
    pub episodic_path: String,
}
```

### MemoryUnit

Represents a unit of memory:

```rust
pub struct MemoryUnit {
    pub id: String,
    pub content: String,
    pub timestamp: u64,
    pub metadata: HashMap<String, String>,
}
```

### Conversation

Represents a conversation:

```rust
pub struct Conversation {
    pub id: String,
    pub model: String,
    pub messages: Vec<Message>,
    pub created_at: u64,
}

pub struct Message {
    pub role: String,
    pub content: String,
    pub timestamp: u64,
}
```

## Error Handling

### KowalskiError

Main error type:

```rust
pub enum KowalskiError {
    ConfigError(String),
    LLMError(String),
    ToolError(String),
    MemoryError(String),
    IOError(std::io::Error),
    Other(String),
}
```

## Agent Implementations

### TemplateAgent

The main agent implementation:

```rust
pub struct TemplateAgent<T: Template> {
    template: T,
    config: Config,
    conversations: HashMap<String, Conversation>,
    tools: Vec<Box<dyn Tool>>,
    memory: Option<Box<dyn MemoryProvider>>,
}

impl<T: Template> TemplateAgent<T> {
    pub async fn new(config: Config, template: T) 
        -> Result<Self, KowalskiError>;
    
    pub fn register_tool(&mut self, tool: Box<dyn Tool>);
    
    pub async fn enable_memory(&mut self) 
        -> Result<(), KowalskiError>;
}
```

### BaseAgent

Basic agent implementation:

```rust
pub struct BaseAgent {
    name: String,
    description: String,
    config: Config,
    conversations: HashMap<String, Conversation>,
}

impl BaseAgent {
    pub async fn new(
        config: Config,
        name: &str,
        description: &str
    ) -> Result<Self, KowalskiError>;
}
```

## Tool Implementations

### WebSearchTool

```rust
pub struct WebSearchTool {
    config: Config,
}

impl WebSearchTool {
    pub fn new(config: Config) -> Self;
}
```

### CodeAnalysisTool

```rust
pub struct CodeAnalysisTool {
    config: Config,
    supported_languages: Vec<String>,
}

impl CodeAnalysisTool {
    pub fn new(config: Config) -> Self;
    pub fn with_languages(config: Config, languages: Vec<String>) -> Self;
}
```

## Memory Implementations

### EpisodicBuffer

RocksDB-based episodic memory:

```rust
pub struct EpisodicBuffer {
    db: DB,
    ollama_client: OllamaClient,
}

impl EpisodicBuffer {
    pub fn new(path: &str, config: Config) 
        -> Result<Self, KowalskiError>;
    
    pub async fn add_with_embedding(&mut self, memory: MemoryUnit) 
        -> Result<(), KowalskiError>;
    
    pub async fn retrieve_with_embedding(
        &self,
        query: &str,
        limit: usize
    ) -> Result<Vec<MemoryUnit>, KowalskiError>;
}
```

## Utility Functions

### Embedding Generation

```rust
pub async fn generate_embedding(
    text: &str,
    config: &Config
) -> Result<Vec<f32>, KowalskiError>;
```

### Conversation Management

```rust
pub fn create_conversation_id() -> String;

pub fn format_conversation_history(
    conversation: &Conversation
) -> String;
```

## Next Steps

- See [Creating Agents](./creating_agents.md) for usage examples
- Read [Using Tools](./tools_guide.md) for tool development
- Check [Memory Architecture](./memory_architecture.md) for memory system details
