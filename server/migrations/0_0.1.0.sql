CREATE TABLE nodes(
    id INTEGER PRIMARY KEY,
    position_x INTEGER,
    position_y INTEGER,
    type: INTEGER NOT NULL,
    data: BLOB NOT NULL
);

CREATE TABLE node_children(
    rowid INTEGER PRIMARY KEY,
    parent INTEGER NOT NULL,
    child INTEGER NOT NULL,

    FOREIGN KEY (parent)
        REFERENCES nodes (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,

    FOREIGN KEY (child)
        REFERENCES nodes (id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);
