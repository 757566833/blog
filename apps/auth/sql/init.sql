CREATE USER blog_user WITH PASSWORD 'blog_password';

-- 创建数据库
CREATE DATABASE blog_db OWNER blog_user;

-- 授权用户可以使用该数据库
GRANT ALL PRIVILEGES ON DATABASE blog_db TO blog_user;

-- psql -U blog_user -d blog_db

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account VARCHAR(100) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    create_time TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

-- 用户表（假外键：仅存储 account_id，不加约束）
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    account VARCHAR(100) NOT NULL UNIQUE,
    nickname VARCHAR(100),
    avatar_url TEXT,
    create_time TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    update_time TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);

CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
   NEW.update_time = CURRENT_TIMESTAMP;
   RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- 应用于 accounts 表
CREATE TRIGGER trigger_update_accounts
BEFORE UPDATE ON accounts
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();

-- 应用于 users 表
CREATE TRIGGER trigger_update_users
BEFORE UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION update_timestamp();