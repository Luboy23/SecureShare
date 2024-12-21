// 导入标准库的 `str` 模块，用于处理字符串
use core::str;
// 导入 `chrono` 库，用于日期和时间的处理，`DateTime` 表示时间点，`Utc` 是 UTC 时区
use chrono::{DateTime, Utc};
// 导入 `serde` 库，用于数据的序列化与反序列化
use serde::{Deserialize, Serialize};
// 导入 `validator` 库，用于表单验证
use validator::{Validate, ValidationError};

// 导入其他模块中的数据结构
use crate::models::{ReceiveFileDetails, SendFileDetails, User};

// 注册用户数据传输对象（DTO）结构体
#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]  // 派生了验证、调试、默认值、克隆、序列化和反序列化等功能
pub struct RegisterUserDto {
    // 用户名字段，必须有值且不能为空
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,  // 用户名字段

    // 邮箱字段，必须有值且符合邮箱格式
    #[validate(
        length(min = 1, message = "Email is required"),
        email(message = "Email is invalid")
    )]
    pub email: String,  // 用户邮箱字段

    // 密码字段，必须有值且至少为 6 个字符
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,  // 用户密码字段

    // 确认密码字段，必须有值且与密码一致
    #[validate(
        length(min = 1, message = "Confirm Password is required"),
        must_match(other = "password", message="passwords do not match")
    )]
    #[serde(rename = "passwordConfirm")]  // 使用不同的字段名用于序列化和反序列化
    pub password_confirm: String,  // 用户确认密码字段
}

// 登录用户数据传输对象（DTO）结构体
#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]  // 派生了验证、调试、默认值、克隆、序列化和反序列化等功能
pub struct LoginUserDto {
    // 邮箱字段，必须有值且符合邮箱格式
    #[validate(length(min = 1, message = "Email is required"), email(message = "Email is invalid"))]
    pub email: String,  // 用户邮箱字段

    // 密码字段，必须有值且至少为 6 个字符
    #[validate(
        length(min = 1, message = "Password is required"),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,  // 用户密码字段
}

// 请求查询参数数据传输对象（DTO）结构体，用于分页等查询
#[derive(Serialize, Deserialize, Validate)]  // 派生序列化、反序列化和验证功能
pub struct RequestQueryDto {
    // 页码，必须大于等于 1
    #[validate(range(min = 1))]
    pub page: Option<usize>,  // 页码字段（可选）

    // 每页条目数，必须在 1 和 50 之间
    #[validate(range(min = 1, max = 50))]
    pub limit: Option<usize>,  // 每页条目数字段（可选）
}

// 用户筛选数据传输对象（DTO）结构体，用于显示或查询用户信息
#[derive(Debug, Serialize, Deserialize)]  // 派生调试、序列化和反序列化功能
pub struct FilterUserDto {
    pub id: String,                // 用户的 ID（字符串类型）
    pub name: String,              // 用户名
    pub email: String,             // 用户邮箱
    pub public_key: Option<String>, // 用户的公钥，可能为空
    pub created_at: DateTime<Utc>, // 用户创建时间
    pub updated_at: DateTime<Utc>, // 用户更新时间
}

// 用于描述用户数据的结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUserDto, // 用户信息，使用 FilterUserDto 进行数据过滤
}

// 用户响应数据的结构体
#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub status: String, // 请求状态
    pub data: UserData,  // 用户数据
}

// 用户发送文件的 DTO（数据传输对象）
#[derive(Debug, Serialize, Deserialize)]
pub struct UserSendFileDto {
    pub file_id: String, // 文件 ID
    pub file_name: String, // 文件名称
    pub recipient_email: String, // 接收者的邮箱
    pub expiration_date: DateTime<Utc>, // 文件过期时间
    pub created_at: DateTime<Utc>, // 文件创建时间
}

impl UserSendFileDto {
    // 用于过滤和构造 UserSendFileDto 的方法
    pub fn filter_send_user_file(file_data: &SendFileDetails) -> Self {
        UserSendFileDto {
            file_id: file_data.file_id.to_string(),
            file_name: file_data.file_name.to_owned(),
            recipient_email: file_data.recipient_email.to_owned(),
            expiration_date: file_data.expiration_date.unwrap(),
            created_at: file_data.created_at.unwrap(),
        }
    }

    // 用于处理多个文件数据并返回 UserSendFileDto 列表的静态方法
    pub fn filter_send_user_files(user: &[SendFileDetails]) -> Vec<UserSendFileDto> {
        user.iter().map(UserSendFileDto::filter_send_user_file).collect()
    }
}

// 用户发送文件列表响应 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct UserSendFileListResponseDto {
    pub status: String, // 响应状态
    pub files: Vec<UserSendFileDto>, // 文件列表
    pub results: i64, // 返回结果的总数
}

// 用户接收文件的 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct UserReceiveFileDto {
    pub file_id: String, // 文件 ID
    pub file_name: String, // 文件名称
    pub sender_email: String, // 发送者邮箱
    pub expiration_date: DateTime<Utc>, // 文件过期时间
    pub created_at: DateTime<Utc>, // 文件创建时间
}

impl UserReceiveFileDto {
    // 用于过滤和构造 UserReceiveFileDto 的方法
    pub fn filter_receive_user_file(file_data: &ReceiveFileDetails) -> Self {
        UserReceiveFileDto {
            file_id: file_data.file_id.to_string(),
            file_name: file_data.file_name.to_owned(),
            sender_email: file_data.sender_email.to_owned(),
            expiration_date: file_data.expiration_date.unwrap(),
            created_at: file_data.created_at.unwrap(),
        }
    }

    // 用于处理多个文件数据并返回 UserReceiveFileDto 列表的静态方法
    pub fn filter_receive_user_files(user: &[ReceiveFileDetails]) -> Vec<UserReceiveFileDto> {
        user.iter().map(UserReceiveFileDto::filter_receive_user_file).collect()
    }
}

// 用户接收文件列表响应 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct UserReceiveFileListResponseDto {
    pub status: String, // 响应状态
    pub files: Vec<UserReceiveFileDto>, // 文件列表
    pub results: i64, // 返回结果的总数
}

// 用户登录响应的 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponseDto {
    pub status: String, // 登录状态
    pub token: String,  // 用户的认证 token
}

// 通用响应 DTO
#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str, // 响应状态
    pub message: String,      // 响应消息
}

// 更新用户名的 DTO
#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct NameUpdateDto {
    #[validate(length(min = 1, message = "Name is required"))] // 校验名称不能为空
    pub name: String, // 新的用户名
}

// 更新用户密码的 DTO
#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct UserPasswordUpdateDto {
    #[validate(
        length(min = 1, message = "New password is required."), // 校验新密码不能为空
        length(min = 6, message = "new password must be at least 6 characters") // 新密码至少 6 位
    )]
    pub new_password: String, // 新密码

    #[validate(
        length(min = 1, message = "New password confirm is required."), // 校验确认新密码不能为空
        length(min = 6, message = "new password confirm must be at least 6 characters"), // 确认密码至少 6 位
        must_match(other = "new_password", message="new passwords do not match") // 确认密码和新密码必须匹配
    )]
    pub new_password_confirm: String, // 确认新密码

    #[validate(
        length(min = 1, message = "Old password is required."), // 校验旧密码不能为空
        length(min = 6, message = "Old password must be at least 6 characters") // 旧密码至少 6 位
    )]
    pub old_password: String, // 旧密码
}

// 通过电子邮件查询用户的 DTO
#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct SearchQueryByEmailDTO {
    #[validate(length(min = 1, message = "Query is required"))] // 校验查询条件不能为空
    pub query: String, // 查询条件（电子邮件）
}

// 用于过滤用户邮箱的 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct FilterEmailDto {
    pub email: String, // 用户的邮箱
}

impl FilterEmailDto {
    // 过滤单个用户的邮箱
    pub fn filter_email(user: &User) -> Self {
        FilterEmailDto {
            email: user.email.to_owned(),
        }
    }

    // 过滤多个用户的邮箱
    pub fn filter_emails(user: &[User]) -> Vec<FilterEmailDto> {
        user.iter().map(FilterEmailDto::filter_email).collect()
    }
}

// 返回用户邮箱列表的响应 DTO
#[derive(Debug, Serialize, Deserialize)]
pub struct EmailListResponseDto {
    pub status: String, // 响应状态
    pub emails: Vec<FilterEmailDto>, // 用户邮箱列表
}

// 文件上传 DTO
#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct FileUploadDtos {
    #[validate(email(message = "Invalid email format"))] // 校验邮箱格式是否合法
    pub recipient_email: String, // 接收者的邮箱

    #[validate(
        length(min = 1, message = "New password is required."), // 校验新密码不能为空
        length(min = 6, message = "New password must be at least 6 characters") // 新密码至少 6 位
    )]
    pub password: String, // 文件加密密码

    #[validate(custom = "validate_expiration_date")] // 自定义的过期日期验证
    pub expiration_date: String, // 文件过期日期
}

// 自定义的过期日期验证函数
fn validate_expiration_date(expiration_date: &str) -> Result<(), ValidationError> {
    if expiration_date.is_empty() { // 如果过期日期为空，返回错误
        let mut error = ValidationError::new("expiration_date_required");
        error.message = Some("Expiration date is required.".into());
        return Err(error);
    }

    // 解析过期日期
    let parsed_date = DateTime::parse_from_rfc3339(expiration_date)
    .map_err(|_| {
        let mut error = ValidationError::new("invalid_date_format");
        error.message = Some("Invalid date format. Expected format is YYYY-MM-DDTHH:MM:SS.ssssssZ.".into());
        error
    })?;

    // 获取当前时间并校验过期时间是否大于当前时间
    let now = Utc::now();

    if parsed_date <= now { // 如果过期时间小于或等于当前时间，返回错误
        let mut error = ValidationError::new("expiration_date_future");
        error.message = Some("Expiration date must be in the future.".into());
        return Err(error);
    }

    Ok(())
}

// 用于文件检索的 DTO
#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct RetrieveFileDto {
    #[validate(length(min = 1, message = "Shared id is required"))] // 校验共享 ID 必须存在
    pub shared_id: String, // 共享 ID

    #[validate(
        length(min = 1, message = "Password is required."), // 校验密码不能为空
        length(min = 6, message = "Password must be at least 6 characters") // 密码至少 6 位
    )]
    pub password: String, // 密码
}