# Contributing

> "Contributing is like dating – it's fun until someone suggests changes." – An Open Source Maintainer

We welcome contributions to Kowalski! This guide will help you get started.

## Ways to Contribute

- **Bug Reports**: Found a bug? Open an issue!
- **Feature Requests**: Have an idea? We'd love to hear it!
- **Code Contributions**: PRs are always welcome
- **Documentation**: Help improve or expand the docs
- **Examples**: Share your use cases and examples
- **Testing**: Help test new features and report issues

## Getting Started

### 1. Fork and Clone

```bash
# Fork the repository on GitHub, then:
git clone https://github.com/YOUR_USERNAME/kowalski.git
cd kowalski
```

### 2. Set Up Development Environment

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install development dependencies
cargo install cargo-watch
cargo install cargo-edit

# Build the project
cargo build
```

### 3. Create a Branch

```bash
git checkout -b feature/my-awesome-feature
# or
git checkout -b fix/that-annoying-bug
```

## Development Workflow

### Running Tests

```bash
# Run all tests
cargo test

# Run tests for a specific crate
cargo test -p kowalski-core

# Run tests with output
cargo test -- --nocapture
```

### Code Quality

We use several tools to maintain code quality:

```bash
# Format code
cargo fmt

# Check for common mistakes
cargo clippy

# Check for unused dependencies
cargo machete

# Check for security advisories
cargo audit
```

### Building Documentation

```bash
# Build API documentation
cargo doc --no-deps --open

# Build the mdbook
cd book
mdbook build
mdbook serve  # To preview locally
```

## Code Style Guidelines

### Rust Code Style

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for automatic formatting
- Address all `clippy` warnings
- Write doc comments for public APIs

Example:

```rust
/// Represents a memory unit in the episodic buffer.
///
/// A memory unit stores a piece of information with metadata
/// about when and how it was created.
///
/// # Examples
///
/// ```
/// use kowalski_core::memory::MemoryUnit;
///
/// let memory = MemoryUnit::new("Remember this", None);
/// ```
pub struct MemoryUnit {
    pub id: String,
    pub content: String,
    pub timestamp: u64,
}
```

### Commit Messages

Follow conventional commits:

```
type(scope): subject

body (optional)

footer (optional)
```

Types:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

Examples:

```
feat(core): add semantic memory support

Implement semantic memory using Qdrant vector database
for long-term knowledge storage and retrieval.

Closes #123
```

```
fix(cli): handle empty input gracefully

Previously, empty input would cause a panic. Now it
shows a helpful message instead.
```

## Pull Request Process

### 1. Prepare Your PR

- Ensure all tests pass: `cargo test`
- Format your code: `cargo fmt`
- Run clippy: `cargo clippy`
- Update documentation if needed
- Add tests for new features

### 2. Open the PR

- Provide a clear title and description
- Reference related issues
- Explain what changed and why
- Include examples if relevant

### 3. Code Review

- Be responsive to feedback
- Make requested changes
- Push updates to your branch
- The PR will be merged once approved

## Adding New Features

### Adding a New Tool

1. Create the tool in `kowalski-tools/src/`:

```rust
// kowalski-tools/src/my_tool.rs

use kowalski_core::tool::Tool;
use async_trait::async_trait;

pub struct MyTool {
    config: Config,
}

#[async_trait]
impl Tool for MyTool {
    fn name(&self) -> &str {
        "my_tool"
    }
    
    fn description(&self) -> &str {
        "Description of what this tool does"
    }
    
    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                // Define parameters
            }
        })
    }
    
    async fn execute(&self, params: Value) -> Result<String, KowalskiError> {
        // Implement tool logic
        Ok("result".to_string())
    }
}
```

2. Add tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_my_tool() {
        let config = Config::default();
        let tool = MyTool::new(config);
        let params = json!({"param": "value"});
        let result = tool.execute(params).await;
        assert!(result.is_ok());
    }
}
```

3. Update documentation and examples

### Adding a New Agent

1. Create a new crate in the workspace
2. Implement the agent using `TemplateAgent`
3. Add examples and tests
4. Update the main README

## Testing Guidelines

- Write tests for all new features
- Aim for high code coverage
- Include both unit and integration tests
- Test error cases and edge conditions

Example:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_agent_creation() {
        let builder = DefaultTemplate::create_agent(
            Vec::new(),
            Some("Test agent".to_string()),
            None,
        ).await;
        assert!(builder.is_ok());
        let agent = builder.unwrap().build().await;
        assert!(agent.is_ok());
    }

    #[tokio::test]
    async fn test_conversation_start() {
        let mut agent = create_test_agent().await;
        let conv_id = agent.start_conversation("llama3.2");
        assert!(!conv_id.is_empty());
    }
}
```

## Documentation Guidelines

- Document all public APIs
- Include examples in doc comments
- Update the mdbook for major features
- Keep the README up to date

## Need Help?

- Open an issue for questions
- Join discussions on GitHub
- Check existing issues and PRs

## Code of Conduct

Be respectful and constructive in all interactions. We're all here to build something great together!

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

Thank you for contributing to Kowalski! 🚀
