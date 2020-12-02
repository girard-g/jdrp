CREATE TABLE items_generated (
                       id VARCHAR(50) NOT NULL PRIMARY KEY,
                       item_type VARCHAR(50) NOT NULL,
                       equiped INTEGER(1) NOT NULL DEFAULT 0,
                       stats TEXT NOT NULL
);