CREATE TABLE players (
                       id VARCHAR(50) NOT NULL PRIMARY KEY,
                       pseudo VARCHAR(15) NOT NULL,
                       password TEXT NOT NULL,
                       is_mj INTEGER(1) NOT NULL DEFAULT 0
);
CREATE TABLE posts (
                       id INTEGER PRIMARY KEY AUTOINCREMENT,
                       author VARCHAR(100) NOT NULL,
                       body TEXT NOT NULL,
                       published_at TIMESTAMP NOT NULL
);