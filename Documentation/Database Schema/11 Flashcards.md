# Flashcards Tables
## Decks
| Field           | Type           | Description                | IS UNIQUE | NOT NULL |
|-----------------|----------------|----------------------------|-----------|----------|
| id              | INT        |
| user_id         | INT        |
| summary_id      | INT        |
| created_at      | TIMESTAMP  |
| created_by      | ID         |
| last_modified   | TIMESTAMP  |
| editor_ids[]    | ID         |
|

## Cards
| Field           | Type           | Description                | IS UNIQUE | NOT NULL |
|-----------------|----------------|----------------------------|-----------|----------|
| id              | INT            | 
| deck_id         | INT            |
| Place           | INT            |
| Front (change this)
| Back (change this)
|