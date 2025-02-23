# Summaries Tables
## Summaries
| Field           | Type           | Description                | UNIQUE | NOT NULL | INDEX |
|-----------------|----------------|----------------------------|--------|----------|-------|
| id              | INT            | The summary's primary key  | True   | True     | True  |
| book_id         | INT            | Foreign key to the book    | False  | True     | True  |
| user_id         | INT            | Summary author foreign key | False  | True     | True  |
| no_of_chapters  | INT            | Number of chapters         | False  | True     | False |
| Likes           | INT            | Number of likes            | False  | True     | False |
| Dislikes        | INT            | Number of dislikes         | False  | True     | False |
| Saves           | INT            | Times added to a library   | False  | True     | False |
| Shares          | INT            | Times shared in a group    | False  | True     | False |

```sql
CREATE TABLE IF NOT EXISTS summaries (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    book_id INT NOT NULL,
    user_id INT NOT NULL,
    no_of_chapters INT NOT NULL,
    likes INT NOT NULL,
    dislikes INT NOT NULL,
    saves INT NOT NULL,
    shares INT NOT NULL,
    CONSTRAINT fk_books FOREIGN KEY (book_id)
        REFERENCES books (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_summaries_book_id ON summaries (book_id);
CREATE INDEX IF NOT EXISTS idx_summaries_user_id ON summaries (user_id);
```

## Chapter links
| Field           | Type           | Description                | UNIQUE | NOT NULL | INDEX |
|-----------------|----------------|----------------------------|--------|----------|-------|
| id              | INT            | Primary key                | True   | True     | False |
| summary_id      | INT            | Summary id foreign key     | False  | True     | True  |
| chapter_number  | INT            | Chapter number             | False  | True     | True  |
| chapter_link    | Text           | Link to chapter            | True   | True     | False |

```sql
CREATE TABLE IF NOT EXISTS chapter_links (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    summary_id INT NOT NULL,
    chapter_number INT NOT NULL,
    chapter_link TEXT UNIQUE NOT NULL,
);
CREATE INDEX IF NOT EXISTS idx_chapter_links_summary_id ON chapter_links (summary_id);
CREATE INDEX IF NOT EXISTS idx_chapter_links_chapter_number ON chapter_links (chapter_number);
```
