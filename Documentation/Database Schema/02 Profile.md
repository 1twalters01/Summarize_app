## Languages
| Field                     | Type         | Description                    | UNIQUE | NOT NULL | INDEX |
|---------------------------|--------------|--------------------------------|--------|----------|-------|
| id                        | INT          | Primary key (internal)         | True   | True     | True  |
| language                  | VARCHAR(30)  | The language in question       | True   | True     | True  |

```sql
CREATE TABLE IF NOT EXISTS languages (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    language VARCHAR(20) UNIQUE NOT NULL
);
```


## User Languages
| Field                 | Type         | Description                  | UNIQUE | NOT NULL | INDEX |
|-----------------------|--------------|------------------------------|--------|----------|-------|
| id                    | INT          | Primary key (internal)       | True   | True     | True  |
| user_id               | INT          | Foreign key to user          | False  | True     | False |
| language_id           | INT          | Foreign key to language      | False  | True     | False |

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

## Preferred Language
| Field                 | Type         | Description                  | UNIQUE | NOT NULL | INDEX |
|-----------------------|--------------|------------------------------|--------|----------|-------|
| user_id               | INT          | Foreign key to user id       | True   | True     | False |
| language_id           | INT          | The preferred language       | False  | True     | True  |

```sql
CREATE TABLE IF NOT EXISTS preferred_languages (
    user_id INT Primary KEY,
    language_id INT,
    CONSTRAINT fk_languages FOREIGN KEY (language_id)
        REFERENCES languages (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
)
```

## Default Themes
| Field                 | Type         | Description                  | UNIQUE | NOT NULL | INDEX |
|-----------------------|--------------|------------------------------|--------|----------|-------|
| id                    | INT          | Primary key (internal)       | True   | True     | True  |
| theme_name            | VARCHAR(20)  | Name of the theme            | True   | True     | True  |
| colour1               | INT          | Colour 1 saved as rgb        | False  | True     | False |
| colour2               | INT          | Colour 2 saved as rgb        | False  | True     | False |
| colour3               | INT          | Colour 3 saved as rgb        | False  | True     | False |
| colour4               | INT          | Colour 4 saved as rgb        | False  | True     | False |
| colour5               | INT          | Colour 5 saved as rgb        | False  | True     | False |
| colour6               | INT          | Colour 6 saved as rgb        | False  | True     | False |

```sql
CREATE TABLE IF NOT EXISTS default_themes (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    theme_name VARCHAR(20) UNIQUE NOT NULL,
    colour1 INT NOT NULL,
    colour2 INT NOT NULL,
    colour3 INT NOT NULL,
    colour4 INT NOT NULL,
    colour5 INT NOT NULL,
    colour6 INT NOT NULL,
    CHECK (colour1 BETWEEN 0 AND 16777215)
    CHECK (colour2 BETWEEN 0 AND 16777215)
    CHECK (colour3 BETWEEN 0 AND 16777215)
    CHECK (colour4 BETWEEN 0 AND 16777215)
    CHECK (colour5 BETWEEN 0 AND 16777215)
    CHECK (colour6 BETWEEN 0 AND 16777215)
)
```

https://stackoverflow.com/questions/26059228/css-hsl-or-rgba-colors

Save colours as (r (0 to 255) << 16) + (g (0 to 255) << 8) + (b (0 to 255))
e.g rgb: 50, 90, 190 -> 50 << 16 + 90 << 8, 190 = 50 * 2^16 + 90 * 2^8 + 190 = 3,300,030
Largest number is: 255*2^16 + 255*2^8 + 255 = 16,777,215 -> Need a u32

## Custom Themes
| Field                 | Type         | Description                  | UNIQUE | NOT NULL | INDEX |
|-----------------------|--------------|------------------------------|--------|----------|-------|
| id                    | INT          | Primary key (internal)       | True   | True     | True  |
| user_id               | INT          | Foreign key to user id       | True   | True     | False |
| theme_name            | VARCHAR(20)  | Name of the theme            | True   | True     | True  |
| colour1               | INT          | Colour 1 saved as rgb        | False  | True     | False |
| colour2               | INT          | Colour 2 saved as rgb        | False  | True     | False |
| colour3               | INT          | Colour 3 saved as rgb        | False  | True     | False |
| colour4               | INT          | Colour 4 saved as rgb        | False  | True     | False |
| colour5               | INT          | Colour 5 saved as rgb        | False  | True     | False |
| colour6               | INT          | Colour 6 saved as rgb        | False  | True     | False |

```sql
CREATE TABLE IF NOT EXISTS custom_themes (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT,
    colour1 INT NOT NULL,
    colour2 INT NOT NULL,
    colour3 INT NOT NULL,
    colour4 INT NOT NULL,
    colour5 INT NOT NULL,
    colour6 INT NOT NULL,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
    CHECK (colour1 BETWEEN 0 AND 16777215)
    CHECK (colour2 BETWEEN 0 AND 16777215)
    CHECK (colour3 BETWEEN 0 AND 16777215)
    CHECK (colour4 BETWEEN 0 AND 16777215)
    CHECK (colour5 BETWEEN 0 AND 16777215)
    CHECK (colour6 BETWEEN 0 AND 16777215)
)
```

Same rgb format

## Default Theme
| Field                 | Type         | Description                  | UNIQUE | NOT NULL | INDEX |
|-----------------------|--------------|------------------------------|--------|----------|-------|
| user_id               | INT          | Foreign key to user id       | True   | True     | False |
| last_default_theme_id | INT          | ID of the last default theme | False  | False    | False |
| use_default_theme     | BOOLEAN      | Use the default theme?       | False  | True     | False |

```sql
CREATE TABLE IF NOT EXISTS default_themes (
    user_id INT PRIMARY KEY,
    last_default_theme_id INT,
    use_default_theme BOOLEAN,
)
```

## Device Themes
| Field                 | Type         | Description                  | UNIQUE | NOT NULL | INDEX |
|-----------------------|--------------|------------------------------|--------|----------|-------|
| device_user_link_id   | INT          | FK to device user link       | True   | True     | False |
| last_default_theme_id | INT          | ID of the last default theme | False  | False    | False |
| use_default_theme     | BOOLEAN      | Use the default theme?       | False  | True     | False |

```sql
CREATE TABLE IF NOT EXISTS device_themes (
    device_user_link_id INT PRIMARY KEY,
    last_default_theme_id INT,
    use_default_theme BOOLEAN,
)
```

Overides default theme if here