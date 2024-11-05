# Users Tables
User
| Field           | Type           | Description                | IS UNIQUE | NOT NULL | INDEX  |
|-----------------|----------------|----------------------------|-----------|----------|--------|
| user_id         | INT            | Primary key (internal)     | True      | True     | True   |
| uuid            | UUID           | External identifier        | True      | True     | True   |
| username        | VARCHAR(50)    | The user’s username        | True      | True     |  |
| email           | VARCHAR(255)   | The user’s email           | True      | True     |  |
| first_name      | VARCHAR(20)    | The user's first name      | False     | False    |  |
| last_name       | VARCHAR(20)    | The user's last name       | False     | False    |  |
| created_at      | TIMESTAMP      | The user's creation time   | False     | True     |  |
| last_login      | TIMESTAMP      | The user's last login      | False     | False    |  |
| is_author       | BOOLEAN        | Is the user an author      | False     | True     |  |
| is_staff        | BOOLEAN        | Is the user staff          | False     | True     |  |
| is_superuser    | BOOLEAN        | Is the user a superuser    | False     | True     |  |

Password Hash
| Field           | Type           | Description                | IS UNIQUE | NOT NULL | INDEX  |
|-----------------|----------------|----------------------------|-----------|----------|--------|
| user_id         | INT            | Foreign key to user id     | True      | True     |  |
| password_hash   | VARCHAR(255)   | The user's password hash   | False     | True     |  |
| previous_hashes | VARCHAR(255)[] | Previous pasword hashes    | False     | True     |  |

Totp
| Field           | Type           | Description                | IS UNIQUE | NOT NULL | INDEX  |
|-----------------|----------------|----------------------------|-----------|----------|--------|
| user_id         | INT            | Foreign key to user id     | True      | True     |  |
| totp_key        | VARCHAR(50)    | The totp key               | False     | False    |  |
| last_updated    | TIMESTAMP      | Time of last totp update   | False     | False    |  |
| is_verified     | BOOLEAN        | Is the totp verified?      | False     | True     |  |
| verified_at     | TIMESTAMP      | Totp key verification time | False     | False    |  |
