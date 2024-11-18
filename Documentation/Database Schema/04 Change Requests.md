# Change Requeste Tables
## Entity types
| Field             | Type           | Description                | IS UNIQUE | NOT NULL | INDEX  |
|-------------------|----------------|----------------------------|-----------|----------|--------|
| id                | INT            | Primary key                | True      | True     | True   |
| entity_type       | VARCHAR(50)    | The type entity modified   | True      | Unique   | True   |
-- 'book', 'author', 'publisher', 'format'

## Status
| Field             | Type           | Description                | IS UNIQUE | NOT NULL | INDEX  |
|-------------------|----------------|----------------------------|-----------|----------|--------|
| id                | INT            | Primary key                | True      | True     | True   |
| status            | VARCHAR(20)    | The status of the request  | True      | Unique   | True   |
-- 'pending', 'approved', 'rejected'

## Change Requests
| Field             | Type           | Description                | IS UNIQUE | NOT NULL | INDEX  |
|-------------------|----------------|----------------------------|-----------|----------|--------|
| id                | INT            | Primary key                | True      | True     | True   |
| user_id           | INT            | The request submitter      | 
| entity_type_id    | INT            | The entity type
| entity_uuid       | UUID           | UUID of the thing being changed
| submitted_at      | TIMESTAMP      | When change request was submitted
| reviewed_at       | TIMESTAMP      | When the request was reviewed
| admin_id          | INT            | The reviewer's user_id
| status            | STATUS         | The change request's status
| change_summary    | TEXT           | Description of the change request


## Change Request Details
| Field             | Type           | Description                | IS UNIQUE | NOT NULL | INDEX  |
|-------------------|----------------|----------------------------|-----------|----------|--------|
| id                | INT            | Primary key                |
| change_request_id | INT            | Change request foreign key |
| field[]           | INT or varchar |  |
| old_value[]       | TEXT           |  |
| new_value[]       | TEXT           |  |


field_name: Specifies the field being modified (e.g., title, genre_name).
old_value: Captures the current value of the field.
new_value: Contains the user’s proposed new value.

