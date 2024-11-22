# Catalogues Tables
## Formats
| Field               | Type           | Description                | UNIQUE | NOT NULL | INDEX |
|---------------------|----------------|----------------------------|--------|----------|-------|
| id                  | INT            | Primary key for formats    | True   | True     | False |
| format_name         | VARCHAR(30)    | Name of the format         | True   | True     | True  |
| description         | TEXT           | Description of the format  | False  | True     | False |

```sql
CREATE TABLE IF NOT EXISTS formats(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    format_name VARCHAR(30) UNIQUE NOT NULL,
    description TEXT NOT NULL,
);
CREATE INDEX idx_formats_name ON formats (format_name);

## Publishers
| Field               | Type           | Description                | UNIQUE | NOT NULL | INDEX |
|---------------------|----------------|----------------------------|--------|----------|-------|
| id                  | INT            | Primary key                | True   | True     | True  |
| publisher_name      | VARCHAR(100)   | Publisher name             | True   | True     | True  |

```sql
CREATE TABLE IF NOT EXISTS publishers(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    publisher_name VARCHAR(100) UNIQUE NOT NULL,
);
CREATE INDEX idx_publishers_name ON publishers (publisher_name);

## Genres
| Field               | Type           | Description                | UNIQUE | NOT NULL | INDEX |
|---------------------|----------------|----------------------------|--------|----------|-------|
| id                  | INT            | The genre's Primary key    | True   | True     | True  |
| genre_name          | VARCHAR(100)   | Genre name                 | True   | True     | True  |

```sql
CREATE TABLE IF NOT EXISTS genres(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    genre_name VARCHAR(100) UNIQUE NOT NULL,
);
CREATE INDEX idx_genres_name ON genres (genre_name);

## Relationships
### Publisher relationships
| Field               | Type           | Description                | UNIQUE | NOT NULL | INDEX |
|---------------------|----------------|----------------------------|--------|----------|-------|
| parent_publisher_id | INT            | parent publisher id        | False  | True     | True  |
| child_publisher_id  | INT            | child publisher id         | False  | True     | True  |

```sql
CREATE TABLE publisher_relationships (
    parent_publisher_id INT NOT NULL,
    child_publisher_id INT NOT NULL,
    PRIMARY KEY (parent_publisher_id, child_publisher_id),
    CONSTRAINT fk_parent FOREIGN KEY(parent_publisher_id)
        REFERENCES publishers (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_child FOREIGN KEY(child_publisher_id)
        REFERENCES publishers (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
CREATE INDEX idx_publisher_relationships_parent_id ON publisher_relationships (parent_publisher_id);
CREATE INDEX idx_publisher_relationships_child_id ON publisher_relationships (child_publisher_id);

### Genre relationships
| Field               | Type           | Description                | UNIQUE | NOT NULL | INDEX |
|---------------------|----------------|----------------------------|--------|----------|-------|
| parent_genre_id     | INT            | parent genre id            | False  | True     | True  |
| child_genre_id      | INT            | child genre id             | False  | True     | True  |

```sql
CREATE TABLE genre_relationships (
    parent_genre_id INT NOT NULL,
    child_genre_id INT NOT NULL,
    PRIMARY KEY (parent_genre_id, child_genre_id),
    CONSTRAINT fk_parent FOREIGN KEY(parent_genre_id)
        REFERENCES genres (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_child FOREIGN KEY(child_genre_id)
        REFERENCES genres (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
CREATE INDEX idx_genre_relationships_parent_id ON genre_relationships (parent_genre_id);
CREATE INDEX idx_genre_relationships_child_id ON genre_relationships (child_genre_id);

### Examples
Find all subgenre ids for a genre:
```sql
SELECT g2.id AS sub_genre_id, g2.genre_name AS sub_genre_name
FROM genre_relationships gr
JOIN genres g1 ON gr.parent_genre_id = g1.id
JOIN genres g2 ON gr.child_genre_id = g2.id
WHERE g1.genre_name = 'Fiction';
```

Find all ancestors of a particular genre:
```sql
WITH RECURSIVE genre_ancestors AS (
    SELECT parent_genre_id, child_genre_id
    FROM genre_relationships
    WHERE child_genre_id = (SELECT id FROM genres WHERE genre_name = 'Cyberpunk')
    UNION ALL
    SELECT gr.parent_genre_id, gr.child_genre_id
    FROM genre_relationships gr
    INNER JOIN genre_ancestors ga ON gr.child_genre_id = ga.parent_genre_id
)
SELECT g.id AS ancestor_id, g.genre_name AS ancestor_name
FROM genre_ancestors ga
JOIN genres g ON ga.parent_genre_id = g.id;
```

Find all decendents of a particular genre:
```sql
WITH RECURSIVE genre_descendants AS (
    SELECT parent_genre_id, child_genre_id
    FROM genre_relationships
    WHERE parent_genre_id = (SELECT id FROM genres WHERE genre_name = 'Non-Fiction')
    UNION ALL
    SELECT gr.parent_genre_id, gr.child_genre_id
    FROM genre_relationships gr
    INNER JOIN genre_descendants gd ON gr.parent_genre_id = gd.child_genre_id
)
SELECT g.id AS descendant_id, g.genre_name AS descendant_name
FROM genre_descendants gd
JOIN genres g ON gd.child_genre_id = g.id;
```

Find all books under a specific genre and its sub-genres:
```sql
WITH RECURSIVE genre_descendants AS (
    SELECT parent_genre_id, child_genre_id
    FROM genre_relationships
    WHERE parent_genre_id = (SELECT id FROM genres WHERE genre_name = 'Fantasy')
    UNION ALL
    SELECT gr.parent_genre_id, gr.child_genre_id
    FROM genre_relationships gr
    INNER JOIN genre_descendants gd ON gr.parent_genre_id = gd.child_genre_id
)
SELECT b.id AS book_id, b.title AS book_title
FROM genre_descendants gd
JOIN book_genres bg ON gd.child_genre_id = bg.genre_id
JOIN books b ON bg.book_id = b.id;
```

Count the number of sub-genres:
```sql
SELECT COUNT(*) AS sub_genre_count
FROM genre_relationships gr
JOIN genres g1 ON gr.parent_genre_id = g1.id
WHERE g1.genre_name = 'Mystery';
```

## Authors
| Field               | Type           | Description                | UNIQUE | NOT NULL | INDEX |
|---------------------|----------------|----------------------------|--------|----------|-------|
| id                  | INT            | Author's primary key       | True   | True     | False |
| user_id             | INT            | Foreign key to user id     | True   | False    | False |
| first_name          | VARCHAR(20)    | Author's first name        | False  | False    | True  |
| last_name           | VARCHAR(20)    | Author's last name         | False  | False    | True  |
| middle_name         | VARCHAR(20)    | Author's middle name       | False  | False    | False |
| pen_name            | VARCHAR(20)    | Author's pen name          | False  | False    | True  |
| date_of_birth       | DATE           | Author's date of birth     | False  | False    | False |
| date_of_death       | DATE           | Author's date of death     | False  | False    | False |
| information         | TEXT           | The author's information   | False  | False    | False |

```sql
CREATE TABLE IF NOT EXISTS authors(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id int NOT NULL,
    first_name VARCHAR(20),
    last_name VARCHAR(20),
    middle_name VARCHAR(20),
    pen_name VARCHAR(20),
    date_of_birth DATE,
    date_of_death DATE,
    information TEXT,
    CONSTRAINT fk_users FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
CREATE INDEX idx_authors_fname ON authors (first_name);
CREATE INDEX idx_authors_lname ON authors (last_name);
CREATE INDEX idx_authors_pname ON authors (pen_name);
```

## Books
### Main table
| Field               | Type           | Description                | UNIQUE | NOT NULL | INDEX |
|---------------------|----------------|----------------------------|--------|----------|-------|
| id                  | VARCHAR(100)   | Primary key for the book   | True   | True     | True  |
| publisher_id        | INT            | Publisher foreign key      | False  | False    | False |
| title               | VARCHAR(100)   | The book's title           | False  | True     | True  |
| subtitle            | VARCHAR(100)   | The book's subitile        | False  | False    | True  |
| edition             | INT            | The book's edition         | False  | False    | False |
| pages               | INT            | Pages in book              | False  | True     | False |
| isbn                | INT            | Book's ISBN number         | True   | False    | True  |
| isbn_13             | INT            | Book's ISBN-13 number      | True   | False    | True  |
| synopsis            | TEXT           | Book synopsis              | False  | False    | False |
| links               | VARCHAR(255)[] | Links to buy book          | True   | False    | False |

```sql
CREATE TABLE IF NOT EXISTS books(
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    publisher_id INT,
    title VARCHAR(100) NOT NULL,
    subtitle VARCHAR(100),
    edition INT,
    pages INT NOT NULL,
    isbn INT UNIQUE,
    isbn_13 INT UNIQUE,
    synopsis TEXT,
    links VARCHAR(255) UNIQUE,
    CONSTRAINT fk_publishers FOREIGN KEY(publisher_id)
        REFERENCES publishers (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
); 
CREATE INDEX idx_books_title ON authors (title);
CREATE INDEX idx_books_subtitle ON authors (subtitle);
CREATE INDEX idx_books_isbn ON authors (isbn);
CREATE INDEX idx_books_isbn_13 ON authors (isbn_13);
```

## Book relationships
### Book - Format relationship
| Field               | Type           | Description                | UNIQUE | NOT NULL | INDEX |
|---------------------|----------------|----------------------------|--------|----------|-------|
| book_id             | INT            | Book foreign key           | False  | True     | True  |
| format_id           | INT            | Format foreign key         | False  | True     | True  |

```sql
CREATE TABLE book_format_relationships (
    book_id INT NOT NULL,
    format_id INT NOT NULL,
    PRIMARY KEY (book_id, format_id),
    CONSTRAINT fk_book FOREIGN KEY(book_id)
        REFERENCES books (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_format FOREIGN KEY(format_id)
        REFERENCES formats (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
CREATE INDEX idx_book_format_relationships_book_id ON book_format_relationships (book_id);
CREATE INDEX idx_book_format_relationships_format_id ON book_format_relationships (format_id);
```

### Book - Genre relationship
| Field               | Type           | Description                | UNIQUE | NOT NULL | INDEX |
|---------------------|----------------|----------------------------|--------|----------|-------|
| book_id             | INT            | Book foreign key           | False  | True     | True  |
| genre_id            | INT            | Genre foreign key          | False  | True     | True  |

```sql
CREATE TABLE book_genre_relationships (
    book_id INT NOT NULL,
    genre_id INT NOT NULL,
    PRIMARY KEY (book_id, genre_id),
    CONSTRAINT fk_book FOREIGN KEY(book_id),
        REFERENCES books (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_genre FOREIGN KEY(genre_id)
        REFERENCES genres (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
CREATE INDEX idx_book_genre_relationships_book_id ON book_genre_relationships (book_id);
CREATE INDEX idx_book_genre_relationships_genre_id ON book_genre_relationships (genre_id);
```

### Book - Author relationship
| Field               | Type           | Description                | UNIQUE | NOT NULL | INDEX |
|---------------------|----------------|----------------------------|--------|----------|-------|
| book_id             | INT            | Book foreign key           | False  | True     | True  |
| author_id           | INT            | Author foreign key         | False  | True     | True  |

```sql
CREATE TABLE book_author_relationships (
    book_id INT NOT NULL,
    author_id INT NOT NULL,
    is_primary_author BOOLEAN NOT NULL DEFAULT TRUE,
    PRIMARY KEY (book_id, author_id),
        REFERENCES books (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_author FOREIGN KEY(author_id)
        REFERENCES authors (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
CREATE INDEX idx_book_author_relationships_book_id ON book_author_relationships (book_id);
CREATE INDEX idx_book_author_relationships_author_id ON book_author_relationships (author_id);
```