use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// JSON-RPC 版本常量
pub const JSONRPC_VERSION: &str = "2.0";

/// JSON-RPC 错误代码
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JsonRpcErrorCode {
    /// 解析错误 (-32700)
    ParseError,
    /// 无效请求 (-32600)
    InvalidRequest,
    /// 方法未找到 (-32601)
    MethodNotFound,
    /// 无效参数 (-32602)
    InvalidParams,
    /// 内部错误 (-32603)
    InternalError,
    /// 服务器错误 (-32000 到 -32099)
    ServerError(i32),
}

impl JsonRpcErrorCode {
    /// 获取错误代码的数值
    pub fn code(&self) -> i32 {
        match self {
            JsonRpcErrorCode::ParseError => -32700,
            JsonRpcErrorCode::InvalidRequest => -32600,
            JsonRpcErrorCode::MethodNotFound => -32601,
            JsonRpcErrorCode::InvalidParams => -32602,
            JsonRpcErrorCode::InternalError => -32603,
            JsonRpcErrorCode::ServerError(code) => *code,
        }
    }

    /// 获取错误代码的默认消息
    pub fn message(&self) -> &'static str {
        match self {
            JsonRpcErrorCode::ParseError => "Parse error",
            JsonRpcErrorCode::InvalidRequest => "Invalid Request",
            JsonRpcErrorCode::MethodNotFound => "Method not found",
            JsonRpcErrorCode::InvalidParams => "Invalid params",
            JsonRpcErrorCode::InternalError => "Internal error",
            JsonRpcErrorCode::ServerError(_) => "Server error",
        }
    }
}

/// JSON-RPC 错误对象
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// 错误代码
    pub code: i32,
    /// 错误消息
    pub message: String,
    /// 错误附加数据
    pub data: Option<Value>,
}

impl JsonRpcError {
    /// 创建一个新的 JSON-RPC 错误
    pub fn new(code: JsonRpcErrorCode, message: Option<String>, data: Option<Value>) -> Self {
        JsonRpcError {
            code: code.code(),
            message: message.unwrap_or_else(|| code.message().to_string()),
            data,
        }
    }
}

/// JSON-RPC 请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC 版本，必须是 "2.0"
    pub jsonrpc: String,
    /// 方法名称
    pub method: String,
    /// 请求参数
    pub params: Option<Value>,
    /// 请求标识符
    pub id: Uuid,
}

impl JsonRpcRequest {
    /// 创建一个新的 JSON-RPC 请求
    pub fn new(method: String, params: Option<Value>, id: Uuid) -> Self {
        JsonRpcRequest {
            jsonrpc: JSONRPC_VERSION.to_string(),
            method,
            params,
            id,
        }
    }
}

/// JSON-RPC 成功响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// JSON-RPC 版本，必须是 "2.0"
    pub jsonrpc: String,
    /// 响应结果
    pub result: Value,
    /// 请求标识符
    pub id: Uuid,
}

impl JsonRpcResponse {
    /// 创建一个新的 JSON-RPC 成功响应
    pub fn new(result: Value, id: Uuid) -> Self {
        JsonRpcResponse {
            jsonrpc: JSONRPC_VERSION.to_string(),
            result,
            id,
        }
    }
}

/// JSON-RPC 错误响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcErrorResponse {
    /// JSON-RPC 版本，必须是 "2.0"
    pub jsonrpc: String,
    /// 错误对象
    pub error: JsonRpcError,
    /// 请求标识符
    pub id: Uuid,
}

impl JsonRpcErrorResponse {
    /// 创建一个新的 JSON-RPC 错误响应
    pub fn new(error: JsonRpcError, id: Uuid) -> Self {
        JsonRpcErrorResponse {
            jsonrpc: JSONRPC_VERSION.to_string(),
            error,
            id,
        }
    }
}

/// JSON-RPC 消息枚举，用于统一处理请求、响应和错误
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcMessage {
    /// 请求消息
    Request(JsonRpcRequest),
    /// 成功响应消息
    Response(JsonRpcResponse),
    /// 错误响应消息
    ErrorResponse(JsonRpcErrorResponse),
}
