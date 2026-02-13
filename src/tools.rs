#[derive(serde::Serialize)]
pub struct Read {
    r#type: &'static str,
    function: ToolFunc,
}

#[derive(serde::Serialize)]
pub struct ToolFunc {
    name: &'static str,
    description: &'static str,
    parameters: ToolParams,
}

#[derive(serde::Serialize)]
pub struct ToolParams {
    r#type: &'static str,
    properties: ToolProps,
    requierd: Vec<&'static str>,
}

#[derive(serde::Serialize)]
pub struct ToolProps {
    file_path: ToolFile,
}

#[derive(serde::Serialize)]
pub struct ToolFile {
    r#type: &'static str,
    description: &'static str,
}

pub fn read_tool() -> Read {
    return Read {
        r#type: "function",
        function: ToolFunc {
            name: "Read",
            description: "Read and return the contents of a file",
            parameters: ToolParams {
                r#type: "object",
                properties: ToolProps {
                    file_path: ToolFile {
                        r#type: "string",
                        description: "The path to the file to read",
                    },
                },
                requierd: vec!["file_path"],
            },
        },
    };
}

#[derive(serde::Serialize)]
pub struct WriteTool {
    r#type: &'static str,
    function: WriteToolFunc,
}

#[derive(serde::Serialize)]
pub struct WriteToolFunc {
    name: &'static str,
    description: &'static str,
    parameters: WriteToolParams,
}
#[derive(serde::Serialize)]
pub struct WriteToolParams {
    r#type: &'static str,
    required: Vec<&'static str>,
    properties: WriteToolProps,
}

#[derive(serde::Serialize)]
pub struct WriteToolProps {
    file_path: WriteToolPropsFilePath,
    content: WriteToolPropsContent,
}

#[derive(serde::Serialize)]
pub struct WriteToolPropsFilePath {
    r#type: &'static str,
    description: &'static str,
}

#[derive(serde::Serialize)]
pub struct WriteToolPropsContent {
    r#type: &'static str,
    description: &'static str,
}

pub fn write_tool() -> WriteTool {
    return WriteTool {
        r#type: "function",
        function: WriteToolFunc {
            name: "Write",
            description: "Write content to a file",
            parameters: WriteToolParams {
                r#type: "object",
                required: vec!["file_path", "content"],
                properties: WriteToolProps {
                    file_path: WriteToolPropsFilePath {
                        r#type: "string",
                        description: "The path of the file to write to",
                    },
                    content: WriteToolPropsContent {
                        r#type: "string",
                        description: "The content to write to the file",
                    },
                },
            },
        },
    };
}

#[derive(serde::Serialize)]
pub struct BashTool {
    r#type: &'static str,
    function: BashToolFunc,
}

#[derive(serde::Serialize)]
pub struct BashToolFunc {
    name: &'static str,
    description: &'static str,
    parameters: BashToolParams,
}

#[derive(serde::Serialize)]
pub struct BashToolParams {
    r#type: &'static str,
    required: Vec<&'static str>,
    properties: BashToolProps,
}

#[derive(serde::Serialize)]
pub struct BashToolProps {
    command: BashToolCommand,
}

#[derive(serde::Serialize)]
pub struct BashToolCommand {
    r#type: &'static str,
    description: &'static str,
}

pub fn bash_tool() -> BashTool {
    return BashTool {
        r#type: "function",
        function: BashToolFunc {
            name: "Bash",
            description: "Execute a shell command",
            parameters: BashToolParams {
                r#type: "object",
                required: vec!["command"],
                properties: BashToolProps {
                    command: BashToolCommand {
                        r#type: "string",
                        description: "The command to execute",
                    },
                },
            },
        },
    };
}
