CREATE TABLE languages (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    iso TEXT NOT NULL
);

CREATE TABLE phrases (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    original TEXT NOT NULL UNIQUE,
    lang_id INTEGER NOT NULL,
    translation TEXT NOT NULL,
    romanization TEXT,
    FOREIGN KEY (lang_id) REFERENCES languages (id)
);
