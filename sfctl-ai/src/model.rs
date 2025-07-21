use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub reason: String,
    pub command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub thoughts: String,
    pub answer: String,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Response {
//     pub action: Option<Action>,
//     pub answer: Option<Answer>,
// }

pub enum ResponseEnum {
    Action(Action),
    Answer(Answer),
}

pub fn get_response_enum_from_response_str(input: &str) -> Vec<ResponseEnum> {
    let mut responses = Vec::new();
    let json_blocks = extract_json_blocks(input);
    if json_blocks.is_empty() {
        return responses;
    }

    for block in json_blocks {
        if let Ok(action) = serde_json::from_str::<Action>(&block) {
            responses.push(ResponseEnum::Action(action));
        } else if let Ok(answer) = serde_json::from_str::<Answer>(&block) {
            responses.push(ResponseEnum::Answer(answer));
        }
    }
    responses
}

fn extract_json_blocks(input: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut rest = input;
    while let Some(start) = rest.find("```json") {
        let after_start = &rest[start + 7..];
        if let Some(end) = after_start.find("```") {
            let json_block = &after_start[..end].trim();
            blocks.push(json_block.to_string());
            rest = &after_start[end + 3..];
        } else {
            break;
        }
    }
    blocks
}

pub fn extract_code_blocks(input: &str) -> Vec<String> {
    extract_blocks(input, "tool_code")
}

pub fn extract_text_blocks(input: &str) -> Vec<String> {
    extract_blocks(input, "text")
}

pub fn extract_blocks(input: &str, block_type: &str) -> Vec<String> {
    let mut blocks = Vec::new();
    let mut rest = input;
    while let Some(start) = rest.find(&format!("```{}", block_type)) {
        let after_start = &rest[start + block_type.len() + 3..];
        if let Some(end) = after_start.find("```") {
            let code_block = &after_start[..end].trim();
            blocks.push(code_block.to_string());
            rest = &after_start[end + 3..];
        } else {
            break;
        }
    }
    blocks
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_STRING: &str = r#"test string
    with multiple lines
    ```json
    { "reason": "reason for using this command", "command": "pwsh command" } 
    ```
    ```json
    { "thoughts": "test thoughts", "answer": "test answer" }  
    ```"#;

    #[test]
    fn test_parse_response() {
        //
        let responses = get_response_enum_from_response_str(TEST_STRING);
        assert_eq!(responses.len(), 2, "Expected two JSON blocks");

        let j1 = &responses[0];
        let j2 = &responses[1];

        match j1 {
            ResponseEnum::Action(action) => {
                assert_eq!(action.reason, "reason for using this command");
                assert_eq!(action.command, "pwsh command");
            }
            _ => panic!("Expected Action variant"),
        }
        match j2 {
            ResponseEnum::Answer(answer) => {
                assert_eq!(answer.thoughts, "test thoughts");
                assert_eq!(answer.answer, "test answer");
            }
            _ => panic!("Expected Answer variant"),
        }
    }

    #[test]
    fn test_extract_code_blocks() {
        let mycode = r#"
        ```text
        This is a text block.
        ```

        
        ```tool_code
        let x = 42;
        ```
        ```tool_code
        let y = 100;
        ```
        ```text
        This is another text block.
        ```"#;
        let code_blocks = extract_code_blocks(&mycode);
        assert_eq!(code_blocks.len(), 2, "Expected two code blocks");
        assert_eq!(code_blocks[0], "let x = 42;", "Code block content mismatch");
        assert_eq!(
            code_blocks[1], "let y = 100;",
            "Code block content mismatch"
        );
        let text_blocks = extract_text_blocks(&mycode);
        assert_eq!(text_blocks.len(), 2, "Expected two text blocks");
        assert_eq!(
            text_blocks[0], "This is a text block.",
            "Text block content mismatch"
        );
        assert_eq!(
            text_blocks[1], "This is another text block.",
            "Text block content mismatch"
        );
    }
}
