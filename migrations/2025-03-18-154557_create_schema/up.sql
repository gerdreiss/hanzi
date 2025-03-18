CREATE TABLE languages (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE
);

CREATE TABLE phrases (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    language_id INTEGER NOT NULL,
    text TEXT NOT NULL UNIQUE,
    translation TEXT NOT NULL,
    romanization TEXT,
    FOREIGN KEY (language_id) REFERENCES languages (id)
);
