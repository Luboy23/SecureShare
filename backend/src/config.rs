// 导入 Debug 和 Clone trait，使得 Config 结构体能够打印调试信息，并允许克隆其实例
#[derive(Debug, Clone)]
pub struct Config {
    // 数据库连接的 URL
    pub database_url: String,
    // JWT 密钥，用于加密和验证 JWT 令牌
    pub jwt_secret: String,
    // JWT 的最大有效期，单位是秒
    pub jwt_maxage: i64,
    // 服务器的端口号
    pub port: u16,
}

// 实现 Config 结构体的方法
impl Config {

    // 初始化 Config 配置，返回 Config 实例
    pub fn init() -> Config {
        // 从环境变量中获取 DATABASE_URL，如果没有设置该环境变量，程序会报错并退出
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        // 从环境变量中获取 JWT_SECRET_KEY，如果没有设置该环境变量，程序会报错并退出
        let jwt_secret = std::env::var("JWT_SECRET_KEY").expect("JWT_SECRET_KEY must be set");

        // 从环境变量中获取 JWT_MAXAGE，若没有设置该环境变量，程序会报错并退出
        let jwt_maxage = std::env::var("JWT_MAXAGE").expect("JWT_MAXAGE must be set");

        // 返回一个 Config 实例，解析 JWT_MAXAGE 并将其转换为 i64 类型，端口号默认为 8000
        Config {
            database_url,
            jwt_secret,
            // 将 JWT_MAXAGE 环境变量值解析为 i64 类型，并处理解析失败的情况
            jwt_maxage: jwt_maxage.parse::<i64>().unwrap(),
            // 默认端口设置为 8000
            port: 8000,
        }
    }
}