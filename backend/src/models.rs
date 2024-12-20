// 导入 `serde` 库，用于数据的序列化与反序列化
use serde::{Deserialize, Serialize};
// 导入 `sqlx` 库，主要用于与数据库进行交互
use sqlx;
// 导入 `chrono` 库，用于日期和时间操作，`DateTime` 表示时间点，`Utc` 表示 UTC 时区
use chrono::{DateTime, Utc};

// 用户数据结构，包含了用户信息
#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow, sqlx::Type)]  // 派生 Debug, Clone, Deserialize, Serialize, sqlx::FromRow 和 sqlx::Type
pub struct User {
    pub id: uuid::Uuid,             // 用户唯一标识符 (UUID)
    pub name: String,               // 用户名
    pub email: String,              // 用户邮箱
    pub password: String,           // 用户密码
    pub public_key: Option<String>, // 用户的公钥，可能为空
    pub created_at: Option<DateTime<Utc>>, // 用户创建时间，可能为空
    pub updated_at: Option<DateTime<Utc>>,

}

// 文件数据结构，包含了文件的基本信息
#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow, sqlx::Type)] // 派生 Debug, Clone, Deserialize, Serialize, sqlx::FromRow 和 sqlx::Type
pub struct File {
    pub id: uuid::Uuid,                    // 文件唯一标识符 (UUID)
    pub user_id: Option<uuid::Uuid>,       // 文件所属用户的唯一标识符 (UUID)，可能为空
    pub file_name: String,                 // 文件名
    pub file_size: i64,                    // 文件大小 (字节数)
    pub encrypted_ase_key: Vec<u8>,        // 加密后的 AES 密钥
    pub encrypted_file: Vec<u8>,           // 加密后的文件数据
    pub iv: Vec<u8>,                       // 初始化向量 (IV) 用于加密解密
    pub created_at: Option<DateTime<Utc>>,  // 文件上传时间，可能为空
}

// 文件分享链接数据结构，包含了分享链接的基本信息
#[derive(Debug, Clone, Deserialize, Serialize, sqlx::FromRow, sqlx::Type)] // 派生 Debug, Clone, Deserialize, Serialize, sqlx::FromRow 和 sqlx::Type
pub struct ShareLink {
    pub id: uuid::Uuid,                    // 分享链接唯一标识符 (UUID)
    pub file_id: Option<uuid::Uuid>,       // 被分享文件的唯一标识符 (UUID)，可能为空
    pub recipient_user_id: Option<uuid::Uuid>, // 接收者的用户标识符 (UUID)，可能为空
    pub password: String,                  // 访问文件的密码
    pub expiration_date: Option<DateTime<Utc>>, // 分享链接的过期时间，可能为空
    pub created_at: Option<DateTime<Utc>>,  // 分享链接创建时间，可能为空
}

// 发送文件详情数据结构，包含了发送文件的基本信息
#[derive(sqlx::FromRow)] // 仅派生 sqlx::FromRow，用于从数据库行中转换成结构体
pub struct SendFileDetails {
    pub file_id: uuid::Uuid,            // 文件的唯一标识符 (UUID)
    pub file_name: String,              // 文件名
    pub recipient_email: String,       // 接收者的邮箱
    pub expiration_date: Option<DateTime<Utc>>, // 文件过期时间，可能为空
    pub created_at: Option<DateTime<Utc>>, // 文件发送时间，可能为空
}

// 接收文件详情数据结构，包含了接收文件的基本信息
#[derive(sqlx::FromRow)] // 仅派生 sqlx::FromRow，用于从数据库行中转换成结构体
pub struct ReceiveFileDetails {
    pub file_id: uuid::Uuid,            // 文件的唯一标识符 (UUID)
    pub file_name: String,              // 文件名
    pub sender_email: String,          // 发送者的邮箱
    pub expiration_date: Option<DateTime<Utc>>, // 文件过期时间，可能为空
    pub created_at: Option<DateTime<Utc>>, // 文件接收时间，可能为空
}