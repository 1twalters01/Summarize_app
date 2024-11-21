# History Tables
## Redis


## Postgres
### Datatype
| Field           | Type           | Description                | UNIQUE | NOT NULL |
|-----------------|----------------|----------------------------|--------|----------|
| id              | INT            | Primary key                | True   | True     |
| datatype        | VARCHAR(20)    | Datatype of entry          | True   | True     |

CREATE TYPE datatype_enum AS ENUM ('summary', 'book', 'author', 'publisher', 'library', 'user');

### History Entries
| Field           | Type           | Description                | UNIQUE | NOT NULL |
|-----------------|----------------|----------------------------|--------|----------|
| id              | INT            | Primary key of the history | True   | True     |
| user_id         | INT            | Foreign key to user        | False  | True     |
| data_uuid       | UUID           | The uuid of the entry      | True   | True     |
| datatype        | Enum           | datatype enum              | False  | True     |
| time_added      | TIMESTAMP      | The time when added        | False  | True     |

CREATE TABLE IF NOT EXISTS history (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    data_uuid UUID UNIQUE NOT NULL,
    datatype datatype_enum NOT NULL,
    time_added TIMESTAMP NOT NULL DEFAULT NOW(),
);
CREATE INDEX idx_history_user_id ON user_history (user_id);
CREATE INDEX idx_history_datatype ON user_history (datatype);
CREATE INDEX idx_history_time_added ON user_history (time_added);

## Datalake
