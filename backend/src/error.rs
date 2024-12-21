// 引入标准库中的 fmt 模块，用于格式化输出
use std::fmt;

// 引入 axum 库，用于处理 HTTP 请求和响应
use axum::{http::StatusCode, response::{IntoResponse, Response}, Json};

// 引入 serde 库，用于数据序列化和反序列化
use serde::{Deserialize, Serialize};

// 错误响应的 DTO（数据传输对象），用于封装错误信息
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub status: String,   // 错误状态
    pub message: String,  // 错误消息
}

// 为 ErrorResponse 实现 Display trait，使其可以格式化为字符串
impl fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 将 ErrorResponse 序列化为 JSON 字符串，并写入 fmt 格式化器
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

// 定义错误消息的枚举类型，用于表示不同的错误情形
#[derive(Debug, PartialEq)]  // Debug 用于调试，PartialEq 用于比较
pub enum ErrorMessage {
    EmptyPassword, // 密码为空
    ExceededMaxPasswordLength(usize), // 密码超出最大长度
    InvalidHashFormat, // 哈希格式无效
    HashingError, // 哈希错误
    InvalidToken, // 无效的令牌
    WrongCredentials, // 错误的凭证（邮箱或密码错误）
    EmailExist, // 邮箱已存在
    UserNoLongerExist, // 用户已不存在
    TokenNotProvided, // 未提供令牌
}

// 为 ErrorMessage 实现 ToString trait，允许将 ErrorMessage 转换为字符串
impl ToString for ErrorMessage {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl ErrorMessage {
    // 将 ErrorMessage 转换为具体的错误信息字符串
    fn to_str(&self) -> String {
        match self {
            ErrorMessage::WrongCredentials => "Email or password is wrong".to_string(), // 错误的凭证
            ErrorMessage::EmailExist => "A user with this email already exists".to_string(), // 邮箱已存在
            ErrorMessage::UserNoLongerExist => "User belonging to this token no longer exists".to_string(), // 用户已不存在
            ErrorMessage::EmptyPassword => "Password cannot be empty".to_string(), // 密码不能为空
            ErrorMessage::HashingError => "Error while hashing password".to_string(), // 哈希错误
            ErrorMessage::InvalidHashFormat => "Invalid password hash format".to_string(), // 哈希格式无效
            ErrorMessage::ExceededMaxPasswordLength(max_length) => format!("Password must not be more than {} characters", max_length), // 密码超出最大长度
            ErrorMessage::InvalidToken => "Authentication token is invalid or expired".to_string(), // 无效或过期的令牌
            ErrorMessage::TokenNotProvided => "You are not logged in, please provide a token".to_string(), // 未提供令牌
        }
    }
}

// 定义一个 HttpError 结构体，用于表示 HTTP 错误
#[derive(Debug, Clone)]  // Debug 用于调试，Clone 用于克隆实例
pub struct HttpError {
    pub message: String,  // 错误信息
    pub status: StatusCode, // 错误的 HTTP 状态码
}

// 为 HttpError 实现方法
impl HttpError {
    // 创建一个新的 HttpError 实例
    pub fn new(message: impl Into<String>, status: StatusCode) -> Self {
        HttpError {
            message: message.into(),  // 将 message 转换为 String
            status,  // 设置 HTTP 状态码
        }
    }

    // 创建一个 500（服务器内部错误）状态的 HttpError
    pub fn server_error(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),  // 设置错误消息
            status: StatusCode::INTERNAL_SERVER_ERROR,  // 设置 HTTP 状态码为 500
        }
    }

    // 创建一个 400（错误的请求）状态的 HttpError
    pub fn bad_request(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),  // 设置错误消息
            status: StatusCode::BAD_REQUEST,  // 设置 HTTP 状态码为 400
        }
    }

    // 创建一个 409（冲突）状态的 HttpError，用于表示唯一约束违规
    pub fn unique_constraint_violation(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),  // 设置错误消息
            status: StatusCode::CONFLICT,  // 设置 HTTP 状态码为 409
        }
    }

    // 创建一个 401（未经授权）状态的 HttpError
    pub fn unauthorized(message: impl Into<String>) -> Self {
        HttpError {
            message: message.into(),  // 设置错误消息
            status: StatusCode::UNAUTHORIZED,  // 设置 HTTP 状态码为 401
        }
    }

    // 将 HttpError 转换为 HTTP 响应
    pub fn into_http_response(self) -> Response {
        // 创建一个 JSON 格式的错误响应
        let json_response = Json(ErrorResponse {
            status: "fail".to_string(),  // 错误状态
            message: self.message.clone(),  // 错误消息
        });

        // 返回一个包含状态码和 JSON 错误消息的响应
        (self.status, json_response).into_response()
    }
}

// 为 HttpError 实现 Display trait，使其可以格式化输出为字符串
impl fmt::Display for HttpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HttpError: message: {}, status: {}",  // 格式化输出错误信息和状态码
            self.message, self.status
        )
    }
}

// 为 HttpError 实现标准错误（std::error::Error）trait，使其能够作为错误使用
impl std::error::Error for HttpError {}

// 为 HttpError 实现 IntoResponse trait，使其能够直接转换为 HTTP 响应
impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        self.into_http_response()  // 将 HttpError 转换为 HTTP 响应
    }
}