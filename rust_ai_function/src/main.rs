

use dotenv::dotenv;
use ollama_rs::{coordinator::Coordinator, generation::{chat::ChatMessage, tools::implementations::Calculator}, Ollama};
use rust_ai_function::{configuration::ollama_config::OllamaConfig, llm_factory::llm::LlmVariant};



/// Get the weather for a given city
/// 
/// * city - City to get the weather for
#[ollama_rs::function]
async fn get_weather(city: String) -> Result<String, Box<dyn Error + Sync + Send>> {
    let url = format!("https://wttr.in/{}?format=%C+%t", city);
    let response = reqwest::get(&url).await?.text().await?;
    Ok(response)
}



#[tokio::main]
async fn main() {
    dotenv().ok();
    let ollama_config = OllamaConfig::inject_env();
    let ollama = Ollama::new(ollama_config.host, ollama_config.port);
    let history = vec![];
    let tools = ollama_rs::tool_group![get_weather,Calculator {}];

    let mut cordinator = Coordinator::new_with_tools(
        ollama,
        LlmVariant::MistralSmall.to_str(),
        history,
        tools);
    
    let user_message = "2+5+4";

    let ur_chat_message = ChatMessage::user(user_message.to_owned());

    let resp = cordinator.chat(vec![ur_chat_message]).await.unwrap();

    print!("Response : {}", resp.message.content)
}
