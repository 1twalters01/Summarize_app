# Groups Tables
## Group (postgres)
| Field             | Type         | Description                | IS UNIQUE | NOT NULL | Index |
|-------------------|--------------|----------------------------|-----------|----------|-------|
| id                | INT          | group primary key          | True      | True     | True  |
| uuid              | UUID         | External identifier        | True      | True     | True  |
| group_name        | VARCHAR(100) | Name of the group          | False     | True     | True  |
| group_description | TEXT         | Group description          | False     | False    | False |
| created_at        | TIMESTAMP    | Group creation time        | False     | True     | False |
| created_by        | INT          | User that created group    | False     | True     | False |
| last active       | TIMESTAMP    | Last group activity        | False     | True     | False |

```sql
CREATE TABLE IF NOT EXISTS groups(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    uuid UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(),
    group_name VARCHAR(100) UNIQUE NOT NULL,
    group_description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by INT NOT NULL,
    last_active TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_groups_uuid ON groups (uuid);
CREATE INDEX idx_groups_name ON groups (group_name);
```

## group user relationship
| Field             | Type         | Description                | IS UNIQUE | NOT NULL | Index |
|-------------------|--------------|----------------------------|-----------|----------|-------|
| group_id          | INT          | group primary key          | False     | True     | True  |
| user_id           | INT          | user primary key           | False     | True     | True  |
| user_role         | Enum         | user role in the group     | False     | True     | False |

```sql
CREATE TABLE IF NOT EXISTS group_user_relationships(
    group_id INT NOT NULL,
    user_id INT NOT NULL,
    user_role role_enum NOT NULL,
    PRIMARY KEY(group_id, user_id)
);
CREATE INDEX idx_group_user_relations_group ON group_user_relationships (group_id);
CREATE INDEX idx_group_user_relations_user ON group_user_relationships (user_id);
CREATE INDEX idx_group_user_relations_role ON group_user_relationships (user_role);
```

## Group messages (scylla?)
| Field             | Type         | Description                | IS UNIQUE | NOT NULL | Index |
|-------------------|--------------|----------------------------|-----------|----------|-------|
| group_uuid        | UUID         | group identifier           | False     | True     | True  |
| message_id        | INT          | individual message id      | False     | True     | True  |
| sender_id         | UUID         | ID of user sending message | False     | True     | True  |
| body              | TEXT         | Body text of message       | False     | True     | True  |
| is_pinned         | BOOLEAN      | Is message pinned          | False     | True     | True  |
| sent_at           | TIMESTAMP    | When the message was sent  | False     | True     | True  |

```cql
```

## Group currently reading

## Group read history

## Group recommendations

## Group challenges
