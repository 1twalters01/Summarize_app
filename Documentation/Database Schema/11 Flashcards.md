# Flashcards Tables
## Decks
| Field           | Type           | Description                | IS UNIQUE | NOT NULL |
|-----------------|----------------|----------------------------|-----------|----------|
| id              | INT
| user_id         | INT
| summary_id      | INT
| created_at      | TIMESTAMP
| created_by      | ID
| last_modified   | TIMESTAMP
| editors[]
|

## Cards
| Field           | Type           | Description                | IS UNIQUE | NOT NULL |
|-----------------|----------------|----------------------------|-----------|----------|
| id
| deck_id
| Place
| Front (change this)
| Back (change this)
|