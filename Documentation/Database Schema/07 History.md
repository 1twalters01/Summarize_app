# History Tables
## Redis


## Postgres
### Datatype
| Field           | Type           | Description                | UNIQUE | NOT NULL | INDEX |
|-----------------|----------------|----------------------------|--------|----------|
| id              | INT            | Primary key                | True   | True     |
| datatype        | VARCHAR(20)    | Datatype of entry          | True   | True     |

```sql
CREATE TYPE IF NOT EXISTS datatype_enum AS ENUM ('summary', 'book', 'author', 'publisher', 'library', 'user');
```

### History Entries
| Field           | Type           | Description                | UNIQUE | NOT NULL | INDEX |
|-----------------|----------------|----------------------------|--------|----------|-------|
| id              | INT            | Primary key of the history | True   | True     | True  |
| history_user_id | INT            | Foreign key to user        | False  | True     | True  |
| datatype        | Enum           | datatype enum              | False  | True     | False |
| time_added      | TIMESTAMP      | The time when added        | False  | True     | False |
| summary_id      | INT            | ID of item being added     | False  | False    | False |
| book_id         | INT            | ID of item being added     | False  | False    | False |
| author_id       | INT            | ID of item being added     | False  | False    | False |
| publisher_id    | INT            | ID of item being added     | False  | False    | False |
| library_id      | INT            | ID of item being added     | False  | False    | False |
| user_id         | INT            | ID of item being added     | False  | False    | False |

```sql
CREATE TABLE IF NOT EXISTS history (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    history_user_id INT NOT NULL,
    datatype datatype_enum NOT NULL,
    time_added TIMESTAMP NOT NULL DEFAULT NOW(),
    summary_id INT NOT NULL,
    book_id INT NOT NULL,
    author_id INT NOT NULL,
    publisher_id INT NOT NULL,
    library_id INT NOT NULL,
    user_id INT NOT NULL,
    CONSTRAINT fk_summaries FOREIGN KEY (summary_id)
        REFERENCES summaries (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
    CONSTRAINT fk_books FOREIGN KEY (book_id)
        REFERENCES books (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
    CONSTRAINT fk_authors FOREIGN KEY (author_id)
        REFERENCES authors (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
    CONSTRAINT fk_publishers FOREIGN KEY (publisher_id)
        REFERENCES publishers (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
    CONSTRAINT fk_libraries FOREIGN KEY (library_id)
        REFERENCES libraries (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_history_user_id ON user_history (user_id);
CREATE INDEX IF NOT EXISTS idx_history_datatype ON user_history (datatype);
CREATE INDEX IF NOT EXISTS idx_history_time_added ON user_history (time_added);
```

## Datalake
