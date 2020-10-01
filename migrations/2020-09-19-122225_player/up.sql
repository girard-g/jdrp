CREATE TABLE players (
                       id VARCHAR(50) NOT NULL PRIMARY KEY,
                       pseudo VARCHAR(15) NOT NULL,
                       password TEXT NOT NULL,
                       is_mj INTEGER(1) NOT NULL DEFAULT 0
);
