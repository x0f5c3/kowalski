use env_logger;
use kowalski_code_agent::agent::CodeAgent;
use kowalski_core::{
    agent::Agent,
    config::Config,
    role::{Audience, Preset, Role},
};
use log::info;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    // Load configuration
    let config = Config::default();
    let mut code_agent = CodeAgent::new(config.clone()).await?;

    // Start a conversation
    info!("🤖 Starting code agent...");
    let conversation_id = code_agent.start_conversation(&config.ollama.model);
    info!("Code Agent Conversation ID: {}", conversation_id);

    // Example code to analyze (remove unused variable warning)

    println!("\n🔍 Analyzing code...");

    // Analyze the code using tool-calling
    let analysis_input = r#"{"name": "rust_analysis", "parameters": {"content": "fn calculate_factorial(n: u32) -> u32 { if n <= 1 { return 1; } n * calculate_factorial(n - 1) } fn main() { let result = calculate_factorial(5); println!(\"Factorial of 5 is: {}\", result); }"}}"#;
    let analysis_result = code_agent
        .chat_with_tools(&conversation_id, analysis_input)
        .await?;
    // Print the analysis result as a string
    println!("\n📊 Analysis Results:\n{}", analysis_result);

    // Add analysis results to conversation
    code_agent
        .add_message(
            &conversation_id,
            "analysis",
            format!("Code analysis results: {}", analysis_result).as_str(),
        )
        .await;

    // Generate a summary with a specific role
    // Set up the role for code analysis
    let role = Role::new(
        "Rust Code Analysis Assistant",
        "You are an expert at analyzing Rust code, providing insights on code quality, safety, and potential improvements."
    )
    .with_audience(Audience::new(
        "Rust Developer",
        "You are speaking to a Rust developer who needs detailed code analysis."
    ))
    .with_preset(Preset::new(
        "Analysis",
        "Provide comprehensive analysis with specific recommendations for improvement."
    ));
    println!("\n📝 Generating technical summary...");

    let mut response = code_agent
        .chat_with_history(
            &conversation_id,
            "Provide a technical summary of the analysis",
            Some(role),
        )
        .await?;

    // Process the streaming response
    let mut buffer = String::new();
    while let Some(chunk) = response.chunk().await? {
        match code_agent
            .process_stream_response(&conversation_id, &chunk)
            .await
        {
            Ok(Some(message)) => {
                // Print the content if it exists
                if !message.content.is_empty() {
                    print!("{}", message.content);
                    io::stdout().flush()?;
                    buffer.push_str(&message.content);
                }

                // Handle tool calls if they exist
                if let Some(tool_calls) = &message.tool_calls {
                    for tool_call in tool_calls {
                        print!("\n[Tool Call] {}: ", tool_call.function.name);
                        // Print arguments as a string, do not iterate
                        println!("{:?}", tool_call.function.arguments);
                        io::stdout().flush()?;
                    }
                }
            }
            Ok(None) => {
                code_agent
                    .add_message(&conversation_id, "assistant", &buffer)
                    .await;
                println!("\n✅ Analysis complete!\n");
                break;
            }
            Err(e) => {
                eprintln!("\n❌ Error processing stream: {}", e);
                break;
            }
        }
    }

    Ok(())
}
