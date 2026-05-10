CREATE TABLE IF NOT EXISTS models (
    uuid TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    fields TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS assets (
    uuid TEXT PRIMARY KEY NOT NULL,
    model_id TEXT NOT NULL,
    name TEXT NOT NULL,
    fields TEXT NOT NULL,
    FOREIGN KEY (model_id) REFERENCES models (uuid)
);
