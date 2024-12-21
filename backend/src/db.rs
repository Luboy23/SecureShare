use async_trait::async_trait; // 引入 `async_trait` 宏，用于支持异步特征（trait）。
use chrono::{DateTime, Utc};  // 引入 `chrono` 库的日期时间类型，用于处理时间和日期。
use sqlx::{Pool, Postgres};  // 引入 `sqlx` 库，用于与 PostgreSQL 数据库交互。
use uuid::Uuid;              // 引入 `uuid` 库，用于生成和处理唯一标识符。

// 引入当前模块中的模型（例如文件、用户、共享链接等），用于操作数据库返回的实体。
use crate::models::{File, ReceiveFileDetails, SendFileDetails, SharedLink, User};

/// 数据库客户端结构体
/// 用于封装与 PostgreSQL 数据库的连接池。
#[derive(Debug, Clone)] // 为结构体派生调试和克隆功能。
pub struct DBClient {
    pool: Pool<Postgres>, // 数据库连接池，用于管理和复用与 PostgreSQL 的连接。
}

impl DBClient {
    /// 创建新的 `DBClient` 实例
    ///
    /// # 参数
    /// - `pool`: 数据库连接池。
    ///
    /// # 返回
    /// 返回一个封装了连接池的 `DBClient` 实例。
    pub fn new(pool: Pool<Postgres>) -> Self {
        DBClient { pool }
    }
}

/// 定义一个用户相关的扩展接口（异步特征）
/// 该特征包含多个与用户和文件管理相关的异步操作。
#[async_trait]
pub trait UserExt {
    /// 根据用户 ID、用户名或邮箱获取用户信息
    ///
    /// # 参数
    /// - `user_id`: 用户唯一标识符（可选）。
    /// - `name`: 用户名（可选）。
    /// - `email`: 用户邮箱（可选）。
    ///
    /// # 返回
    /// 返回包含用户信息的 `Option<User>` 或查询错误。
    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error>;

    /// 保存新的用户信息
    ///
    /// # 参数
    /// - `name`: 用户名。
    /// - `email`: 用户邮箱。
    /// - `password`: 用户密码。
    ///
    /// # 返回
    /// 返回保存成功的 `User` 或操作错误。
    async fn save_user<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
    ) -> Result<User, sqlx::Error>;

    /// 更新用户的用户名
    ///
    /// # 参数
    /// - `user_id`: 用户唯一标识符。
    /// - `name`: 新用户名。
    ///
    /// # 返回
    /// 返回更新后的 `User` 或操作错误。
    async fn update_user_name<T: Into<String> + Send>(
        &self,
        user_id: Uuid,
        name: T,
    ) -> Result<User, sqlx::Error>;

    /// 更新用户的密码
    ///
    /// # 参数
    /// - `user_id`: 用户唯一标识符。
    /// - `password`: 新密码。
    ///
    /// # 返回
    /// 返回更新后的 `User` 或操作错误。
    async fn update_user_password(
        &self,
        user_id: Uuid,
        password: String,
    ) -> Result<User, sqlx::Error>;

    /// 保存用户的公钥信息
    ///
    /// # 参数
    /// - `user_id`: 用户唯一标识符。
    /// - `public_key`: 用户的公钥。
    ///
    /// # 返回
    /// 返回操作结果（成功或错误）。
    async fn save_user_key(&self, user_id: Uuid, public_key: String) -> Result<(), sqlx::Error>;

    /// 根据邮箱搜索用户
    ///
    /// # 参数
    /// - `user_id`: 当前用户 ID（用于限制搜索范围）。
    /// - `query`: 搜索关键词（邮箱部分匹配）。
    ///
    /// # 返回
    /// 返回符合条件的用户列表或查询错误。
    async fn search_by_email(&self, user_id: Uuid, query: String)
        -> Result<Vec<User>, sqlx::Error>;

    /// 保存加密文件
    ///
    /// # 参数
    /// - `user_id`: 上传者 ID。
    /// - `file_name`: 文件名。
    /// - `file_size`: 文件大小（字节）。
    /// - `recipient_user_id`: 接收者 ID。
    /// - `password`: 文件密码。
    /// - `expiration_date`: 文件到期时间。
    /// - `encrypted_aes_key`: 加密后的 AES 密钥。
    /// - `encrypted_file`: 加密后的文件内容。
    /// - `iv`: 初始化向量。
    ///
    /// # 返回
    /// 返回操作结果（成功或错误）。
    async fn save_encrypted_file(
        &self,
        user_id: Uuid,
        file_name: String,
        file_size: i64,
        recipient_user_id: Uuid,
        password: String,
        expiration_date: DateTime<Utc>,
        encrypted_aes_key: Vec<u8>,
        encrypted_file: Vec<u8>,
        iv: Vec<u8>,
    ) -> Result<(), sqlx::Error>;

    /// 获取共享链接信息
    ///
    /// # 参数
    /// - `shared_id`: 共享链接 ID。
    /// - `user_id`: 当前用户 ID。
    ///
    /// # 返回
    /// 返回共享链接信息或查询错误。
    async fn get_shared(
        &self,
        shared_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<SharedLink>, sqlx::Error>;

    /// 获取文件信息
    ///
    /// # 参数
    /// - `file_id`: 文件 ID。
    ///
    /// # 返回
    /// 返回文件信息或查询错误。
    async fn get_file(
        &self,
        file_id: Uuid,
    ) -> Result<Option<File>, sqlx::Error>;

    /// 获取用户发送的文件列表
    ///
    /// # 参数
    /// - `user_id`: 用户 ID。
    /// - `page`: 分页页码。
    /// - `limit`: 每页条目数。
    ///
    /// # 返回
    /// 返回文件详情列表和总记录数，或查询错误。
    async fn get_sent_files(
        &self,
        user_id: Uuid,
        page: u32,
        limit: usize
    ) -> Result<(Vec<SendFileDetails>, i64), sqlx::Error>;

    /// 获取用户接收的文件列表
    ///
    /// # 参数
    /// - `user_id`: 用户 ID。
    /// - `page`: 分页页码。
    /// - `limit`: 每页条目数。
    ///
    /// # 返回
    /// 返回文件详情列表和总记录数，或查询错误。
    async fn get_receive_files(
        &self,
        user_id: Uuid,
        page: u32,
        limit: usize
    ) -> Result<(Vec<ReceiveFileDetails>, i64), sqlx::Error>;

    /// 删除已过期的文件
    ///
    /// # 返回
    /// 返回操作结果（成功或错误）。
    async fn delete_expired_files(
        &self
    ) -> Result<(), sqlx::Error>;
}


#[async_trait]
impl UserExt for DBClient {
    async fn get_user(
        &self,
        user_id: Option<Uuid>,
        name: Option<&str>,
        email: Option<&str>,
    ) -> Result<Option<User>, sqlx::Error> {
        let mut user: Option<User> = None;

        if let Some(user_id) = user_id {
            user = sqlx::query_as!(
                User,
                r#"SELECT id, name, email, password, public_key, created_at, updated_at FROM users WHERE id = $1"#,
                user_id
            ).fetch_optional(&self.pool).await?;
        } else if let Some(name) = name {
            user = sqlx::query_as!(
                User,
                r#"SELECT id, name, email, password, public_key, created_at, updated_at FROM users WHERE name = $1"#,
                name
            ).fetch_optional(&self.pool).await?;
        } else if let Some(email) = email {
            user = sqlx::query_as!(
                User,
                r#"SELECT id, name, email, password, public_key, created_at, updated_at FROM users WHERE email = $1"#,
                email
            ).fetch_optional(&self.pool).await?;
        }

        Ok(user)
    }

    async fn save_user<T: Into<String> + Send>(
        &self,
        name: T,
        email: T,
        password: T,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (name, email, password) 
            VALUES ($1, $2, $3) 
            RETURNING id, name, email, password, public_key, created_at, updated_at
            "#,
            name.into(),
            email.into(),
            password.into()
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    async fn update_user_name<T: Into<String> + Send>(
        &self,
        user_id: Uuid,
        new_name: T,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET name = $1, updated_at = Now()
            WHERE id = $2
            RETURNING id, name, email, password, public_key, created_at, updated_at
            "#,
            new_name.into(),
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn update_user_password(
        &self,
        user_id: Uuid,
        new_password: String,
    ) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET password = $1, updated_at = Now()
            WHERE id = $2
            RETURNING id, name, email, password, public_key, created_at, updated_at
            "#,
            new_password,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    async fn save_user_key(&self, user_id: Uuid, public_key: String) -> Result<(), sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            UPDATE users
            SET public_key = $1, updated_at = Now()
            WHERE id = $2
            RETURNING id, name, email, password, public_key, created_at, updated_at
            "#,
            public_key,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(())
    }
    async fn search_by_email(
        &self,
        user_id: Uuid,
        query: String,
    ) -> Result<Vec<User>, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, name, email, password, public_key, created_at, updated_at
            FROM users
            WHERE email LIKE $1
            AND public_key IS NOT NULL
            AND id != $2
            "#,
            query,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(user)
    }
    async fn save_encrypted_file(
        &self,
        user_id: Uuid,
        file_name: String,
        file_size: i64,
        recipient_user_ud: Uuid,
        password: String,
        expiration_date: DateTime<Utc>,
        encrypted_aes_key: Vec<u8>,
        encrypted_file: Vec<u8>,
        iv: Vec<u8>,
    ) -> Result<(), sqlx::Error> {
        // Insert into the files table and get the file_id
        let file_id: Uuid = sqlx::query_scalar!(
            r#"
            INSERT INTO files (user_id, file_name, file_size, encrypted_aes_key, encrypted_file, iv, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW())
            RETURNING id
            "#,
            user_id,
            file_name,
            file_size,
            encrypted_aes_key,
            encrypted_file,
            iv
        )
        .fetch_one(&self.pool)
        .await?;

        // Insert into the shared_links table using the returned file_id
        sqlx::query!(
            r#"
            INSERT INTO shared_links (file_id, recipient_user_id, password, expiration_date, created_at)
            VALUES ($1, $2, $3, $4, NOW())
            "#,
            file_id,
            recipient_user_ud,
            password,
            expiration_date
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_shared(
        &self,
        shared_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<SharedLink>, sqlx::Error> {
        let shared_link = sqlx::query_as!(
            SharedLink,
            r#"
            SELECT id, file_id, recipient_user_id, password, expiration_date, created_at
            FROM shared_links
            WHERE id = $1
            AND recipient_user_id = $2
            AND expiration_date > NOW()
            "#,
            shared_id,
            user_id,
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(shared_link)
    }

    async fn get_file(
        &self,
        file_id: Uuid,
    ) -> Result<Option<File>, sqlx::Error> {
        let file = sqlx::query_as!(
            File,
            r#"
            SELECT id, user_id, file_name, file_size, encrypted_aes_key, encrypted_file, iv, created_at
            FROM files
            WHERE id = $1
            "#,
            file_id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(file)
    }
    async fn get_sent_files(
        &self,
        user_id: Uuid,
        page: u32,
        limit: usize
    ) -> Result<(Vec<SendFileDetails>, i64), sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        let files = sqlx::query_as!(
            SendFileDetails,
            r#"
                SELECT
                    f.id AS file_id,
                    f.file_name,
                    u.email AS recipient_email,
                    sl.expiration_date,
                    sl.created_at
                FROM 
                    shared_links sl
                JOIN 
                    files f ON sl.file_id = f.id
                JOIN 
                    users u ON sl.recipient_user_id = u.id
                WHERE 
                    f.user_id = $1
                ORDER BY 
                    sl.created_at DESC 
                LIMIT $2 
                OFFSET $3
            "#,
            user_id,
            limit as i64,
            offset as i64,
        )
        .fetch_all(&self.pool)
        .await?;

        let count_row = sqlx::query_scalar!(
            r#"
                SELECT COUNT(*)
                FROM shared_links sl
                JOIN files f ON sl.file_id = f.id
                WHERE f.user_id = $1
            "#,
            user_id,
        )
        .fetch_one(&self.pool)
        .await?;

        let total_count = count_row.unwrap_or(0);

        Ok((files, total_count))
    }

    async fn get_receive_files(
        &self,
        user_id: Uuid,
        page: u32,
        limit: usize
    ) -> Result<(Vec<ReceiveFileDetails>, i64), sqlx::Error> {
        let offset = (page - 1) * limit as u32;

        let files = sqlx::query_as!(
            ReceiveFileDetails,
            r#"
                SELECT
                    sl.id AS file_id,
                    f.file_name,
                    u.email AS sender_email,
                    sl.expiration_date,
                    sl.created_at
                FROM 
                    shared_links sl
                JOIN 
                    files f ON sl.file_id = f.id
                JOIN 
                    users u ON f.user_id = u.id
                WHERE 
                    sl.recipient_user_id = $1
                ORDER BY 
                    sl.created_at DESC 
                LIMIT $2 
                OFFSET $3
            "#,
            user_id,
            limit as i64,
            offset as i64,
        )
        .fetch_all(&self.pool)
        .await?;

        let count_row = sqlx::query_scalar!(
            r#"
                SELECT COUNT(*)
                FROM shared_links sl
                JOIN files f ON sl.file_id = f.id
                WHERE sl.recipient_user_id = $1
            "#,
            user_id,
        )
        .fetch_one(&self.pool)
        .await?;

        let total_count = count_row.unwrap_or(0);

        Ok((files, total_count))
    }

    async fn delete_expired_files(
        &self
    ) -> Result<(), sqlx::Error> {
        
        let expired_shared_links: Vec<Uuid> = sqlx::query_scalar!(
            r#"
            SELECT sl.id
            FROM shared_links sl
            WHERE sl.expiration_date < NOW()
            "#,
        ).
        fetch_all(&self.pool)
        .await?;

        if expired_shared_links.is_empty() {
            println!("No expired files or shared links to delete.");
            return Ok(());
        }

        let expired_file_ids: Vec<Uuid> = sqlx::query_scalar!(
            r#"
            SELECT f.id
            FROM files f
            WHERE f.id IN (
                SELECT sl.file_id
                FROM shared_links sl
                WHERE sl.expiration_date < NOW()
            )
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        sqlx::query!(
            r#"
            DELETE FROM shared_links
            WHERE id = ANY($1)
            "#,
            &expired_shared_links[..] // Pass the list of expired shared link IDs
        )
        .execute(&self.pool)
        .await?;

        // Delete the expired files
        sqlx::query!(
            r#"
            DELETE FROM files
            WHERE id = ANY($1)
            "#,
            &expired_file_ids[..] // Pass the list of expired file IDs
        )
        .execute(&self.pool)
        .await?;

        println!("Successfully deleted expired files and their shared links.");

        Ok(())

    }
}