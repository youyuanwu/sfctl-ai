use std::{collections::VecDeque, vec};

use futures::StreamExt;
use genai::{
    Client, ModelIden,
    chat::{ChatMessage, ChatRequest, ChatStreamEvent, printer::PrintChatStreamOptions},
    resolver::{AuthData, AuthResolver},
};

use crate::{cmd_parse::CmdKind, model::extract_code_blocks, pwsh::PwshSession};

const MODEL: &str = "gemini-2.0-flash";
const SYSTEM_PROMPT: &str = include_str!("system_prompt.txt");

pub struct AiConnection {
    pub client: Client,
}

impl AiConnection {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // -- Build an auth_resolver and the AdapterConfig
        let auth_resolver = AuthResolver::from_resolver_fn(
            |model_iden: ModelIden| -> Result<Option<AuthData>, genai::resolver::Error> {
                let ModelIden {
                    adapter_kind,
                    model_name,
                } = model_iden;
                tracing::info!(
                    "\n>> Custom auth provider for {adapter_kind} (model: {model_name}) <<"
                );

                // This will cause it to fail if any model is not an GEMINI_API_KEY
                let key = std::env::var("GEMINI_API_KEY").map_err(|_| {
                    genai::resolver::Error::ApiKeyEnvNotFound {
                        env_name: "GEMINI_API_KEY".to_string(),
                    }
                })?;
                Ok(Some(AuthData::from_single(key)))
            },
        );

        // -- Build the new client with this adapter_config
        let client = Client::builder().with_auth_resolver(auth_resolver).build();

        Ok(AiConnection { client })
    }

    // pub async fn run_user_prompt(&self, command: &str) -> Result<String, Box<dyn std::error::Error>> {
    //     let mut chat_req = ChatRequest::default().with_system("Answer in one sentence");

    //     for &question in questions {
    //         chat_req = chat_req.append_message(ChatMessage::user(question));

    //         println!("\n--- Question:\n{question}");
    //         let chat_res = client
    //             .exec_chat_stream(MODEL, chat_req.clone(), None)
    //             .await
    //             .unwrap();

    //         println!("\n--- Answer: (streaming)");
    //         let assistant_answer = print_chat_stream(chat_res, None).await.unwrap();

    //         chat_req = chat_req.append_message(ChatMessage::assistant(assistant_answer));
    //     }
    //     Ok(())
    // }

    pub fn create_chat(&self) -> AiChat {
        // let tool = Tool::new("ServiceFabric Powershell")
        //     .with_description("Run a Service Fabric Powershell command");

        // let tools = vec![tool];

        // Create the chat request with the system prompt and tools
        let req = ChatRequest::default().with_system(SYSTEM_PROMPT);
        //.with_tools(tools);
        AiChat {
            req,
            client: self.client.clone(),
            pwsh_session: PwshSession::new().expect("cannot open powershell session"),
            pending_ps_commands: VecDeque::new(),
            pending_ps_commands_results: VecDeque::new(),
            pending_user_input: VecDeque::new(),
        }
    }
}

pub struct AiChat {
    req: ChatRequest,
    client: Client,
    pwsh_session: PwshSession,
    pending_ps_commands: VecDeque<String>,
    pending_ps_commands_results: VecDeque<(String, String)>,
    pending_user_input: VecDeque<String>,
}

impl AiChat {
    pub async fn process_ps_command(&mut self) {
        while let Some(code) = self.pending_ps_commands.pop_front() {
            let code = PwshSession::trim_command(&code);
            // classify the command
            let kind = crate::cmd_parse::classify_cmd(&code);
            let need_ack = !matches!(kind, CmdKind::Read);

            // ask user permission to run the command
            let ack = if need_ack {
                crate::ack::ack_command(&code).await
            } else {
                true
            };
            let tools_content = if !ack {
                tracing::info!("User declined to run the command: {}", code);
                println!("Please provide reason for declining:");
                let reason = crate::ack::get_user_input().await;
                tracing::info!("User reason for declining: {}", reason);
                if !reason.is_empty() {
                    self.pending_user_input.push_back(reason);
                }
                format!("User declined to run the command: {}", code)
            } else {
                self.pwsh_session
                    .run_command(code.as_str())
                    .await
                    .unwrap_or_else(|e| format!("Error running command: {e}"))
            };
            tracing::info!("Tool Response: {}", tools_content);
            self.pending_ps_commands_results
                .push_back((code, tools_content));
        }
    }

    pub async fn send_ps_result_to_chat(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.pending_ps_commands_results.is_empty() {
            tracing::info!("No pending PowerShell command results to send.");
            return Ok(());
        }
        while let Some((code, tool_response)) = self.pending_ps_commands_results.pop_front() {
            self.req = self.req.clone().append_message(ChatMessage::system(format!(
                "Tool call: ```\n{}\n```\n
                Tool response: ```\n{}\n```",
                code, tool_response
            )));
        }
        while let Some(reason) = self.pending_user_input.pop_front() {
            self.req = self
                .req
                .clone()
                .append_message(ChatMessage::user(reason.to_string()));
        }
        self.run_prompt().await
    }

    pub async fn run_prompt(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.req = self.req.clone();

        let mut chat_stream = self
            .client
            .exec_chat_stream(MODEL, self.req.clone(), None)
            .await?;

        tracing::info!("--- Capturing tool calls ---");
        let mut chunks: Vec<String> = vec![];
        while let Some(result) = chat_stream.stream.next().await {
            match result? {
                ChatStreamEvent::Start => {
                    tracing::info!("Stream started");
                }
                ChatStreamEvent::Chunk(chunk) => {
                    chunks.push(chunk.content);
                }
                ChatStreamEvent::ToolCallChunk(tool_chunk) => {
                    panic!("Tool call chunk not supported: {:?}", tool_chunk);
                }
                ChatStreamEvent::ReasoningChunk(chunk) => {
                    tracing::info!("Reasoning: {}", chunk.content);
                }
                ChatStreamEvent::End(_end) => {
                    tracing::info!("Stream ended");
                }
            }
        }

        let chunks = chunks.join("");
        tracing::info!("Captured chunks: {}", chunks);
        if chunks.is_empty() {
            panic!("No chunks captured, cannot continue.");
        }

        let code_blocks = extract_code_blocks(&chunks);

        if code_blocks.is_empty() {
            tracing::info!("No code blocks captured.");
        } else {
            self.pending_ps_commands.extend(code_blocks);
        }
        let text_blocks = crate::model::extract_text_blocks(&chunks);
        if text_blocks.is_empty() {
            tracing::info!("No text blocks captured.");
        } else {
            println!("{}", text_blocks.join("\n"));
        }
        Ok(())
    }

    pub async fn get_user_input(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        println!(">");
        let input = crate::ack::get_user_input().await;
        if input.is_empty() {
            tracing::info!("No user input provided.");
            return Ok(false);
        } else {
            self.req = self
                .req
                .clone()
                .append_message(ChatMessage::user(input.clone()));
        }

        Ok(true)
    }

    pub fn has_pending_commands(&self) -> bool {
        !self.pending_ps_commands.is_empty() || !self.pending_ps_commands_results.is_empty()
    }

    pub async fn run_user_prompt_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let _print_options = PrintChatStreamOptions::from_print_events(false);
        loop {
            if !self.has_pending_commands() {
                tracing::info!("No pending commands, waiting for user input.");
                if self.get_user_input().await? {
                    self.run_prompt().await?;
                }
            }

            self.process_ps_command().await;
            self.send_ps_result_to_chat().await?;

            if self.pending_ps_commands.is_empty() {
                self.req = self.req.clone().append_message(ChatMessage::system(
                    "All commands executed. Please give the final response if any.",
                ));
                self.run_prompt().await?;
            }
            tracing::info!("pending commands: {:?}", self.pending_ps_commands);
            tracing::info!(
                "pending commands results: {:?}",
                self.pending_ps_commands_results
            );
        }
    }
}
