#[derive(serde::Serialize)]
pub struct Read<'a> {
    r#type: &'a str,
    function: ToolFunc<'a>,
}

#[derive(serde::Serialize)]
pub struct ToolFunc<'a> {
    name: &'a str,
    description: &'a str,
    parameters: ToolParams<'a>,
}

#[derive(serde::Serialize)]
pub struct ToolParams<'a> {
    r#type: &'a str,
    properties: ToolProps<'a>,
    requierd: Vec<&'a str>,
}

#[derive(serde::Serialize)]
pub struct ToolProps<'a> {
    file_path: ToolFile<'a>,
}

#[derive(serde::Serialize)]
pub struct ToolFile<'a> {
    r#type: &'a str,
    description: &'a str,
}

pub fn read_tool<'a>() -> Read<'a> {
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
