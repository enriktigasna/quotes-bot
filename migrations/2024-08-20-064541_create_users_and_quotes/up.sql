CREATE TABLE people (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR NOT NULL UNIQUE
);

CREATE TABLE quotes (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    person_id INTEGER NOT NULL,
    content TEXT NOT NULL,
    FOREIGN KEY (person_id) REFERENCES people(id)
);