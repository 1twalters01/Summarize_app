# Users Tables
## Initialisation
sudo -i -u postgres
psql -U username -d database_name
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "pg_cron";

## Users
| Field              | Type         | Description                 | UNIQUE | NOT NULL | INDEX |
|--------------------|--------------|-----------------------------|--------|----------|-------|
| id                 | INT          | Primary key (internal)      | True   | True     | True  |
| uuid               | UUID         | External identifier         | True   | True     | True  |
| username           | VARCHAR(50)  | The user’s username         | True   | True     | True  |
| email              | VARCHAR(100) | The user’s email            | True   | True     | True  |
| first_name         | VARCHAR(50)  | The user's first name       | False  | False    | False |
| last_name          | VARCHAR(50)  | The user's last name        | False  | False    | False |
| created_at         | TIMESTAMP    | The user's creation time    | False  | True     | False |
| last_login         | TIMESTAMP    | The user's last login       | False  | False    | False |
| is_author          | BOOLEAN      | Is the user an author       | False  | True     | False |
| is_staff           | BOOLEAN      | Is the user staff           | False  | True     | False |
| is_superuser       | BOOLEAN      | Is the user a superuser     | False  | True     | False |

```sql
CREATE TABLE IF NOT EXISTS users(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    uuid UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    first_name VARCHAR(50),
    last_name VARCHAR(50),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_login TIMESTAMP,
    is_author BOOLEAN NOT NULL DEFAULT FALSE,
    is_staff BOOLEAN NOT NULL DEFAULT FALSE,
    is_superuser BOOLEAN NOT NULL DEFAULT FALSE
);
CREATE INDEX IF NOT EXISTS idx_users_email ON users (email);
CREATE INDEX IF NOT EXISTS idx_users_username ON users (username);
CREATE INDEX IF NOT EXISTS idx_users_uuid ON users (uuid);
```

## Password History
| Field              | Type         | Description                 | UNIQUE | NOT NULL | INDEX |
|--------------------|--------------|-----------------------------|--------|----------|-------|
| id                 | INT          | Primary key                 | True   | True     | True  |
| user_id            | INT          | Foreign key to user id      | True   | True     | False |
| password_hash      | VARCHAR(255) | The user's password hash    | False  | True     | False |
| created_at         | TIMESTAMP    | The password creation time  | False  | True     | False |

```sql
CREATE TABLE IF NOT EXISTS password_history(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
```

### Example usage - Get the latest password
```sql
SELECT password_hash
FROM password_history
WHERE user_id = 1
ORDER BY created_at DESC
LIMIT 1;
```

## Totp Secrets
| Field              | Type         | Description                 | UNIQUE | NOT NULL | INDEX |
|--------------------|--------------|-----------------------------|--------|----------|-------|
| user_id            | INT          | Foreign key to user id      | True   | True     | False |
| encrypted_totp_key | VARCHAR(100) | The encrypted totp key      | False  | True     | False |
| last_updated       | TIMESTAMP    | Time of last totp update    | False  | True     | False |
| is_activated       | BOOLEAN      | Is the totp activated?      | False  | True     | False |
| is_verified        | BOOLEAN      | Is the totp verified?       | False  | True     | False |
| verified_at        | TIMESTAMP    | Totp key verification time  | False  | False    | False |

```sql
CREATE TABLE IF NOT EXISTS totp_secrets(
    user_id int UNIQUE NOT NULL,
    encrypted_totp_key VARCHAR(100) NOT NULL,
    last_updated TIMESTAMP NOT NULL DEFAULT NOW(),
    is_activated BOOLEAN NOT NULL,
    is_verified BOOLEAN NOT NULL,
    verified_at TIMESTAMP,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
```

## Refresh Tokens
| Field              | Type         | Description                 | UNIQUE | NOT NULL | INDEX |
|--------------------|--------------|-----------------------------|--------|----------|-------|
| user_id            | INT          | Foreign key to user id      | False  | True     | False |
| refresh_token      | VARCHAR(50)  | The refresh token           | True   | True     | False |
| created_at         | TIMESTAMP    | The token's creation time   | False  | True     | False |
| expires_at         | TIMESTAMP    | The token's expiration time | False  | True     | False |

```sql
CREATE TABLE IF NOT EXISTS refresh_tokens(
    user_id int NOT NULL,
    refresh_token VARCHAR UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP DEFAULT NOW() + INTERVAL '1 week',
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
CONSTRAINT unique_user_token UNIQUE (user_id, refresh_token);

SELECT cron.schedule(
    'delete_expired_rows_refresh_tokens',
    '0 0 * * *',
    $$ DELETE FROM refresh_tokens WHERE expires_at < NOW(); $$
);
```

## Token Blacklist
| Field              | Type         | Description                 | UNIQUE | NOT NULL | INDEX |
|--------------------|--------------|-----------------------------|--------|----------|-------|
| user_id            | INT          | Foreign key to user id      | False  | True     | False |
| token              | VARCHAR(150) | The token to be blacklist   | True   | True     | True  |
| expires_at         | TIMESTAMP    | The token's expiration time | False  | True     | False |

```sql
CREATE TABLE IF NOT EXISTS token_blacklist(
    user_id int UNIQUE NOT NULL,
    token VARCHAR(150) UNIQUE NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_blacklist_token ON token_blacklist (token);

SELECT cron.schedule(
    'delete_expired_rows_token_blacklist',
    '0 0 * * *',
    $$ DELETE FROM token_blacklist WHERE expires_at < NOW(); $$
);
```
