# Installation & Setup

> "Installation is like cooking – it's easy until you burn something." – A Frustrated Developer

## Prerequisites

Before installing Kowalski, ensure you have the following:

- **Rust** (latest stable version)
  - Install via [rustup.rs](https://rustup.rs)
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

- **Ollama** (for local LLMs)
  - Download from [ollama.com](https://ollama.com/)
  - Recommended for running models like llama3.2

- **(Optional)** Other LLM providers (OpenAI, Anthropic, etc.)

## Installation Steps

### 1. Clone the Repository

```bash
git clone https://github.com/yarenty/kowalski.git
cd kowalski
```

### 2. Build the Project

```bash
cargo build --release
```

This will compile all the crates in the workspace. The build process may take a few minutes the first time as it downloads and compiles dependencies.

### 3. Install & Run Ollama

```bash
# Install Ollama (see https://ollama.com/download)
ollama serve &

# Download a model (llama3.2 runs on CPU)
ollama pull llama3.2
```

### 4. Configure Kowalski

Copy the default configuration file:

```bash
cp config.toml.example config.toml
```

Edit `config.toml` to customize settings like:
- Ollama host and port
- Model selection
- Memory paths
- Qdrant configuration

### 5. Run Kowalski CLI

```bash
cargo run --release --bin kowalski-cli

# Or use the built binary directly
./target/release/kowalski-cli
```

## Verify Installation

To verify that Kowalski is installed correctly, try running a simple command:

```bash
./target/release/kowalski-cli
```

You should see the Kowalski CLI interface. Type `help` to see available commands.

## Next Steps

- Read the [Quick Start](./quickstart.md) guide to learn basic usage
- Explore [Architecture Overview](./architecture.md) to understand the system design
- Check out [Creating Agents](./creating_agents.md) to build your first agent
