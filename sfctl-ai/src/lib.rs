use tokio_util::sync::CancellationToken;
pub mod ack;
pub mod ai;
pub mod model;
pub mod pwsh;

pub async fn app_loop(token: CancellationToken) {
    let ai_conn = ai::AiConnection::new().unwrap();
    let mut chat = ai_conn.create_chat();
    println!("Welcome");
    loop {
        println!(">");
        tokio::select! {
            _ = chat.run_user_prompt_loop() => {
                // This will run until the user decides to exit
                tracing::info!("User prompt loop finished.");
            }
            _ = token.cancelled() => {
                tracing::info!("Cancellation requested, exiting loop.");
                break;
            }
        }
    }
    tracing::info!("Exiting app loop.");
}

// #[cfg(test)]
// mod tests {
//     use genai::{
//         Client, ModelIden,
//         chat::{ChatMessage, ChatRequest, printer::print_chat_stream},
//         resolver::{AuthData, AuthResolver},
//     };

//     const MODEL: &str = "gemini-2.0-flash";
//     #[tokio::test]
//     #[ignore = "Requires GEMINI_API_KEY environment variable"]
//     async fn test_genai() {
//         tracing_subscriber::fmt()
//             .with_max_level(tracing::Level::INFO)
//             .init();

//         let questions = &[
//             // Follow-up questions
//             "Why is the sky blue?",
//             "Why is it red sometimes?",
//         ];

//         // -- Build an auth_resolver and the AdapterConfig
//         let auth_resolver = AuthResolver::from_resolver_fn(
//             |model_iden: ModelIden| -> Result<Option<AuthData>, genai::resolver::Error> {
//                 let ModelIden {
//                     adapter_kind,
//                     model_name,
//                 } = model_iden;
//                 println!("\n>> Custom auth provider for {adapter_kind} (model: {model_name}) <<");

//                 // This will cause it to fail if any model is not an GEMINI_API_KEY
//                 let key = std::env::var("GEMINI_API_KEY").map_err(|_| {
//                     genai::resolver::Error::ApiKeyEnvNotFound {
//                         env_name: "GEMINI_API_KEY".to_string(),
//                     }
//                 })?;
//                 Ok(Some(AuthData::from_single(key)))
//             },
//         );

//         // -- Build the new client with this adapter_config
//         let client = Client::builder().with_auth_resolver(auth_resolver).build();

//         let mut chat_req = ChatRequest::default().with_system("Answer in one sentence");

//         for &question in questions {
//             chat_req = chat_req.append_message(ChatMessage::user(question));

//             println!("\n--- Question:\n{question}");
//             let chat_res = client
//                 .exec_chat_stream(MODEL, chat_req.clone(), None)
//                 .await
//                 .unwrap();

//             println!("\n--- Answer: (streaming)");
//             let assistant_answer = print_chat_stream(chat_res, None).await.unwrap();

//             chat_req = chat_req.append_message(ChatMessage::assistant(assistant_answer));
//         }
//     }
// }
