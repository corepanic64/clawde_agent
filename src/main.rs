use async_openai::{Client, config::OpenAIConfig};
use clap::Parser;
use serde_json::{Value, from_str, json};
use std::{env, process};
mod tools;
use tools::read_tool;

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

    #[allow(unused_variables)]
    let response: Value = client
        .chat()
        .create_byot(json!({
            "messages": [
                {
                    "role": "user",
                    "content": args.prompt,
                },
            ],
            "model": "anthropic/claude-haiku-4.5",
            "tools": [read_tool]
        }))
        .await?;

    if let Some(tool) = response["choices"][0]["message"]["tool_calls"].get(0) {
        let read_name = tool["function"]["name"].as_str().unwrap();
        match read_name {
            "Read" => {
                let t = tool["function"]["arguments"].as_str();
                match t {
                    Some(a) => {
                        let args = from_str::<Value>(a);
                        match args {
                            Ok(b) => {
                                let m = b.as_object();
                                match m {
                                    Some(f) => {
                                        let p = f.get("file_path");
                                        match p {
                                            Some(w) => {
                                                let pa = w.as_str();
                                                match pa {
                                                    Some(q) => {
                                                        let c = std::fs::read_to_string(q);
                                                        match c {
                                                            Ok(r) => println!("{r}"),
                                                            Err(e) => {}
                                                        }
                                                    }
                                                    None => {}
                                                }
                                            }
                                            None => {}
                                        }
                                    }
                                    None => {}
                                }
                            }
                            Err(e) => {}
                        }
                    }
                    None => {}
                }
                // let args = from_str::<Value>();
                // let args = args.as_object().unwrap();
                // let path = args.get("file_path").unwrap();
                // let content = std::fs::read_to_string(path.as_str().unwrap()).unwrap();
                // println!("{content}")
            }
            _ => {}
        }
    } else if let Some(content) = response["choices"][0]["message"]["content"].as_str() {
        println!("{}", content);
    }

    Ok(())
}
