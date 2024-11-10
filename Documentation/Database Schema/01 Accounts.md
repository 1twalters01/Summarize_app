# Users Tables
## Initialisation
sudo -i -u postgres
psql -U username -d database_name
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "pg_cron";

## Users
CREATE TABLE IF NOT EXISTS users(
    user_id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    uuid UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    first_name VARCHAR(50),
    last_name VARCHAR(50),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_login TIMESTAMP,
    is_author BOOLEAN NOT NULL,
    is_staff BOOLEAN NOT NULL,
    is_superuser BOOLEAN NOT NULL
);

| Field           | Type           | Description                 | IS UNIQUE | NOT NULL | INDEX  |
|-----------------|----------------|-----------------------------|-----------|----------|--------|
| user_id         | INT            | Primary key (internal)      | True      | True     | True   |
| uuid            | UUID           | External identifier         | True      | True     | True   |
| username        | VARCHAR(50)    | The user’s username         | True      | True     |  |
| email           | VARCHAR(100)   | The user’s email            | True      | True     |  |
| first_name      | VARCHAR(50)    | The user's first name       | False     | False    |  |
| last_name       | VARCHAR(50)    | The user's last name        | False     | False    |  |
| created_at      | TIMESTAMP      | The user's creation time    | False     | True     |  |
| last_login      | TIMESTAMP      | The user's last login       | False     | False    |  |
| is_author       | BOOLEAN        | Is the user an author       | False     | True     |  |
| is_staff        | BOOLEAN        | Is the user staff           | False     | True     |  |
| is_superuser    | BOOLEAN        | Is the user a superuser     | False     | True     |  |

## Password Hashes
CREATE TABLE IF NOT EXISTS password_hashes(
    user_id INT UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    previous_hashes VARCHAR(255)[] NOT NULL,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (user_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

| Field           | Type           | Description                 | IS UNIQUE | NOT NULL | INDEX  |
|-----------------|----------------|-----------------------------|-----------|----------|--------|
| user_id         | INT            | Foreign key to user id      | True      | True     |  |
| password_hash   | VARCHAR(255)   | The user's password hash    | False     | True     |  |
| previous_hashes | VARCHAR(255)[] | Previous pasword hashes     | False     | True     |  |

## Totp Secrets
CREATE TABLE IF NOT EXISTS totp_secrets(
    user_id int UNIQUE NOT NULL,
    totp_key VARCHAR(100) NOT NULL,
    last_updated TIMESTAMP NOT NULL DEFAULT NOW(),
    is_activated BOOLEAN NOT NULL,
    is_verified BOOLEAN NOT NULL,
    verified_at TIMESTAMP,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (user_id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

| Field           | Type           | Description                 | IS UNIQUE | NOT NULL | INDEX  |
|-----------------|----------------|-----------------------------|-----------|----------|--------|
| user_id         | INT            | Foreign key to user id      | True      | True     |  |
| totp_key        | VARCHAR(100)   | The totp key                | False     | True     |  |
| last_updated    | TIMESTAMP      | Time of last totp update    | False     | True     |  |
| is_activated    | BOOLEAN        | Is the totp activated?      | False     | True     |  |
| is_verified     | BOOLEAN        | Is the totp verified?       | False     | True     |  |
| verified_at     | TIMESTAMP      | Totp key verification time  | False     | False    |  |

## Refresh Tokens
CREATE TABLE IF NOT EXISTS refresh_tokens(
    user_id int UNIQUE NOT NULL,
    refresh_token VARCHAR UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP DEFAULT NOW() + INTERVAL '1 week'
);

SELECT cron.schedule(
    'delete_expired_rows_refresh_tokens',
    '0 0 * * *',
    $$ DELETE FROM refresh_tokens WHERE expires_at < NOW(); $$
);

| Field           | Type           | Description                 | IS UNIQUE | NOT NULL | INDEX  |
|-----------------|----------------|-----------------------------|-----------|----------|--------|
| user_id         | INT            | Foreign key to user id      | True      | True     |  |
| refresh_token   | VARCHAR(50)    | The refresh token           | True      | True     |  |
| created_at      | TIMESTAMP      | The token's creation time   | False     | True     |  |
| expires_at      | TIMESTAMP      | The token's expiration time | False     | True     |  |

## Token Blacklist
CREATE TABLE IF NOT EXISTS token_blacklist(
    user_id int UNIQUE NOT NULL,
    token VARCHAR(150) UNIQUE NOT NULL,
    expires_at TIMESTAMP DEFAULT NOW() + INTERVAL '2 weeks'
);

SELECT cron.schedule(
    'delete_expired_rows_token_blacklist',
    '0 0 * * *',
    $$ DELETE FROM token_blacklist WHERE expires_at < NOW(); $$
);

| Field           | Type           | Description                 | IS UNIQUE | NOT NULL | INDEX  |
|-----------------|----------------|-----------------------------|-----------|----------|--------|
| user_id         | INT            | Foreign key to user id      | True      | True     |  |
| token           | VARCHAR(150)   | The token to be blacklist   | True      | True     |  |
| expires_at      | TIMESTAMP      | The token's expiration time | False     | True     |  |
