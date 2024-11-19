# Summaries Tables
## Summaries
| Field           | Type           | Description                | IS UNIQUE | NOT NULL | INDEX |
|-----------------|----------------|----------------------------|-----------|----------|-------|
| id              | INT            | The summary's primary key  | True      | True     | True  |
| book_id         | INT            | Foreign key to the book    | False     | True     | True  |
| user_id         | INT            | Summary author foreign key | False     | True     | True  |
| no_of_chapters  | INT            | Number of chapters         | False     | True     | False |
| chapter_links   | TEXT[]         | Links to chapters          | True      | True     | False |
| Likes           | INT            | Number of likes            | False     | True     | False |
| Dislikes        | INT            | Number of dislikes         | False     | True     | False |
| Saves           | INT            | Times added to a library   | False     | True     | False |
| Shares          | INT            | Times shared in a group    | False     | True     | False |
