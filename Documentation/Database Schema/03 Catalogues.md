# Catalogues Tables
Authors
| Field           | Type           | Description                | IS UNIQUE | NOT NULL | INDEX  |
|-----------------|----------------|----------------------------|-----------|----------|--------|
| author_id       | INT            | Author's primary key       | True      | True     |  |
| user_id         | INT            | Foreign key to user id     | True      | False    |  |
| first_name      | VARCHAR(20)    | Author's first name        | False     | False    |  |
| last_name       | VARCHAR(20)    | Author's last name         | False     | False    |  |
| middle_names    | VARCHAR(20)    | Author's middle name       | False     | False    |  |
| pen_name        | VARCHAR(20)    | Author's pen name          | False     | False    |  |
| date_of_birth   | DATE           | Author's date of birth     | False     | False    |  |
| date_of_death   | DATE           | Author's date of death     | False     | False    |  |

Books
| Field           | Type           | Description                | IS UNIQUE | NOT NULL | INDEX  |
|-----------------|----------------|----------------------------|-----------|----------|--------|
| book_id         | VARCHAR(100)   | The book's primary key     | True      | True     |  |
| format_ids      | INT[]          | Foreign key book formats   | False     | True     |  |
| Author_ids      | INT[]          | Author foreign key         | False     | False    |  |
| co-author_ids   | INT[]          | Author foreign key         | False     | False    |  |
| genre_ids       | INT[]          | Book genres foreign key    | False     | True     |  |
| publisher_id    | INT            | Publisher foreign key      | False     | False    |  |
| title           | VARCHAR        | The book's title           | False     | True     |  |
| subtitle        | VARCHAR(100)   | The book's subitile        | False     | False    |  |
| edition         | INT            | The book's edition         | False     | False    |  |
| pages           | INT            | Pages in book              | False     | True     |  |
| isbn            | INT            | Book's ISBN number         | True      | False    |  |
| isbn_13         | INT            | Book's ISBN-13 number      | True      | False    |  |
| synopsis        | TEXT           | Book synopsis              | False     | False    |  |
| links           | VARCHAR(100)[] | Links to buy book          | True      | False    |  |

Formats
| Field           | Type           | Description                | IS UNIQUE | NOT NULL | INDEX  |
|-----------------|----------------|----------------------------|-----------|----------|--------|
| format_id       | INT            | Primary key for formats    | True      | True     |  |
| format_name     | VARCHAR(30)    | Name of the format         | True      | True     |  |
| description     | TEXT           | Description of the format  | False     | True     |  |

Publishers
| Field           | Type           | Description                | IS UNIQUE | NOT NULL | INDEX  |
|-----------------|----------------|----------------------------|-----------|----------|--------|
| publisher_id    | INT            | Primary key                | True      | True     |  |
| publisher_name  | VARCHAR(100)   | Publisher name             | True      | True     |  |
| publisher_house | INT            | Foreign key to publisher   | False     | False    |  |

Genres
| Field           | Type           | Description                | IS UNIQUE | NOT NULL | INDEX  |
|-----------------|----------------|----------------------------|-----------|----------|--------|
| genre_id        | INT            | The genre's Primary key    | True      | True     |  |
| genre_name      | VARCHAR(100)   | Genre name                 | True      | True     |  |
| subgenre_ids    | INT[]          | Foreign key to genre       | False     | False    |  |
| supergenre_ids  | INT[]          | Foreign key to genre       | False     | False    |  |
