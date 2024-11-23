# Quotes Tables
## Quote
| Field           | Type      | Description                  | UNIQUE | NOT NULL | INDEX |
|-----------------|-----------|------------------------------|--------|----------|-------|
| id              | INT       | The primary key              | True   | True     | True  |
| user_id         | INT       | User foreign key             | True   | True     | True  |
| quote           | TEXT      | The quote text               | False  | True     | False |
| author_id       | INT       | The author of the book       | False  | False    | True  |
| book_id         | INT       | The book in question         | False  | False    | True  |
| summary_id      | INT       | Summary the quote is from    | False  | False    | True  |
| chapter_id      | INT       | Chapter id the quote is from | False  | False    | False |
| first_character | INT       | Where the quote starts       | False  | False    | False |
| created_at      | TIMESTAMP | Creation time                | False  | True     | False |
| last_modified   | TIMESTAMP | Last modification time       | False  | False    | False |

```sql
CREATE TABLE IF NOT EXISTS quote (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id NOT NULL,
    quote TEXT
);
CREATE INDEX idx_quote_user on quote(user_id);
CREATE INDEX idx_quote_author on quote(author_id);
CREATE INDEX idx_quote_book on quote(book_id);
CREATE INDEX idx_quote_summary on quote(summary_id);
```
