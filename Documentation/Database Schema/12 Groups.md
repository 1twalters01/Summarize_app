# Groups Tables
## Group (postgres)
| Field             | Type         | Description                 | UNIQUE | NOT NULL | Index |
|-------------------|--------------|-----------------------------|--------|----------|-------|
| id                | INT          | group primary key           | True   | True     | True  |
| uuid              | UUID         | External identifier         | True   | True     | True  |
| group_name        | VARCHAR(100) | Name of the group           | False  | True     | True  |
| group_description | TEXT         | Group description           | False  | False    | False |
| created_at        | TIMESTAMP    | Group creation time         | False  | True     | False |
| created_by        | INT          | User that created group     | False  | True     | False |
| last active       | TIMESTAMP    | Last group activity         | False  | True     | False |

```sql
CREATE TABLE IF NOT EXISTS groups (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    uuid UUID UNIQUE NOT NULL DEFAULT gen_random_uuid(),
    group_name VARCHAR(100) UNIQUE NOT NULL,
    group_description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    created_by INT NOT NULL,
    last_active TIMESTAMP NOT NULL DEFAULT NOW()
);
CREATE INDEX IF NOT EXISTS idx_groups_uuid ON groups (uuid);
CREATE INDEX IF NOT EXISTS idx_groups_name ON groups (group_name);
```

## group user relationship
| Field             | Type         | Description                 | UNIQUE | NOT NULL | Index |
|-------------------|--------------|-----------------------------|--------|----------|-------|
| group_id          | INT          | group primary key           | False  | True     | True  |
| user_id           | INT          | user primary key            | False  | True     | True  |
| user_role         | Enum         | user role in the group      | False  | True     | False |

```sql
CREATE TABLE IF NOT EXISTS group_user_relationships (
    group_id INT NOT NULL,
    user_id INT NOT NULL,
    user_role role_enum NOT NULL,
    PRIMARY KEY(group_id, user_id)
);
CREATE INDEX IF NOT EXISTS idx_group_user_relations_group ON group_user_relationships (group_id);
CREATE INDEX IF NOT EXISTS idx_group_user_relations_user ON group_user_relationships (user_id);
CREATE INDEX IF NOT EXISTS idx_group_user_relations_role ON group_user_relationships (user_role);
```

## Group messages (scylla?)
| Field             | Type         | Description                 | UNIQUE | NOT NULL | Index |
|-------------------|--------------|-----------------------------|--------|----------|-------|
| message_uuid      | UUID         | Primary key                 | False  | True     | True  |
| group_uuid        | UUID         | group identifier            | False  | True     | True  |
| sender_uuid       | UUID         | ID of user sending message  | False  | True     | True  |
| body              | TEXT         | Body text of message        | False  | True     | True  |
| is_pinned         | BOOLEAN      | Is message pinned           | False  | True     | True  |
| sent_at           | TIMESTAMP    | When the message was sent   | False  | True     | True  |

```cql
CREATE TABLE group_messages (
    message_uuid UUID PRIMARY KEY,
    group_uuid UUID,
    sender_uuid UUID
    body TEXT,
    is_pinned BOOlEAN,
    sent_at TIMESTAMP,
) WITH CLUSTERING ORDER BY (sent_at ASC);
```

## Group currently reading
| Field             | Type         | Description                 | UNIQUE | NOT NULL | Index |
|-------------------|--------------|-----------------------------|--------|----------|-------|
| id                | INT          | Primary key                 | True   | True     | True  |
| group_id          | INT          | The group's fk              | False  | True     | True  |
| summary_id        | INT          | The summary being read      | False  | False    | True  |
| book_id           | INT          | The book being read         | False  | True     | True  |
| last_modified     | TIMESTAMP    | Last modification time      | False  | True     | False |
| read_description  | TEXT         | Book/summary description    | False  | False    | False |

One of summary_id or book_id must not be Null
```sql
CREATE TABLE IF NOT EXISTS group_currently_reading (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    group_id INT NOT NULL,
    summary_id INT,
    book_id INT NOT NULL,
    last_modified TIMESTAMP NOT NULL DEFAULT NOW(),
    read_description TEXT,
    CONSTRAINT chk_summary_or_book CHECK (
        (summary_id IS NOT NULL AND book_id IS NULL) OR 
        (summary_id IS NULL AND book_id IS NOT NULL)
    )
);
CREATE INDEX IF NOT EXISTS idx_group_currently_reading_group ON group_currently_reading (group_id);
CREATE INDEX IF NOT EXISTS idx_group_currently_reading_summary ON group_currently_reading (summary_id);
CREATE INDEX IF NOT EXISTS idx_group_currently_reading_book ON group_currently_reading (book_id);
```

## Group read history
| Field             | Type         | Description                 | UNIQUE | NOT NULL | Index |
|-------------------|--------------|-----------------------------|--------|----------|-------|
| id                | INT          | Primary key                 | True   | True     | True  |
| group_id          | INT          | The group in question's id  | False  | True     | True  |
| summary_id        | INT          | The summary that was read   | False  | False    | True  |
| book_id           | INT          | The book that was read      | False  | True     | True  |
| added_at          | TIMESTAMP    | Time the history was added  | False  | True     | False |
| completed_at      | TIMESTAMP    | Time of completion          | False  | False    | False |
| was_deleted       | BOOLEAN      | If item was deleted         | False  | True     | False |
| read_description  | TEXT         | Book/summary description    | False  | False    | False |

```sql
CREATE TABLE IF NOT EXISTS group_read_history (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    group_id INT NOT NULL,
    summary_id INT,
    book_id INT NOT NULL,
    added_at TIMESTAMP NOT NULL
    completed_at TIMESTAMP,
    was_deleted BOOLEAN NOT NULL,
    read_description TEXT,
);
CREATE INDEX IF NOT EXISTS idx_group_read_history_group ON group_read_history (group_id);
CREATE INDEX IF NOT EXISTS idx_group_read_history_summary ON group_read_history (summary_id);
CREATE INDEX IF NOT EXISTS idx_group_read_history_book ON group_read_history (book_id);
```

## Group Progress
| Field             | Type         | Description                 | UNIQUE | NOT NULL | Index |
|-------------------|--------------|-----------------------------|--------|----------|-------|
| id                | INT          | Progress key                | True   | True     | True  |
| group_history_id  | INT          | Group reading history fk    | False  | True     | True  |
| user_id           | INT          | The user's id               | False  | True     | True  |
| quiz_finish_count | INT          | Times a quiz was finished   | False  | True     | False |
| deck_cards_viewed | INT          | Number of cards viewed      | False  | True     | False |
| deck_card_count   | INT          | Number of cards in deck     | False  | True     | False |

```sql
CREATE TABLE IF NOT EXISTS group_progress (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    group_history_id NOT NULL,
    user_id INT NOT NULL,
    quiz_finish_count INT NOT NULL,
    deck_cards_view INT NOT NULL,
    deck_card_count INT NOT NULL,
);
CREATE INDEX IF NOT EXISTS idx_group_progress_history_id ON group_reading_id (group_history_id);
CREATE INDEX IF NOT EXISTS idx_group_progress_users ON group_reading_id (user_id);
```

## Group recommendations
| Field             | Type         | Description                 | UNIQUE | NOT NULL | Index |
|-------------------|--------------|-----------------------------|--------|----------|-------|
| group_id          | INT          | The group in question's id  | False  | True     | True  |
| user_id           | INT          | The recommendee's id        | False  | True     | True  |
| book_id           | INT          | The book being suggested    | False  | True     | True  |
| summary_id        | INT          | The summary being suggested | False  | False    | True  |

```sql
CREATE TABLE IF NOT EXISTS group_recommendations (

);
CREATE INDEX IF NOT EXISTS idx_group_recommendations_group ON group_recommendations (group_id);
CREATE INDEX IF NOT EXISTS idx_group_recommendations_user ON group_recommendations (user_id);
CREATE INDEX IF NOT EXISTS idx_group_recommendations_book ON group_recommendations (book_id);
CREATE INDEX IF NOT EXISTS idx_group_recommendations_summary ON group_recommendations (summary_id);
```