use dotenv::dotenv;
use ollama_rs::{coordinator::Coordinator, generation::chat::ChatMessage, Ollama};
use rust_ai_function::{configuration::ollama_config::OllamaConfig, llm_factory::llm::LlmVariant};
use ollama_rs::{generation::tools::implementations::Calculator};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let ollama_config = OllamaConfig::inject_env();
    let ollama = Ollama::new(ollama_config.host, ollama_config.port);
    let history = vec![];
    let tools = ollama_rs::tool_group![Calculator {}];

    let mut cordinator = Coordinator::new_with_tools(
        ollama,
        LlmVariant::MistralSmall.to_str(),
        history,
        tools);
    
    let user_message = "2+2*2-4";

    let ur_chat_message = ChatMessage::user(user_message.to_owned());

    let resp = cordinator.chat(vec![ur_chat_message]).await.unwrap();

    print!("Response : {}", resp.message.content)
}
