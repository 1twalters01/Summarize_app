## Languages
| Field                  | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|------------------------|--------------|-------------------------------|--------|----------|-------|
| id                     | INT          | Primary key (internal)        | True   | True     | True  |
| language               | VARCHAR(30)  | The language in question      | True   | True     | True  |

```sql
CREATE TABLE IF NOT EXISTS languages (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    language VARCHAR(20) UNIQUE NOT NULL
);
```


## User Languages
| Field              | Type         | Description                 | UNIQUE | NOT NULL | INDEX |
|--------------------|--------------|-----------------------------|--------|----------|-------|
| id                 | INT          | Primary key (internal)      | True   | True     | True  |
| user_id            | INT          | Foreign key to user id      | False  | True     | False |
| language_id

```sql
CREATE TABLE IF NOT EXISTS user_languages (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT,
    language_id INT NOT NULL,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
    CONSTRAINT fk_languages FOREIGN KEY (language_id)
        REFERENCES languages (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
    CONSTRAINT unique_user_language UNIQUE (user_id, language);
);
```

## Colour


## Default Themes
| Field              | Type         | Description                 | UNIQUE | NOT NULL | INDEX |
|--------------------|--------------|-----------------------------|--------|----------|-------|
| id                 | INT          | Primary key (internal)      | True   | True     | True  |
| theme              | VARCHAR(10)  | Name of the theme           | True   | True     | True  |

```sql
```

## Theme
| Field              | Type         | Description                 | UNIQUE | NOT NULL | INDEX |
|--------------------|--------------|-----------------------------|--------|----------|-------|

```sql
```
