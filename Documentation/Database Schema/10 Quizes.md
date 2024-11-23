# Quizes Tables
## Quiz
| Field           | Type         | Description                   | UNIQUE | NOT NULL | Index |
|-----------------|--------------|-------------------------------|--------|----------|-------|
| id              | INT          | Quiz primary key              | True   | True     | True  |
| user_id         | INT          | Owner of the quiz's id        | False  | True     | True  |
| summary_id      | INT          | ID of associated summary      | True   | False    | True  |
| book_id         | INT          | ID of associated book         | False  | False    | True  |
| description     | TEXT         | Quiz description              | False  | False    | False |
| created_at      | TIMESTAMP    | The quiz's creation time      | False  | True     | False |
| last_modified   | TIMESTAMP    | Last modification time        | False  | True     | False |
| is_public       | BOOLEAN      | Is quiz public?               | False  | True     | True  |

```sql
CREATE TABLE quiz (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
)
```

## Editors
| Field           | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|-----------------|--------------|-------------------------------|--------|----------|-------|
| quiz_id         | INT          | Foreign key to quiz           | False  | True     | True  |
| user_id         | INT          | Foreign key to user           | False  | True     | True  |

```sql
CREATE TABLE quiz_editors (
    quiz_id INT NOT NULL,
    user_id INT NOT NULL,
    CONSTRAINT fk_quiz FOREIGN KEY (quiz_id)
        REFERENCES quizs (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_editor FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    UNIQUE (quiz_id, user_id)
);
CREATE INDEX idx_quiz_editors ON quiz_editors (quiz_id, user_id);
```

## Viewers
| Field           | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|-----------------|--------------|-------------------------------|--------|----------|-------|
| quiz_id         | INT          | Foreign key to quiz           | False  | True     | True  |
| user_id         | INT          | Foreign key to user           | False  | True     | True  |

```sql
CREATE TABLE quiz_viewers (
    quiz_id INT NOT NULL,
    user_id INT NOT NULL,
    CONSTRAINT fk_quiz FOREIGN KEY (quiz_id)
        REFERENCES libraries (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_viewer FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    UNIQUE (quiz_id, user_id)
);
CREATE INDEX idx_quiz_viewers ON quiz_viewers (quiz_id, user_id);
```

## Questions
| Field           | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|-----------------|--------------|-------------------------------|--------|----------|-------|
| id              | INT          | Card primary key              | True   | True     | True  |
| quiz_id         | INT          | The quiz the question is in   | False  | True     | True  |
| place           | INT          | Place in quiz                 | False  | True     | False |
| question        | TEXT         | The front of the card         | False  | True     | False |
| answer          | TEXT         | The back of the card          | False  | True     | False |
| created_at      | TIMESTAMP    | The quiz's creation time      | False  | True     | False |
| last_modified   | TIMESTAMP    | Last modification time        | False  | True     | False |

```sql
CREATE TABLE questions (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    quiz_id INT NOT NULL,
    place INT NOT NULL,
    question TEXT NOT NULL,
    answer TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    last_modified TIMESTAMP NOT NULL,
    UNIQUE (quiz_id, place)
);
CREATE INDEX idx_question_quiz ON questions (quiz_id);
```

## Progress
| Field           | Type         | Description                   | UNIQUE | NOT NULL | INDEX |
|-----------------|--------------|-------------------------------|--------|----------|-------|
| id              | INT          | Primary key                   | True   | True     | True  |
| user_id         | INT          | The user's progress           | False  | True     | True  |
| question_id     | INT          | Associated question           | False  | True     | True  |
| review_count    | INT          | Number of times reviewed      | False  | True     | True  |
| last_reviewed   | TIMESTAMP    | Last review time              | False  | False    | False |
| ease_factor     | FLOAT        | Spaced repetition ease factor | False  | True     | False |

```sql
CREATE TABLE progress (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    question_id INT NOT NULL,
    review_count INT NOT NULL,
    last_review TIMESTAMP,
    ease_factor FLOAT NOT NULL,
    Unique (user_id, question_id)
);
CREATE INDEX idx_progress_user ON progress (user_id);
CREATE INDEX idx_progress_question ON progress (question_id);
CREATE INDEX idx_progress_review_count ON progress (review_count);
```
