# Change Request Tables
## Entity types
| Field             | Type        | Description                | UNIQUE | NOT NULL | INDEX |
|-------------------|-------------|----------------------------|--------|----------|-------|
| id                | INT         | Primary key                | True   | True     | True  |
| entity_type       | VARCHAR(50) | The type entity modified   | True   | True     | True  |

[comment]: # (datatypes: book, author, publisher, format)

CREATE TABLE IF NOT EXISTS entity_types (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    entity_type VARCHAR(50) UNIQUE NOT NULL
);

## Status
| Field             | Type        | Description                | UNIQUE | NOT NULL | INDEX |
|-------------------|-------------|----------------------------|--------|----------|-------|
| id                | INT         | Primary key                | True   | True     | True  |
| status            | VARCHAR(20) | The status of the request  | True   | True     | True  |

[comment]: # (datatypes: pending, approved, rejected)

CREATE TABLE IF NOT EXISTS status (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    status VARCHAR(20) UNIQUE NOT NULL
);

## Change Requests
| Field             | Type        | Description                | UNIQUE | NOT NULL | INDEX |
|-------------------|-------------|----------------------------|--------|----------|-------|
| id                | INT         | Primary key                | True   | True     | True  |
| user_id           | INT         | The request submitter      | False  | True     | True  |
| entity_type_id    | INT         | Entity type foreign key    | False  | True     | True  |
| entity_id         | INT         | ID of item being changed   | False  | True     | True  |
| submitted_at      | TIMESTAMP   | Time of submission         | False  | True     | False |
| reviewed_at       | TIMESTAMP   | Time of review             | False  | True     | False |
| admin_id          | INT         | The reviewer's user_id     | False  | True     | False |
| status_id         | INT         | Change request status      | False  | True     | True  |
| change_summary    | TEXT        | Change request Description | False  | False    | False |

CREATE TABLE IF NOT EXISTS change_requests (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    entity_type_id INT NOT NULL,
    entity_id INT NOT NULL,
    submitted_at TIMESTAMP NOT NULL DEFAULT NOW(),
    reviewed_at TIMESTAMP,
    admin_id INT,
    status INT NOT NULL, -- default 'pending'
    change_summary TEXT NOT NULL,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_entity_type FOREIGN KEY (entity_type_id)
        REFERENCES entity_types (id)
        ON DELETE SET NULL
        ON UPDATE CASCADE,
    CONSTRAINT fk_admins FOREIGN KEY (admin_id)
        REFERENCES users (id)
        ON DELETE SET NULL
        ON UPDATE CASCADE
);

## Fields
| Field             | Type        | Description                | UNIQUE | NOT NULL | INDEX |
|-------------------|-------------|----------------------------|--------|----------|-------|
| id                | INT         | Primary key                | True   | True     | True  |
| field_name        | VARCHAR(50) | Name of field              | True   | True     | True  |

CREATE TABLE IF NOT EXISTS status (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    field_name VARCHAR(50) UNIQUE NOT NULL
);

## Change Request Details
| Field             | Type        | Description                | UNIQUE | NOT NULL | INDEX |
|-------------------|-------------|----------------------------|--------|----------|-------|
| id                | INT         | Primary key                | True   | True     | True  |
| change_request_id | INT         | Change request foreign key | False  | True     | True  |
| field_id          | INT         | Field in question          | False  | True     | False |
| old_value         | TEXT        | Old value                  | False  | True     | False |
| new_value         | TEXT        | New value                  | False  | True     | False |

CREATE TABLE IF NOT EXISTS change_request_details (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    change_request_id INT NOT NULL,
    field_id INT NOT NULL,
    old_value TEXT,
    new_value TEXT NOT NULL,
    CONSTRAINT fk_change_requests FOREIGN KEY (change_request_id)
        REFERENCES change_requests (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
