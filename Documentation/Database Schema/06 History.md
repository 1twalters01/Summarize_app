# History Tables
## Redis


## Postgres
History (Postgres)
| Field           | Type           | Description                | IS UNIQUE | NOT NULL |
|-----------------|----------------|----------------------------|-----------|----------|
| id              | INT            | Primary key of the history | True      | True     |
| user_id         | INT            | Foreign key to user        | False     | True     |

Datatype
| Field           | Type           | Description                | IS UNIQUE | NOT NULL |
|-----------------|----------------|----------------------------|-----------|----------|
| datatype_id     | INT            | Primary key                | True      | True     |
| datatype        | VARCHAR(20)    | Datatype of entry          | True      | True     |

[comment]: # (datatypes: summary, book, author, publisher, library, user)

History Entries
| Field           | Type           | Description                | IS UNIQUE | NOT NULL |
|-----------------|----------------|----------------------------|-----------|----------|
| history_id      | INT            | Foreign key to History     | False     | True     |
| data_uuid       | UUID           | The uuid of the entry      | True      | True     |
| datatype        | DATATYPE       | The type of the entry      | True      | True     |
| time_added      | TIMESTAMP      | The time when added        | True      | True     |

## Datalake
