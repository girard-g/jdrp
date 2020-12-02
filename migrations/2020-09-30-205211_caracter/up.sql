CREATE TABLE caracter (
                       id VARCHAR(50) NOT NULL PRIMARY KEY,
                       player_id VARCHAR(50) NOT NULL,
                       stats TEXT NOT NULL,
                       gold INT(5) NOT NULL DEFAULT 0
);