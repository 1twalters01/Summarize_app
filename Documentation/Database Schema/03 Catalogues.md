# Catalogues Tables
## Formats
CREATE TABLE IF NOT EXISTS formats(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    format_name VARCHAR(30) UNIQUE NOT NULL,
    description TEXT UNIQUE NOT NULL,
);

Formats
| Field              | Type           | Description                | IS UNIQUE | NOT NULL | INDEX  |
|--------------------|----------------|----------------------------|-----------|----------|--------|
| id                 | INT            | Primary key for formats    | True      | True     |  |
| format_name        | VARCHAR(30)    | Name of the format         | True      | True     |  |
| description        | TEXT           | Description of the format  | True      | True     |  |

## Publishers
CREATE TABLE IF NOT EXISTS publishers(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    publisher_name VARCHAR(100) UNIQUE NOT NULL,
    super_publisher_id INT UNIQUE,
    sub_publisher_id INT UNIQUE,
    CONSTRAINT fk_super FOREIGN KEY(super_publisher_id)
        REFERENCES publishers (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_sub FOREIGN KEY(sub_publisher_id)
        REFERENCES publishers (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

Publishers
| Field              | Type           | Description                | IS UNIQUE | NOT NULL | INDEX  |
|--------------------|----------------|----------------------------|-----------|----------|--------|
| id                 | INT            | Primary key                | True      | True     |  |
| publisher_name     | VARCHAR(100)   | Publisher name             | True      | True     |  |
| super_publisher_id | INT            | Foreign key to publisher   | False     | False    |  |
| sub_publisher_id   | INT            | Foreign key to publisher   | False     | False    |  |

## Genres
CREATE TABLE IF NOT EXISTS genres(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    genre_name VARCHAR(100) UNIQUE NOT NULL,
    super_genre_ids INT UNIQUE,
    sub_genre_ids INT UNIQUE,
    CONSTRAINT fk_super FOREIGN KEY(super_genre_ids)
        REFERENCES genres (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_sub FOREIGN KEY(sub_genre_ids)
        REFERENCES genres (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

Genres
| Field              | Type           | Description                | IS UNIQUE | NOT NULL | INDEX  |
|--------------------|----------------|----------------------------|-----------|----------|--------|
| id                 | INT            | The genre's Primary key    | True      | True     |  |
| genre_name         | VARCHAR(100)   | Genre name                 | True      | True     |  |
| sub_genre_ids      | INT[]          | Foreign key to genre       | False     | False    |  |
| super_genre_ids    | INT[]          | Foreign key to genre       | False     | False    |  |

## Authors
CREATE TABLE IF NOT EXISTS authors(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id int NOT NULL,
    first_name VARCHAR(20),
    last_name VARCHAR(20),
    middle_name VARCHAR(20),
    pen_name VARCHAR(20),
    date_of_birth TIMESTAMP,
    date_of_death TIMESTAMP,
    information TEXT UNIQUE NOT NULL,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

Authors
| Field              | Type           | Description                | IS UNIQUE | NOT NULL | INDEX  |
|--------------------|----------------|----------------------------|-----------|----------|--------|
| id                 | INT            | Author's primary key       | True      | True     |  |
| user_id            | INT            | Foreign key to user id     | True      | False    |  |
| first_name         | VARCHAR(20)    | Author's first name        | False     | False    |  |
| last_name          | VARCHAR(20)    | Author's last name         | False     | False    |  |
| middle_names       | VARCHAR(20)    | Author's middle name       | False     | False    |  |
| pen_name           | VARCHAR(20)    | Author's pen name          | False     | False    |  |
| date_of_birth      | DATE           | Author's date of birth     | False     | False    |  |
| date_of_death      | DATE           | Author's date of death     | False     | False    |  |
| information        | TEXT           | The author's information   | True      | True     |  |

## Books
CREATE TABLE IF NOT EXISTS books(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    format_ids INT[] NOT NULL,
    author_ids INT[] NOT NULL,
    co_author_ids INT[],
    genre_ids INT[] NOT NULL,
    publisher_id INT[],
    title VARCHAR(100) NOT NULL,
    subtitle VARCHAR(100),
    edition INT,
    pages INT NOT NULL,
    isbn INT UNIQUE,
    isbn_13 INT UNIQUE,
    synopsis TEXT,
    links VARCHAR(100) UNIQUE,
    CONSTRAINT fk_formats FOREIGN KEY(format_ids)
        REFERENCES formats (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_authors FOREIGN KEY(author_ids)
        REFERENCES authors (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_co_authors FOREIGN KEY(co_author_ids)
        REFERENCES formats (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_genres FOREIGN KEY(genre_ids)
        REFERENCES genres (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_publishers FOREIGN KEY(publisher_id)
        REFERENCES publishers (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

Books
| Field              | Type           | Description                | IS UNIQUE | NOT NULL | INDEX  |
|--------------------|----------------|----------------------------|-----------|----------|--------|
| id                 | VARCHAR(100)   | Primary key for the book   | True      | True     |  |
| format_ids         | INT[]          | Foreign key book formats   | False     | True     |  |
| Author_ids         | INT[]          | Author foreign key         | False     | True     |  |
| co_author_ids      | INT[]          | Author foreign key         | False     | False    |  |
| genre_ids          | INT[]          | Book genres foreign key    | False     | True     |  |
| publisher_id       | INT            | Publisher foreign key      | False     | False    |  |
| title              | VARCHAR(100)   | The book's title           | False     | True     |  |
| subtitle           | VARCHAR(100)   | The book's subitile        | False     | False    |  |
| edition            | INT            | The book's edition         | False     | False    |  |
| pages              | INT            | Pages in book              | False     | True     |  |
| isbn               | INT            | Book's ISBN number         | True      | False    |  |
| isbn_13            | INT            | Book's ISBN-13 number      | True      | False    |  |
| synopsis           | TEXT           | Book synopsis              | False     | False    |  |
| links              | VARCHAR(100)[] | Links to buy book          | True      | False    |  |
