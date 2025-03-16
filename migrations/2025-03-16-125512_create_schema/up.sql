CREATE TABLE languages (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    name VARCHAR NOT NULL,
    iso VARCHAR NOT NULL
);

CREATE TABLE phrases (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    original TEXT NOT NULL,
    lang_id INTEGER NOT NULL,
    translation TEXT NOT NULL,
    romanization TEXT,

    FOREIGN KEY (lang_id) REFERENCES languages (id)
);