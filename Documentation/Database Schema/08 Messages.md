# Messages Tables (Scylla)
### chat
| Field                | Type         | Description                 | UNIQUE | NOT NULL | INDEX |
|----------------------|--------------|-----------------------------|--------|----------|-------|
| chat_id              | UUID         | Chats primary key           | True   | True     | True  |
| name                 | TEXT         | Name of the chat            | False  | False    | True  |
| created_at           | TIMESTAMP    | Creation datetime           | False  | True     | True  |
| participants         | List<UUID>   | Participants in the chat    | False  | True     | False |
| is_group             | BOOLEAN      | Group chat or individuals   | False  | True     | False |
| last_message_time    | TIMESTAMP    | Last message sent time      | False  | True     | False |
| last_message_id      | TIMEUUID     | Identifier for last message | False  | True     | True  |
| last_message_preview | TEXT         | Last chat message preview   | False  | False    | False |

```cql
CREATE TABLE chat (
    chat_id UUID PRIMARY KEY,
    name TEXT,
    created_at TIMESTAMP,
    participants LIST<UUID>,
    is_group BOOLEAN,
    last_message_time TIMESTAMP,
    last_message_id TIMEUUID,
    last_message_preview TEXT
);
```

### messages
| Field                | Type         | Description                 | UNIQUE | NOT NULL | INDEX |
|----------------------|--------------|-----------------------------|--------|----------|-------|
| chat_id              | UUID         | Identifier for chat         | True   | True     | True  |
| message_id           | TIMEUUID     | individual message id       | True   | False    | True  |
| sender_id            | UUID         | ID of user sending message  | False  | True     | True  |
| body                 | TEXT         | Body text of message        | False  | True     | True  |
| sent_at              | TIMESTAMP    | When the message was sent   | False  | True     | True  |

```cql
CREATED TABLE messages (
    chat_id UUID,
    message_id TIMEUUID,
    sender_id UUID,
    body TEXT,
    sent_at TIMESTAMP,
    PRIMARY KEY (chat_id, message_id)
) WITH CLUSTERING ORDER BY (message_id ASC);
```

### Read Receipts
read_receipts
| Field                | Type         | Description                 | UNIQUE | NOT NULL | INDEX |
|----------------------|--------------|-----------------------------|--------|----------|-------|
| chat_id              | UUID         | Identifier for chat         | False  | True     | True  |
| user_id              | UUID         | Identifier for user         | False  | True     | True  |
| message_id           | TIMEUUID     | Identifier for message      | False  | True     | True  |
| read_at              | TIMESTAMP    | Read time                   | False  | True     | True  |

```cql
CREATED TABLE read_receipts (
    chat_id UUID,
    user_id UUID,
    message_id TIMEUUID,
    read_at TIMESTAMP,
    PRIMARY KEY (chat_id, user_id)
) WITH CLUSTERING ORDER BY (read_at ASC);
```

### latest_read_receipts
| Field                | Type         | Description                 | UNIQUE | NOT NULL | INDEX |
|----------------------|--------------|-----------------------------|--------|----------|-------|
| user_id              | UUID         | Identifier for user         | False  | True     | True  |
| chat_id              | UUID         | Identifier for chat         | False  | True     | True  |
| last_read_message_id | TIMEUUID     | Last read message id        | False  | True     | True  |
| read_at              | TIMESTAMP    | Read time                   | False  | True     | True  |

```cql
CREATED TABLE latest_read_receipts (
    user_id UUID,
    chat_id UUID,
    last_read_message_id TIMEUUID,
    read_at TIMESTAMP,
    PRIMARY KEY (user_is, chat_id)
) WITH CLUSTERING ORDER BY (read_at ASC);
```

### user_chats
| Field                | Type         | Description                 | UNIQUE | NOT NULL | INDEX |
|----------------------|--------------|-----------------------------|--------|----------|-------|
| user_id              | UUID         | Identifier for user         | False  | True     | True  |
| chat_id              | UUID         | Identifier for chat         | False  | True     | True  |
| last_message_time    | TIMESTAMP    | Last message sent time      | False  | True     | False |
| last_message_id      | TIMEUUID     | Identifier for last message | False  | True     | True  |
| last_message_preview | TEXT         | Last chat message preview   | False  | False    | False |

```cql
CREATE TABLE user_chats (
    user_id UUID,
    chat_id UUID,
    last_message TIMESTAMP,
    last_message_id TIMEUUID,
    last_message_preview TEXT,
    PRIMARY KEY (user_id, chat_id)
) WITH CLUSTERING ORDER BY (last_message DESC);
```

### Examples
Fetch latest messages in a chat:
```cql
SELECT * FROM messages 
WHERE chat_id = <chat_id>
LIMIT 50;
```

Fetch all chats for a user:
```cql
SELECT * FROM user_chats 
WHERE user_id = <user_id>;
```
Update last read message:
```cql
 INSERT INTO read_receipts (chat_id, user_id, message_id, read_at)
VALUES (<chat_id>, <user_id>, <message_id>, NOW());
INSERT INTO latest_read_receipts (user_id, chat_id, user_id, last_read_message, read_at)
    VALUES (<user_id>, <chat_id>, <message_id>, NOW())
ON CONFLICT (user_id, chat_id)
    DO UPDATE SET last_read_message = EXCLUDED.last_read_message, read_at = EXCLUDED.read_at;
```

Fetch unread messages:
```cql
SELECT * FROM messages WHERE chat_id = <chat_id>
  AND message_id > (
    SELECT last_read_message
    FROM latest_read_receipts
    WHERE user_id = <user_id> AND chat_id = <chat_id>
);
```

Count how many times a message was read:
```cql
SELECT message_id, COUNT(*) AS read_count
FROM read_receipts
WHERE chat_id = <chat_id>
GROUP BY message_id
ORDER BY read_count DESC;
```

Delete old read receipts:
```cql
DELETE FROM read_receipts
WHERE read_at < NOW() - INTERVAL '<Interval> days';
```
