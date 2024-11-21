# Messages Tables

## Scylla

### conversations
| Field              | Type         | Description                 | UNIQUE | NOT NULL | INDEX |
|--------------------|--------------|-----------------------------|--------|----------|-------|
| id | UUID/INT |
| name | TEXT |
| created_at | TIMESTAMP |
| participants | List<UUID> |
| is_group | BOOLEAN |
| last_message | TIMESTAMP |
| last_message_preview | TEXT |

CREATE TABLE conversations (
    conversation_id UUID PRIMARY KEY,
    name TEXT,
    created_at TIMESTAMP,
    participants LIST<UUID>,
    is_group BOOLEAN,
    last_message TIMESTAMP,
    last_message_preview TEXT
);

### messages
| Field              | Type         | Description                 | UNIQUE | NOT NULL | INDEX |
|--------------------|--------------|-----------------------------|--------|----------|-------|
| conversation_id | UUID |
| message_id | TIMEUUID |
| sender_id | UUID |
| body | TEXT |
| sent_at | TIMESTAMP |

CREATED TABLE messages (
    conversation_id UUID,
    message_id TIMEUUID,
    sender_id UUID,
    body TEXT,
    sent_at TIMESTAMP,
    PRIMARY KEY (conversation_id, message_id)
) WITH CLUSTERING ORDER BY (message_id ASC);

### Read Receipts
read_receipts
| Field              | Type         | Description                 | UNIQUE | NOT NULL | INDEX |
|--------------------|--------------|-----------------------------|--------|----------|-------|
| conversation_id | UUID |
| user_id | UUID |
| message_id | TIMEUUID |
| read_at | TIMESTAMP |

CREATED TABLE read_receipts (
    conversation_id UUID,
    user_id UUID,
    message_id TIMEUUID,
    read_at TIMESTAMP,
    PRIMARY KEY (conversation_id, user_id)
) WITH CLUSTERING ORDER BY (read_at ASC);

### latest_read_receipts
| Field              | Type         | Description                 | UNIQUE | NOT NULL | INDEX |
|--------------------|--------------|-----------------------------|--------|----------|-------|
| user_id | UUID |
| conversation_id | UUID |
| last_read_message_id | TIMEUUID |
| read_at | TIMESTAMP |

CREATED TABLE latest_read_receipts (
    user_id UUID,
    conversation_id UUID,
    last_read_message_id TIMEUUID,
    read_at TIMESTAMP,
    PRIMARY KEY (user_is, conversation_id)
) WITH CLUSTERING ORDER BY (read_at ASC);

### user_conversations
| Field              | Type         | Description                 | UNIQUE | NOT NULL | INDEX |
|--------------------|--------------|-----------------------------|--------|----------|-------|
| user_id | UUID |
| conversation_id | UUID |
| last_message | TIMESTAMP |
| last_message_preview | TEXT |

CREATE TABLE user_conversations (
    user_id UUID,
    conversation_id UUID,
    last_message TIMESTAMP,
    last_message_preview TEXT,
    PRIMARY KEY (user_id, conversation_id)
) WITH CLUSTERING ORDER BY (last_message DESC);

### Examples
Fetch latest messages in a conversation:
SELECT * FROM messages 
WHERE conversation_id = <conversation_id>
LIMIT 50;

Fetch all conversations for a user:
SELECT * FROM user_conversations 
WHERE user_id = <user_id>;

Update last read message:
 INSERT INTO read_receipts (conversation_id, user_id, message_id, read_at)
VALUES (<conversation_id>, <user_id>, <message_id>, NOW());
INSERT INTO latest_read_receipts (user_id, conversation_id, user_id, last_read_message, read_at)
    VALUES (<user_id>, <conversation_id>, <message_id>, NOW())
ON CONFLICT (user_id, conversation_id)
    DO UPDATE SET last_read_message = EXCLUDED.last_read_message, read_at = EXCLUDED.read_at;

Fetch unread messages:
SELECT * FROM messages WHERE conversation_id = <conversation_id>
  AND message_id > (
    SELECT last_read_message
    FROM latest_read_receipts
    WHERE user_id = <user_id> AND conversation_id = <conversation_id>
);

Count how many times a message was read:
SELECT message_id, COUNT(*) AS read_count
FROM read_receipts
WHERE conversation_id = <conversation_id>
GROUP BY message_id
ORDER BY read_count DESC;

Delete old read receipts:
DELETE FROM read_receipts
WHERE read_at < NOW() - INTERVAL '<Interval> days';

