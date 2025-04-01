CREATE TABLE phrases (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    original TEXT NOT NULL UNIQUE,
    pinyin TEXT NOT NULL,
    translation TEXT NOT NULL
);
