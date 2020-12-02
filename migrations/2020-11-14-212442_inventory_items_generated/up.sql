CREATE TABLE inventory_items_generated (
                       inventory_id VARCHAR(50) NOT NULL,
                       items_generated_id VARCHAR(50) NOT NULL,
                       posx INT(3) NOT NULL,
                       posy INT(3) NOT NULL,
                       PRIMARY KEY(inventory_id,items_generated_id)
);