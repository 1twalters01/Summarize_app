# Libraries Tables
## Libraries
| Field            | Type         | Description                      | UNIQUE | NOT NULL | INDEX |
|------------------|--------------|----------------------------------|--------|----------|-------|
| id               | INT          | Primary key of library           | True   | True     | True  |
| user_id          | INT          | Foreign key to owner             | False  | True     | True  |
| library_name     | VARCHAR(100) | Name of the library              | False  | True     | False |
| description      | TEXT         | Optional library description     | False  | False    | False |
| time_created     | DATETIME     | Time of library creation         | False  | True     | False |
| last_modified    | DATETIME     | Time of last modification        | False  | True     | False |
| last_added       | DATETIME     | Last summary added time          | False  | False    | False |
| is_public        | BOOLEAN      | Public status of library         | False  | True     | True  |
| follows          | Int          | Number of follows                | False  | True     | False |

CREATE TABLE libraries (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INT NOT NULL,
    library_name VARCHAR(100) NOT NULL,
    description TEXT,
    time_created TIMESTAMP DEFAULT NOW(),
    last_modified TIMESTAMP DEFAULT NOW(),
    last_added TIMESTAMP,
    is_public BOOLEAN NOT NULL DEFAULT FALSE,
    CONSTRAINT fk_user FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
CREATE INDEX idx_libraries_user ON libraries (user_id);
CREATE INDEX idx_libraries_public ON libraries (is_public);

## Editors
| Field            | Type         | Description                      | UNIQUE | NOT NULL | INDEX |
|------------------|--------------|----------------------------------|--------|----------|-------|
| library_id       | INT          | Foreign key to library           | False  | True     | True  |
| user_id          | INT          | Foreign key to user              | False  | True     | True  |

CREATE TABLE library_editors (
    library_id INT NOT NULL,
    user_id INT NOT NULL,
    CONSTRAINT fk_library FOREIGN KEY (library_id)
        REFERENCES libraries (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_editor FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    UNIQUE (library_id, user_id)
);
CREATE INDEX idx_library_editors ON library_editors (library_id, user_id);

## Viewers
| Field            | Type         | Description                      | UNIQUE | NOT NULL | INDEX |
|------------------|--------------|----------------------------------|--------|----------|-------|
| library_id       | INT          | Foreign key to library           | False  | True     | True  |
| user_id          | INT          | Foreign key to user              | False  | True     | True  |

CREATE TABLE library_viewers (
    library_id INT NOT NULL,
    user_id INT NOT NULL,
    CONSTRAINT fk_library FOREIGN KEY (library_id)
        REFERENCES libraries (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_viewer FOREIGN KEY (user_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    UNIQUE (library_id, user_id) -- Prevent duplicate viewer entries
);
CREATE INDEX idx_library_viewers ON library_viewers (library_id, user_id);

## Shelves
| Field            | Type         | Description                      | UNIQUE | NOT NULL | INDEX |
|------------------|--------------|----------------------------------|--------|----------|-------|
| id               | INT          | Primary key of shelf             | True   | True     | True  |
| library_id       | INT          | Library foreign key              | False  | True     | True  |
| name             | VARCHAR(20)  | Name of shelf                    | False  | True     | False |

CREATE TABLE shelves (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    library_id INT NOT NULL,
    name VARCHAR(100) NOT NULL,
    CONSTRAINT fk_library FOREIGN KEY (library_id)
        REFERENCES libraries (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);

## Entries
| Field            | Type         | Description                      | UNIQUE | NOT NULL | INDEX |
|------------------|--------------|----------------------------------|--------|----------|-------|
| ID               | INT          | Primary key of entry             | True   | True     | True  |
| summary_id       | INT          | Foreign key to summaries         | False  | False    |
| shelf_id         | INT          | Foreign key to shelves           | False  | False    |
| added_by_id      | INT          | User that added the summary      | False  | False    |
| date_added       | DATETIME     | Last summary added time          | False  | False    |

CREATE TABLE entries (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    summary_id INT NOT NULL,
    shelf_id INT,
    library_id INT NOT NULL, -- Redundant but helpful for direct queries
    added_by_id INT NOT NULL,
    date_added TIMESTAMP DEFAULT NOW(),
    CONSTRAINT fk_summary FOREIGN KEY (summary_id)
        REFERENCES summaries (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_shelf FOREIGN KEY (shelf_id)
        REFERENCES shelves (id)
        ON DELETE SET NULL
        ON UPDATE CASCADE,
    CONSTRAINT fk_library FOREIGN KEY (library_id)
        REFERENCES libraries (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_added_by FOREIGN KEY (added_by_id)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE
);
CREATE INDEX idx_shelf_entries_library_summary ON entries (library_id, summary_id);

## Collections
| Field            | Type         | Description                      | IS UNIQUE | NOT NULL | INDEX |
|------------------|--------------|----------------------------------|-----------|----------|-------|
| id               | INT          | Primary key to collection        | True      | True     | True  |
| created_by       | INT          | User that created the collection | False     | True     | False |
| owner_uuid       | UUID         | User or group that owns this     | False     | True     | False |
| time_created     | DATETIME     | Time of collection creation      | False     | True     | False |
| last_modified    | DATETIME     | Time of last modification        | False     | False    | False |
| last_modified_by | INT          | User that last modified this     | False     | True     | False |

CREATE TABLE library_collections (
    id INT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    created_by INT NOT NULL, -- FK to users table
    owner_id INT NOT NULL, -- FK to users table
    name VARCHAR(100) NOT NULL, -- Collection name
    description TEXT, -- Optional description
    time_created TIMESTAMP DEFAULT NOW(),
    last_modified TIMESTAMP,
    last_modified_by INT, -- FK to users table
    CONSTRAINT fk_collection_owner FOREIGN KEY (owner_id) REFERENCES users (id) ON DELETE CASCADE,
    CONSTRAINT fk_created_by FOREIGN KEY (created_by)
        REFERENCES users (id)
        ON DELETE CASCADE
        ON UPDATE CASCADE,
    CONSTRAINT fk_modified_by FOREIGN KEY (last_modified_by)
        REFERENCES users (id)
        ON DELETE SET NULL
        ON UPDATE CASCADE
);

# Library Collections links
| Field            | Type         | Description                      | IS UNIQUE | NOT NULL | INDEX |
|------------------|--------------|----------------------------------|-----------|----------|-------|
| library_id       | INT          | Library foreign key              | False     | True     | True  |
| collection_id    | INT          | Collection foreign key           | False     | True     | True  |

CREATE TABLE library_collection_links (
    library_id INT NOT NULL, -- FK to libraries table
    collection_id INT NOT NULL, -- FK to library_collections table
    CONSTRAINT fk_link_library FOREIGN KEY (library_id) REFERENCES libraries (id) ON DELETE CASCADE,
    CONSTRAINT fk_link_collection FOREIGN KEY (collection_id) REFERENCES library_collections (id) ON DELETE CASCADE,
    UNIQUE (library_id, collection_id) -- Prevent duplicate links
);
CREATE INDEX idx_library_collection_links ON library_collection_links (collection_id, library_id);
