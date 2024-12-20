-- 添加迁移脚本
-- 确保 UUID 扩展可用，用于生成 UUID
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- 创建用户表
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(), -- 使用 uuid_generate_v4() 自动生成主键
    name VARCHAR(100) NOT NULL,                     -- 用户名，长度限制为 100，不能为空
    email VARCHAR(255) UNIQUE NOT NULL,             -- 用户邮箱，长度限制为 255，必须唯一且不能为空
    password VARCHAR(255) NOT NULL,                 -- 用户密码，存储加密后的密码
    public_key TEXT,                                -- 用户公钥，用于后续加密或验证操作
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(), -- 创建时间，默认当前时间
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()  -- 更新时间，默认当前时间
);

-- 创建文件表
CREATE TABLE files (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(), -- 使用 uuid_generate_v4() 自动生成主键
    user_id UUID REFERENCES users(id) ON DELETE CASCADE, -- 用户外键，关联到 users 表，当用户被删除时，关联的文件也被删除
    file_name VARCHAR(255) NOT NULL,               -- 文件名，长度限制为 255，不能为空
    file_size BIGINT NOT NULL,                     -- 文件大小，使用 BIGINT 存储，不能为空
    encrypted_aes_key BYTEA NOT NULL,              -- 加密后的 AES 密钥，用于文件加密，不能为空
    encrypted_file BYTEA NOT NULL,                 -- 加密后的文件内容，不能为空
    iv BYTEA NOT NULL,                             -- AES 加密的初始化向量，不能为空
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW() -- 创建时间，默认当前时间
);

-- 创建共享链接表
CREATE TABLE shared_links (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(), -- 使用 uuid_generate_v4() 自动生成主键
    file_id UUID REFERENCES files(id) ON DELETE CASCADE, -- 文件外键，关联到 files 表，当文件被删除时，关联的共享链接也被删除
    recipient_user_id UUID REFERENCES users(id) ON DELETE CASCADE, -- 接收用户的外键，关联到 users 表，当用户被删除时，关联的共享链接也被删除
    password VARCHAR(255) NOT NULL,               -- 共享链接的密码保护字段，不能为空
    expiration_date TIMESTAMP WITH TIME ZONE NOT NULL, -- 共享链接的过期时间，不能为空
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()  -- 创建时间，默认当前时间
);