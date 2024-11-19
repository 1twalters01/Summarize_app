# Libraries Tables
## Libraries
| Field            | Type           | Description                      | IS UNIQUE | NOT NULL | INDEX |
|------------------|----------------|----------------------------------|-----------|----------|-------|
| id               | INT            | Primary key of library           | True      | True     |
| user_id          | INT            | Foreign key to owner             | False     | True     |
| editors          | INT[]          | Users allowed to edit            | False     | False    |
| library_name     | VARCHAR(100)   | Name of the library              | False     | True     |
| time_created     | DATETIME       | Time of library creation         | False     | True     |
| last_modified    | DATETIME       | Time of last modification        | False     | False    |
| last_added       | DATETIME       | Last summary added time          | False     | False    |
| is_public        | BOOLEAN        | Public status of library         | False     | True     |
| viewer_ids       | INT[]          | Users that can view if private   | False     | False    |
| follows          | Int            | Number of follows                | False     | True     |

## Library Entries
| Field            | Type           | Description                      | IS UNIQUE | NOT NULL | INDEX |
|------------------|----------------|----------------------------------|-----------|----------|-------|
| summary_id       | INT            | Foreign key to summaries         | False     | False    |
| library_id       | INT            | Foreign key to library           | False     | False    |
| date_added       | DATETIME       | Last summary added time          | False     | False    |
| added_by_id      | INT            | User that added the summary      | False     | False    |

## Library Collections
| Field            | Type           | Description                      | IS UNIQUE | NOT NULL | INDEX |
|------------------|----------------|----------------------------------|-----------|----------|-------|
| collection_id    | INT            | Primary key to collection        | True      | True     |
| library_id       | INT[]          | Foreign key to libraries         | False     | False    |
| owner_uuid       | UUID           | User or group that owns this     | False     | True     |
| created_by       | INT            | User that created the collection | False     | True     |
| time_created     | DATETIME       | Time of collection creation      | False     | True     |
| last_modified_by | INT            | User that last modified this     | False     | True     |
| last_modified    | DATETIME       | Time of last modification        | False     | False    |
