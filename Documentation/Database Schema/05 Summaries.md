# Summaries Tables
Summaries
| Field           | Type           | Description                | IS UNIQUE | NOT NULL |
|-----------------|----------------|----------------------------|-----------|----------|
| id              | INT            | The summary's primary key  | True      | True     |
| book_id         | INT            | Foreign key to the book    | False     | True     |
| user_id         | INT            | Summary author foreign key | False     | True     |
| no_of_chapters  | INT            | Number of chapters         | False     | True     |
| chapter_links   | TEXT[]         | Links to chapters          | True      | True     |
| Likes           | INT            | Number of likes            | False     | True     |
| Dislikes        | INT            | Number of dislikes         | False     | True     |
| Saves           | INT            | Times added to a library   | False     | True     |
| Shares          | INT            | Times shared in a group    | False     | True     |
