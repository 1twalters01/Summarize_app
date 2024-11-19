# Change Request Tables
## Entity types
| Field             | Type        | Description                | UNIQUE | NOT NULL | INDEX |
|-------------------|-------------|----------------------------|--------|----------|-------|
| id                | INT         | Primary key                | True   | True     | True  |
| entity_type       | VARCHAR(50) | The type entity modified   | True   | Unique   | True  |
-- 'book', 'author', 'publisher', 'format', 'summary', 'user'

## Status
| Field             | Type        | Description                | UNIQUE | NOT NULL | INDEX |
|-------------------|-------------|----------------------------|--------|----------|-------|
| id                | INT         | Primary key                | True   | True     | True  |
| status            | VARCHAR(20) | The status of the request  | True   | Unique   | True  |
-- 'pending', 'approved', 'rejected'

## Change Requests
| Field             | Type        | Description                | UNIQUE | NOT NULL | INDEX |
|-------------------|-------------|----------------------------|--------|----------|-------|
| id                | INT         | Primary key                | True   | True     | True  |
| user_id           | INT         | The request submitter      | False  | True     | True  |
| entity_type_id    | INT         | The entity type            | False  | True     | True  |
| entity_uuid       | UUID        | UUID of item being changed | False  | True     | True  |
| submitted_at      | TIMESTAMP   | Time of submission         | False  | True     | False |
| reviewed_at       | TIMESTAMP   | Time of review             | False  | True     | False |
| admin_id          | INT         | The reviewer's user_id     | False  | True     | False |
| status_id         | INT         | Change request status      | False  | True     | True  |
| change_summary    | TEXT        | Change request Description | False  | False    | False |

## Fields
| Field             | Type        | Description                | UNIQUE | NOT NULL | INDEX |
|-------------------|-------------|----------------------------|--------|----------|-------|
| id                | INT         | Primary key                | True   | True     | True  |
| field_name        | VARCHAR(50) | Name of field              | True   | True     | True  |

## Change Request Details
| Field             | Type        | Description                | UNIQUE | NOT NULL | INDEX |
|-------------------|-------------|----------------------------|--------|----------|-------|
| id                | INT         | Primary key                | True   | True     | True  |
| change_request_id | INT         | Change request foreign key | False  | True     | True  |
| field_id          | INT         | Field in question          | False  | True     | False |
| old_value         | TEXT        | Old value                  | False  | True     | False |
| new_value         | TEXT        | New value                  | False  | True     | False |
