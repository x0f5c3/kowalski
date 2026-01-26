# Configuration

Kowalski uses TOML configuration files for flexible setup and customization.

## Configuration File Location

By default, Kowalski looks for `config.toml` in the current directory. You can specify a different location:

```bash
kowalski-cli --config /path/to/config.toml
```

## Basic Configuration

Here's a minimal `config.toml`:

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

## Configuration Sections

### Ollama Configuration

Configure the Ollama LLM provider:

```toml
[ollama]
# Ollama server host
host = "localhost"

# Ollama server port
port = 11434

# Default model to use
model = "llama3.2"

# Optional: Temperature for generation (0.0 to 1.0)
temperature = 0.7

# Optional: Top-p sampling
top_p = 0.9

# Optional: Maximum tokens to generate
max_tokens = 2048
```

### Qdrant Configuration

Configure the Qdrant vector database for semantic memory:

```toml
[qdrant]
# Qdrant server URL
url = "http://localhost:6333"

# Collection name for storing embeddings
collection = "kowalski_memory"

# Optional: Vector size (must match embedding model)
vector_size = 384

# Optional: Distance metric
distance = "Cosine"
```

### Memory Configuration

Configure the memory system:

```toml
[memory]
# Path for episodic memory (RocksDB)
episodic_path = "./data/episodic_db"

# Optional: Maximum episodic memory entries
max_episodic_entries = 10000

# Optional: Episodic memory retention (days)
episodic_retention_days = 30

# Optional: Enable semantic memory
enable_semantic = true

# Optional: Semantic search limit
semantic_search_limit = 10
```

### Tool Configuration

Configure individual tools:

```toml
[tools.web_search]
enabled = true
max_results = 10
timeout_seconds = 30

[tools.code_analysis]
enabled = true
max_file_size_mb = 10
supported_languages = ["rust", "python", "java"]

[tools.pdf]
enabled = true
max_pages = 100
extract_images = false
```

### Logging Configuration

Configure logging levels:

```toml
[logging]
# Log level: trace, debug, info, warn, error
level = "info"

# Log to file
log_file = "./logs/kowalski.log"

# Enable colored output
colored = true
```

### Agent Configuration

Configure agent-specific settings:

```toml
[agent]
# Default agent name
default_name = "Kowalski"

# Default system prompt
default_prompt = "You are a helpful AI assistant."

# Conversation timeout (seconds)
conversation_timeout = 3600

# Maximum conversation history
max_history_entries = 100
```

## Environment Variables

You can override configuration values using environment variables:

```bash
# Override Ollama host
export KOWALSKI_OLLAMA_HOST="192.168.1.100"

# Override model
export KOWALSKI_OLLAMA_MODEL="llama3.1"

# Override Qdrant URL
export KOWALSKI_QDRANT_URL="http://vectordb:6333"
```

Environment variable naming convention:
- Prefix with `KOWALSKI_`
- Use uppercase
- Replace dots with underscores
- Example: `[ollama]` → `host` → `KOWALSKI_OLLAMA_HOST`

## Configuration in Code

Load and use configuration programmatically:

```rust
use kowalski_core::config::Config;

// Load from default location
let config = Config::default();

// Load from specific file
let config = Config::from_file("my_config.toml")?;

// Access configuration values
println!("Model: {}", config.ollama.model);
println!("Qdrant URL: {}", config.qdrant.url);
```

### Creating Configuration Programmatically

```rust
use kowalski_core::config::{Config, OllamaConfig, QdrantConfig, MemoryConfig};

let config = Config {
    ollama: OllamaConfig {
        host: "localhost".to_string(),
        port: 11434,
        model: "llama3.2".to_string(),
        ..Default::default()
    },
    qdrant: QdrantConfig {
        url: "http://localhost:6333".to_string(),
        collection: "my_collection".to_string(),
        ..Default::default()
    },
    memory: MemoryConfig {
        episodic_path: "./data/memory".to_string(),
        ..Default::default()
    },
};
```

## Configuration Validation

Kowalski validates configuration on load:

```rust
use kowalski_core::config::Config;

match Config::from_file("config.toml") {
    Ok(config) => {
        println!("Configuration loaded successfully");
        // Use config
    }
    Err(e) => {
        eprintln!("Configuration error: {}", e);
        // Handle error
    }
}
```

## Best Practices

1. **Version Control**: Keep `config.toml.example` in version control, but `.gitignore` the actual `config.toml`
2. **Secrets**: Don't store API keys in config files; use environment variables
3. **Defaults**: Provide sensible defaults for all optional settings
4. **Documentation**: Comment your configuration files
5. **Validation**: Always validate configuration values before use

## Example: Complete Configuration

```toml
# Kowalski Configuration File

[ollama]
host = "localhost"
port = 11434
model = "llama3.2"
temperature = 0.7
max_tokens = 2048

[qdrant]
url = "http://localhost:6333"
collection = "kowalski_semantic"
vector_size = 384
distance = "Cosine"

[memory]
episodic_path = "./data/episodic"
max_episodic_entries = 10000
episodic_retention_days = 30
enable_semantic = true
semantic_search_limit = 10

[tools.web_search]
enabled = true
max_results = 10
timeout_seconds = 30

[tools.code_analysis]
enabled = true
max_file_size_mb = 10
supported_languages = ["rust", "python", "java"]

[logging]
level = "info"
log_file = "./logs/kowalski.log"
colored = true

[agent]
default_name = "Kowalski"
conversation_timeout = 3600
max_history_entries = 100
```

## Next Steps

- Learn about [Memory Architecture](./memory_architecture.md)
- Explore [Creating Agents](./creating_agents.md)
- Read [Use Cases](./use_cases.md) for practical examples
