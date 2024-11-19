# Quotes Tables
## Quote
| Field           | Type  | Description                | IS UNIQUE | NOT NULL | INDEX |
|-----------------|-------|----------------------------|-----------|----------|-------|
| id              | INT   | The primary key            |
| user_id         | INT   | User foreign key           |
| Summary_id      | INT   | Summary foreign key        |
| Quote           | TEXT  | The quote text             |
