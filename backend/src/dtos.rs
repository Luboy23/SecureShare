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

impl FilterUserDto {
    pub fn filter_user(user: &User) -> Self {
        FilterUserDto {
            id: user.id.to_string(),
            name: user.name.to_owned(),
            email: user.email.to_owned(),
            public_key: user.public_key.to_owned(),
            created_at: user.created_at.unwrap(),
            updated_at: user.updated_at.unwrap(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: FilterUserDto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponseDto {
    pub status: String,
    pub data: UserData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSendFileDto {
    pub file_id: String,
    pub file_name: String,
    pub recipient_email: String,
    pub expiration_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl UserSendFileDto {
    pub fn filter_send_user_file(file_data: &SendFileDetails) -> Self {
        UserSendFileDto {
            file_id: file_data.file_id.to_string(),
            file_name: file_data.file_name.to_owned(),
            recipient_email: file_data.recipient_email.to_owned(),
            expiration_date: file_data.expiration_date.unwrap(),
            created_at: file_data.created_at.unwrap(),
        }
    }

    pub fn filter_send_user_files(user: &[SendFileDetails]) -> Vec<UserSendFileDto> {
        user.iter().map(UserSendFileDto::filter_send_user_file).collect()
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserSendFileListResponseDto {
    pub status: String,
    pub files: Vec<UserSendFileDto>,
    pub results: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserReceiveFileDto {
    pub file_id: String,
    pub file_name: String,
    pub sender_email: String,
    pub expiration_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl UserReceiveFileDto {
    pub fn filter_receive_user_file(file_data: &ReceiveFileDetails) -> Self {
        UserReceiveFileDto {
            file_id: file_data.file_id.to_string(),
            file_name: file_data.file_name.to_owned(),
            sender_email: file_data.sender_email.to_owned(),
            expiration_date: file_data.expiration_date.unwrap(),
            created_at: file_data.created_at.unwrap(),
        }
    }

    pub fn filter_receive_user_files(user: &[ReceiveFileDetails]) -> Vec<UserReceiveFileDto> {
        user.iter().map(UserReceiveFileDto::filter_receive_user_file).collect()
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct UserReceiveFileListResponseDto {
    pub status: String,
    pub files: Vec<UserReceiveFileDto>,
    pub results: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserLoginResponseDto {
    pub status: String,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: &'static str,
    pub message: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct NameUpdateDto {
    #[validate(length(min = 1, message = "Name is required"))]
    pub name: String,
}

#[derive(Debug, Validate, Default, Clone, Serialize, Deserialize)]
pub struct UserPasswordUpdateDto {
    #[validate(
        length(min = 1, message = "New password is required."),
        length(min = 6, message = "new password must be at least 6 characters")
    )]
    pub new_password: String,

    #[validate(
        length(min = 1, message = "New password confirm is required."),
        length(min = 6, message = "new password confirm must be at least 6 characters"),
        must_match(other = "new_password", message="new passwords do not match")
    )]
    pub new_password_confirm: String,

    #[validate(
        length(min = 1, message = "Old password is required."),
        length(min = 6, message = "Old password must be at least 6 characters")
    )]
    pub old_password: String,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct SearchQueryByEmailDTO {
    #[validate(length(min = 1, message = "Query is requireed"))]
    pub query: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterEmailDto {
    pub email: String,
}

impl FilterEmailDto {
    pub fn filter_email(user: &User) -> Self {
        FilterEmailDto {
            email: user.email.to_owned(),
        }
    }

    pub fn filter_emails(user: &[User]) -> Vec<FilterEmailDto> {
        user.iter().map(FilterEmailDto::filter_email).collect()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EmailListResponseDto {
    pub status: String,
    pub emails: Vec<FilterEmailDto>,
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct FileUploadDtos {
    #[validate(email(message = "Invalid email format"))]
    pub recipient_email: String,

    #[validate(
        length(min = 1, message = "New password is required."),
        length(min = 6, message = "New password must be at least 6 characters")
    )]
    pub password: String,

    #[validate(custom = "validate_expiration_date")]
    pub expiration_date: String,
}

fn validate_expiration_date(expiration_date: &str) -> Result<(), ValidationError> {
    if expiration_date.is_empty() {
        let mut error = ValidationError::new("expiration_date_required");
        error.message = Some("Expiration date is required.".into());
        return Err(error);
    }

    let parsed_date = DateTime::parse_from_rfc3339(expiration_date)
    .map_err(|_| {
        let mut error = ValidationError::new("invalid_date_format");
        error.message = Some("Invalid date format. Expected format is YYYY-MM-DDTHH:MM:SS.ssssssZ.".into());
        error
    })?;

    let now = Utc::now();

    if parsed_date <= now {
        let mut error = ValidationError::new("expiration_date_future");
        error.message = Some("Expiration date must be in the future.".into());
        return Err(error);
    }

    Ok(())
}

#[derive(Validate, Debug, Default, Clone, Serialize, Deserialize)]
pub struct RetrieveFileDto {
    #[validate(length(min = 1, message = "Shared id is required"))]
    pub shared_id: String,

    #[validate(
        length(min = 1, message = "Password is required."),
        length(min = 6, message = "Password must be at least 6 characters")
    )]
    pub password: String,
}