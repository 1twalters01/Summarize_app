# Flashcards Tables
## Decks
| Field           | Type         | Description                   | UNIQUE | NOT NULL | Index |
|-----------------|--------------|-------------------------------|--------|----------|-------|
| id              | INT          | Deck primary key              | True   | True     | True  |
| user_id         | INT          | Owner of the deck's id        | False  | True     | True  |
| summary_id      | INT          | ID of associated summary      | True   | False    | True  |
| book_id         | INT          | ID of associated book         | False  | False    | True  |
| deck_name       | VARCHAR(100) | Name of the deck              | False  | True     | True  |
| description     | TEXT         | Deck description              | False  | False    | False |
| created_at      | TIMESTAMP    | The deck's creation time      | False  | True     | False |
| last_modified   | TIMESTAMP    | Last modification time        | False  | True     | False |
| is_public       | BOOLEAN      | Is flashcard public?          | False  | True     | True  |

```sql
CREATE TABLE IF NOT EXISTS decks (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    summary_id INT UNIQUE,
    book_id INT NOT NULL,
    deck_name VARCHAR(100) NOT NULL,
    description TEXT,
    created_at TIMESTAMP NOT NULL,
    last_modified TIMESTAMP NOT NULL,
    is_public NOT NULL,
);
CREATE INDEX IF NOT EXISTS idx_deck_users on deck (user_id);
CREATE INDEX IF NOT EXISTS idx_deck_summary on deck (summary_id);
CREATE INDEX IF NOT EXISTS idx_deck_book on deck (book_id);
CREATE INDEX IF NOT EXISTS idx_deck_name on deck (deck_name);
CREATE INDEX IF NOT EXISTS idx_deck_is_public on deck (is_public);
```

## Tags
| Field           | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|-----------------|--------------|-------------------------------|--------|----------|-------|
| id              | INT          | Primary key                   | True   | True     | True  |
| tag_name        | VARCHAR(30)  | Name of the tag               | True   | True     | True  |

```sql
CREATE TABLE IF NOT EXISTS tags (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    tag_name VARCHAR(30) UNIQUE NOT NULL
);
```

## Deck Tags
| Field           | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|-----------------|--------------|-------------------------------|--------|----------|-------|
| deck_id         | INT          | Foreign key to deck           | False  | True     | True  |
| tag_id          | INT          | Foreign key to tag            | False  | True     | True  |

```sql
CREATE TABLE IF NOT EXISTS deck_tag_relations (
    deck_id INT NOT NULL,
    tag_id INT NOT NULL,
    CONSTRAINT fk_deck FOREIGN KEY (deck_id)
        REFERENCES decks (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_tag FOREIGN KEY (tag_id)
        REFERENCES tags (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    Unique (deck_id, tag_id);
);
CREATE INDEX IF NOT EXISTS idx_deck_tags ON deck_tags (deck_id, tag_id);
```

## Editors
| Field           | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|-----------------|--------------|-------------------------------|--------|----------|-------|
| deck_id         | INT          | Foreign key to deck           | False  | True     | True  |
| user_id         | INT          | Foreign key to user           | False  | True     | True  |

```sql
CREATE TABL IF NOT EXISTSE deck_editors (
    deck_id INT NOT NULL,
    user_id INT NOT NULL,
    CONSTRAINT fk_deck FOREIGN KEY (deck_id)
        REFERENCES decks (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_editor FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    UNIQUE (deck_id, user_id)
);
CREATE INDEX IF NOT EXISTS idx_deck_editors ON deck_editors (deck_id, user_id);
```

## Viewers
| Field           | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|-----------------|--------------|-------------------------------|--------|----------|-------|
| deck_id         | INT          | Foreign key to deck           | False  | True     | True  |
| user_id         | INT          | Foreign key to user           | False  | True     | True  |

```sql
CREATE TABLE IF NOT EXISTS deck_viewers (
    deck_id INT NOT NULL,
    user_id INT NOT NULL,
    CONSTRAINT fk_deck FOREIGN KEY (deck_id)
        REFERENCES libraries (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_viewer FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    UNIQUE (deck_id, user_id)
);
CREATE INDEX IF NOT EXISTS idx_deck_viewers ON deck_viewers (deck_id, user_id);
```

## Cards
| Field           | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|-----------------|--------------|-------------------------------|--------|----------|-------|
| id              | INT          | Card primary key              | True   | True     | True  |
| deck_id         | INT          | The deck the card is in       | False  | True     | True  |
| place           | INT          | Place in deck                 | False  | True     | False |
| question        | TEXT         | The front of the card         | False  | True     | False |
| answer          | TEXT         | The back of the card          | False  | True     | False |
| created_at      | TIMESTAMP    | The deck's creation time      | False  | True     | False |
| last_modified   | TIMESTAMP    | Last modification time        | False  | True     | False |

```sql
CREATE TABLE IF NOT EXISTS cards (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    deck_id INT NOT NULL,
    place INT NOT NULL,
    question TEXT NOT NULL,
    answer TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    last_modified TIMESTAMP NOT NULL,
    UNIQUE (deck_id, place)
);
CREATE INDEX IF NOT EXISTS idx_card_deck ON cards (deck_id);
```

## Progress
| Field           | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|-----------------|--------------|-------------------------------|--------|----------|-------|
| id              | INT          | Primary key                   | True   | True     | True  |
| user_id         | INT          | The user's progress           | False  | True     | True  |
| card_id         | INT          | Associated card               | False  | True     | True  |
| review_count    | INT          | Number of times reviewed      | False  | True     | True  |
| last_reviewed   | TIMESTAMP    | Last review time              | False  | False    | False |
| ease_factor     | FLOAT        | Spaced repetition ease factor | False  | True     | False |

```sql
CREATE TABLE IF NOT EXISTS progress (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    card_id INT NOT NULL,
    review_count INT NOT NULL,
    last_review TIMESTAMP,
    ease_factor FLOAT NOT NULL,
    Unique (user_id, card_id)
);
CREATE INDEX IF NOT EXISTS idx_progress_user ON progress (user_id);
CREATE INDEX IF NOT EXISTS idx_progress_card ON progress (card_id);
CREATE INDEX IF NOT EXISTS idx_progress_review_count ON progress (review_count);
```
