use env_logger;
use kowalski_core::{agent::Agent, config::Config};
use kowalski_web_agent::agent::WebAgent;

use log::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::init();

    let config = Config::default();
    // Load configuration
    let mut web_agent = WebAgent::new(config.clone()).await?;

    // Start a conversation
    info!("🤖 Starting web agent...");
    let conversation_id = web_agent.start_conversation(&config.ollama.model);
    info!("Web Agent Conversation ID: {}", conversation_id);

    // Perform a web search using tool-calling
    let search_input = r#"{"name": "web_search", "parameters": {"query": "AI"}}"#;
    let search_results = web_agent
        .chat_with_tools(&conversation_id, search_input)
        .await?;
    println!("\n📑 Search Results:\n{}", search_results);

    // Optional: Use the web_scrape tool to extract content from a specific URL
    // Uncomment the following lines to scrape a web page:
    //
    // let scrape_input = r#"{"name": "web_scrape", "parameters": {"url": "https://example.com"}}"#;
    // let page_content = web_agent.chat_with_tools(&conversation_id, scrape_input).await?;
    // println!("\n🌐 Page Content:\n{}", page_content);

    // Add search query to conversation
    web_agent
        .add_message(
            &conversation_id,
            "user",
            format!("Search for {} and provide a summary", "AI").as_str(),
        )
        .await;

    // Display the search results
    println!("\n🌐 Processing search results: {}", search_results);

    Ok(())
}
