# Users Tables
## Initialisation
sudo -i -u postgres
psql -U username -d database_name
CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS "pg_cron";

## Users
| Field                  | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|------------------------|--------------|-------------------------------|--------|----------|-------|
| id                     | INT          | Primary key (internal)        | True   | True     | True  |
| uuid                   | UUID         | External identifier           | True   | True     | True  |
| created_at             | TIMESTAMP    | The user's creation time      | False  | True     | False |
| last_login             | TIMESTAMP    | The user's last login         | False  | False    | False |
| is_guest               | BOOLEAN      | Is the user a guest           | False  | True     | False |

```sql
CREATE TABLE IF NOT EXISTS users(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    uuid UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_login TIMESTAMP,
    is_guest BOOLEAN NOT NULL DEFAULT TRUE,
);
CREATE INDEX IF NOT EXISTS idx_users_email ON users (email);
CREATE INDEX IF NOT EXISTS idx_users_username ON users (username);
CREATE INDEX IF NOT EXISTS idx_users_uuid ON users (uuid);
```

## User Data
| Field                  | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|------------------------|--------------|-------------------------------|--------|----------|-------|
| user_id                | INT          | Foreign key to user id        | True   | True     | False |
| username               | VARCHAR(50)  | The user’s username           | True   | True     | True  |
| first_name             | VARCHAR(50)  | The user's first name         | False  | False    | False |
| last_name              | VARCHAR(50)  | The user's last name          | False  | False    | False |

```sql
CREATE TABLE IF NOT EXISTS user_data(
    user_id INT PRIMARY KEY,
    username VARCHAR(50) UNIQUE NOT NULL,
    first_name VARCHAR(50),
    last_name VARCHAR(50),
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
```

User_id can only have set of details so it was made as the primary key

## Email Table
| Field                  | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|------------------------|--------------|-------------------------------|--------|----------|-------|
| user_id                | INT          | Foreign key to user id        | False  | True     | False |
| email                  | VARCHAR(255) | The user’s email              | True   | True     | True  |

```sql
CREATE TABLE IF NOT EXISTS emails(
    user_id INT PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
```

For normal sign in.

## Oauth Providers
| Field                  | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|------------------------|--------------|-------------------------------|--------|----------|-------|
| id                     | INT          | Primary key (internal)        | True   | True     | True  |
| oauth_provider       | VARCHAR(20)  | What created the account      | True   | True     | True  |

```sql
CREATE TABLE IF NOT EXISTS oauth_providers (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    provider VARCHAR(20) UNIQUE NOT NULL
);
```

summarize, oauth-google, oauth-apple

## Oauth
| Field                  | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|------------------------|--------------|-------------------------------|--------|----------|-------|
| id                     | INT          | Primary key for provider      | True   | True     | True  |
| user_id                | INT          | Foreign key to user id        | False  | True     | False |
| oauth_email            | VARCHAR(255) | The user’s email              | False  | True     | True  |
| oauth_provider_id    | INT          | FK to account provider        | False  | True     | False |
| oauth_provider_user_id | VARCHAR(255) | oauth provider user id        | True   | True     | True  |

```sql
CREATE TABLE IF NOT EXISTS oauth (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    oauth_email VARCHAR(255) UNIQUE NOT NULL,
    oauth_provider_id INT UNIQUE NOT NULL,
    oauth_provider_user_id VARCHAR(255) UNIQUE NOT NULL
    UNIQUE (oauth_email, oauth_provider_id)
);
```

## Oauth Refresh Tokens
| Field                  | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|------------------------|--------------|-------------------------------|--------|----------|-------|
| id                     | INT          | Primary key                   | True   | True     | True  |
| oauth_id               | INT          | Foreign key to oauth table    | True   | True     | True  |
| oauth_refresh_token    | VARCHAR(150) | Oauth refresh token           | True   | True     | True  |

```sql
CREATE TABLE IF NOT EXISTS oauth_refresh_tokens(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    oauth_id int UNIQUE NOT NULL,
    oauth_refresh_token VARCHAR(100) UNIQUE NOT NULL,
    CONSTRAINT fk_oauth FOREIGN KEY (oauth_id)
        REFERENCES oauth (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
```

## Roles
| Field                  | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|------------------------|--------------|-------------------------------|--------|----------|-------|
| id                     | INT          | Primary key for role          | True   | True     | True  |
| role                   | Varchar(20)  | Name of the role              | True   | True     | True  |

```sql
CREATE TABLE IF NOT EXISTS roles(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name VARCHAR(20) UNIQUE NOT NULL
);
```

Current roles: Admin, Superuser, Author

## User Roles
| Field                  | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|------------------------|--------------|-------------------------------|--------|----------|-------|
| user_id                | INT          | Foreign key to user id        | True   | True     | True  |
| role_id                | INT          | Foreign key to role           | False  | True     | False |

```sql
CREATE TABLE IF NOT EXISTS user_roles(
    user_id INT,
    role_id INT,
    PRIMARY KEY (user_id, role_id),
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
    CONSTRAINT fk_role FOREIGN KEY (user_id)
    	REFERENCES roles(id)
)
```

## Password History
| Field                  | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|------------------------|--------------|-------------------------------|--------|----------|-------|
| id                     | INT          | Primary key                   | True   | True     | True  |
| user_id                | INT          | Foreign key to user id        | True   | True     | False |
| password_hash          | VARCHAR(255) | The user's password hash      | False  | True     | False |
| created_at             | TIMESTAMP    | The password creation time    | False  | True     | False |

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
| Field                  | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|------------------------|--------------|-------------------------------|--------|----------|-------|
| user_id                | INT          | Foreign key to user id        | True   | True     | False |
| encrypted_totp_key     | VARCHAR(100) | The encrypted totp key        | False  | True     | False |
| last_updated           | TIMESTAMP    | Time of last totp update      | False  | True     | False |
| is_activated           | BOOLEAN      | Is the totp activated?        | False  | True     | False |
| is_verified            | BOOLEAN      | Is the totp verified?         | False  | True     | False |
| verified_at            | TIMESTAMP    | Totp key verification time    | False  | False    | False |

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

Can turn on and off totp without regenerating it - need verified at to log when verified in this case.

## SMS
| Field                  | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|------------------------|--------------|-------------------------------|--------|----------|-------|
| user_id                | INT          | Foreign key to user id        | True   | True     | False |

```sql
CREATE TABLE IF NOT EXISTS sms_verifications (
    user_id INT UNIQUE NOT NULL,
    encrypted_phone_number VARCHAR(100) NOT NULL,
    last_updated TIMESTAMP NOT NULL DEFAULT NOW(),
    is_activated BOOLEAN NOT NULL,
    last_sent TIMESTAMP NOT NULL DEFAULT NOW(),
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    verified_at TIMESTAMP,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
```

## Biometrics Platform
| Field                  | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|------------------------|--------------|-------------------------------|--------|----------|-------|
| id                     | INT          | Primary key for provider      | True   | True     | True  |
| platform               | VARCHAR(20)  | Name of biometrics platform   | True   | True     | True  |

e.g. Apple Face ID, Android Biometric API, Windows Hello

```sql
CREATE TABLE IF NOT EXISTS biometrics_platforms (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    platform VARCHAR(20) UNIQUE NOT NULL
);
```

## Biometrics
| Field                  | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|------------------------|--------------|-------------------------------|--------|----------|-------|
| user_id                | INT          | Foreign key to user id        | True   | True     | True  |
| biometrics_platform_id | INT          | Public key given by platform  | True   | True     | True  |
| public_key             | VARCHAR(100) | Public key from platform      | True   | True     | True  |
| is_activated           | BOOLEAN      | Is biometric activated        | False  | True     | False |
| is_verified            | BOOLEAN      | Is biometric verified         | False  | True     | False |
| verified_at            | TIMESTAMP    | Verification Time             | False  | True     | False |

```sql
CREATE TABLE IF NOT EXISTS biometric_data (
    user_id INT UNIQUE NOT NULL,
    biometrics_platform_id INT NOT NULL,
    public_key TEXT NOT NULL,
    last_updated TIMESTAMP NOT NULL DEFAULT NOW(),
    is_activated BOOLEAN NOT NULL,
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    verified_at TIMESTAMP,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
    CONSTRAINT fk_biometrics_platform FOREIGN KEY (biometrics_platform_id)
        REFERENCES biometrics_platforms (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
```

Authentication Workflow with Platform Biometrics
Registration

    User registers on the app and enables biometric authentication.
    Platform generates a key pair (public/private).
    Public key is sent to my server and stored in the biometric_data table.

Login
    Server generates and stores a challenge (random data e.g. a random string) in redis
    Server sends the challenge to the user's device.
    Platform verifies the biometric data locally.
    If the biometric match succeeds:
        The private key signs the challenge.
        The signed challenge and user identifier are sent back to your server.
    Server validates the signature using the stored public key.
    If valid, the user is authenticated.
    

## Refresh Tokens
| Field                  | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|------------------------|--------------|-------------------------------|--------|----------|-------|
| user_id                | INT          | Foreign key to user id        | False  | True     | False |
| refresh_token          | VARCHAR(50)  | The refresh token             | True   | True     | False |
| created_at             | TIMESTAMP    | The token's creation time     | False  | True     | False |
| expires_at             | TIMESTAMP    | The token's expiration time   | False  | True     | False |

```sql
CREATE TABLE IF NOT EXISTS refresh_tokens(
    user_id int NOT NULL,
    refresh_token VARCHAR UNIQUE NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP DEFAULT NOW() + INTERVAL '1 Day',
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
| Field                  | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|------------------------|--------------|-------------------------------|--------|----------|-------|
| user_id                | INT          | Foreign key to user id        | False  | True     | False |
| token                  | VARCHAR(150) | The token to be blacklist     | True   | True     | True  |
| expires_at             | TIMESTAMP    | The token's expiration time   | False  | True     | False |

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
