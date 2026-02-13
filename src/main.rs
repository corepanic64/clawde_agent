use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use serde_json::{Value, from_str, json};
use std::{env, process};
mod tools;
use std::process::Command;
use tools::{bash_tool, read_tool, write_tool};

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short = 'p', long)]
    prompt: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let read_tool = read_tool();
    let write_tool = write_tool();
    let bash_tool = bash_tool();

    let base_url = env::var("OPENROUTER_BASE_URL")
        .unwrap_or_else(|_| "https://openrouter.ai/api/v1".to_string());

    let api_key = env::var("OPENROUTER_API_KEY").unwrap_or_else(|_| {
        eprintln!("OPENROUTER_API_KEY is not set");
        process::exit(1);
    });

    let config = OpenAIConfig::new()
        .with_api_base(base_url)
        .with_api_key(api_key);

    let client = Client::with_config(config);
    let mut msgs: Vec<Value> = vec![];

    msgs.push(json!(
        {
            "role": "user",
            "content": args.prompt,
        }
    ));

    loop {
        let response: Value = client
            .chat()
            .create_byot(json!({
                "messages": msgs,
                "model": "anthropic/claude-haiku-4.5",
                "tools": [read_tool, write_tool, bash_tool]
            }))
            .await?;
        let message = &response["choices"][0]["message"];

        let assistant_msg = json!({
            "role": "assistant",
            "content": message.get("content"),
            "tool_calls": message.get("tool_calls"),
        });
        msgs.push(assistant_msg);

        if let Some(choices) = message["tool_calls"].as_array() {
            if !choices.is_empty() {
                for tool_call in choices {
                    let id = tool_call["id"].as_str().unwrap();
                    let func_name = tool_call["function"]["name"].as_str().unwrap();
                    let func_args = tool_call["function"]["arguments"].as_str().unwrap();
                    // let func_params = tool_call["function"]["parameters"].as_str().unwrap();
                    match func_name {
                        "Read" => {
                            let args = from_str::<Value>(func_args)?;
                            let args = args.as_object().unwrap();
                            let path = args.get("file_path").unwrap().as_str().unwrap();
                            let content = std::fs::read_to_string(path)?;
                            msgs.push(json!({
                                    "role": "tool",
                                    "tool_call_id": id,
                                    "content": content
                            }));
                        }
                        "Write" => {
                            let args = from_str::<Value>(func_args)?;
                            let args = args.as_object().unwrap();
                            let path = args.get("file_path").unwrap().as_str().unwrap();
                            let content = args.get("content").unwrap().as_str().unwrap();
                            std::fs::write(path, content)?;
                            msgs.push(json!({
                                    "role": "tool",
                                    "tool_call_id": id,
                                    "content":""
                            }));
                        }
                        "Bash" => {
                            let args = from_str::<Value>(func_args)?;
                            let args = args.as_object().unwrap();
                            let command = args.get("command").unwrap().as_str().unwrap();
                            let output = Command::new("sh").arg("-c").arg(command).output()?;
                            let content = if output.status.success() {
                                output.stdout
                            } else {
                                output.stderr
                            };
                            msgs.push(json!({
                                    "role": "tool",
                                    "tool_call_id": id,
                                    "content": String::from_utf8(content)?
                            }));
                        }
                        _ => println!(
                            "Well it looks like your tool is not Read buddy. The name (long pause) is the Rustagen(t)"
                        ),
                    }
                }
            }
        } else if let Some(tool) = response["choices"][0]["message"]["tool_calls"].get(0) {
            let read_name = tool["function"]["name"].as_str().unwrap();
            match read_name {
                "Read" => {
                    let args = from_str::<Value>(tool["function"]["arguments"].as_str().unwrap())?;
                    let args = args.as_object().unwrap();
                    let path = args.get("file_path").unwrap();
                    let content = std::fs::read_to_string(path.as_str().unwrap()).unwrap();
                    println!("{content}");
                    break;
                }
                _ => {}
            }
        } else if let Some(content) = response["choices"][0]["message"]["content"].as_str() {
            println!("{}", content);
            break;
        }
    }
    Ok(())
}
