mod pdk;

use base64::Engine;
use extism_pdk::*;
use pdk::types::*;
use serde_json::json;
use sha1::Sha1;
use sha2::{Digest, Sha224, Sha256, Sha384, Sha512};

// Called when the tool is invoked.
pub(crate) fn call(input: CallToolRequest) -> Result<CallToolResult, Error> {
    extism_pdk::log!(
        LogLevel::Info,
        "called with args: {:?}",
        input.params.arguments
    );
    let args = input.params.arguments.unwrap_or_default();

    let data = match args.get("data") {
        Some(v) => v.as_str().unwrap(),
        None => return Err(Error::msg("`data` is required")),
    };

    // Determine algorithm: prefer explicit argument; otherwise use the tool name.
    // This preserves backward compatibility with the single "hash" tool while
    // enabling per-algorithm tools like "sha256", "base64", etc.
    let algorithm = match args.get("algorithm").and_then(|v| v.as_str()) {
        Some(a) => a,
        None => input.params.name.as_str(),
    };

    let result = match algorithm {
        "sha256" => {
            let mut hasher = Sha256::new();
            hasher.update(data.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        "sha512" => {
            let mut hasher = Sha512::new();
            hasher.update(data.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        "sha384" => {
            let mut hasher = Sha384::new();
            hasher.update(data.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        "sha224" => {
            let mut hasher = Sha224::new();
            hasher.update(data.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        "sha1" => {
            let mut hasher = Sha1::new();
            hasher.update(data.as_bytes());
            format!("{:x}", hasher.finalize())
        }
        "md5" => {
            format!("{:x}", md5::compute(data))
        }
        "base32" => base32::encode(base32::Alphabet::Rfc4648 { padding: true }, data.as_bytes()),
        "base64" | _ => base64::engine::general_purpose::STANDARD.encode(data),
    };

    Ok(CallToolResult {
        is_error: None,
        content: vec![Content {
            annotations: None,
            text: Some(result),
            mime_type: Some("text/plain".into()),
            r#type: ContentType::Text,
            data: None,
        }],
    })
}

pub(crate) fn describe() -> Result<ListToolsResult, Error> {
    // Helper to build a schema requiring only the "data" string
    let data_only = || {
        json!({
            "type": "object",
            "properties": {
                "data": { "type": "string", "description": "Data to hash or encode" }
            },
            "required": ["data"],
            "additionalProperties": false
        })
        .as_object()
        .unwrap()
        .clone()
    };

    Ok(ListToolsResult {
        tools: vec![
            // Backward-compatible aggregate tool
            ToolDescription {
                name: "hash".into(),
                description: "Hash/encode data using one of: sha256, sha512, sha384, sha224, sha1, md5, base32, base64 (requires 'algorithm')".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "data": { "type": "string", "description": "Data to hash or encode" },
                        "algorithm": {
                            "type": "string",
                            "description": "Algorithm to use",
                            "enum": ["sha256", "sha512", "sha384", "sha224", "sha1", "md5", "base32", "base64"]
                        }
                    },
                    "required": ["data", "algorithm"],
                    "additionalProperties": false
                })
                .as_object()
                .unwrap()
                .clone(),
            },

            // One tool per algorithm
            ToolDescription { name: "sha256".into(), description: "SHA-256 hash (hex)".into(), input_schema: data_only() },
            ToolDescription { name: "sha512".into(), description: "SHA-512 hash (hex)".into(), input_schema: data_only() },
            ToolDescription { name: "sha384".into(), description: "SHA-384 hash (hex)".into(), input_schema: data_only() },
            ToolDescription { name: "sha224".into(), description: "SHA-224 hash (hex)".into(), input_schema: data_only() },
            ToolDescription { name: "sha1".into(), description: "SHA-1 hash (hex)".into(), input_schema: data_only() },
            ToolDescription { name: "md5".into(), description: "MD5 hash (hex)".into(), input_schema: data_only() },
            ToolDescription { name: "base32".into(), description: "Base32 encode (RFC 4648, padded)".into(), input_schema: data_only() },
            ToolDescription { name: "base64".into(), description: "Base64 encode (standard)".into(), input_schema: data_only() },
        ],
    })
}
