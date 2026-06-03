CREATE TABLE IF NOT EXISTS models (
    uuid TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    fields TEXT NOT NULL,
    created_at INT NOT NULL DEFAULT strftime("%s", 'now'),
    updated_at INT NOT NULL DEFAULT strftime("%s", 'now'),
);
CREATE TABLE IF NOT EXISTS assets (
    uuid TEXT PRIMARY KEY NOT NULL,
    model_id TEXT,
    name TEXT NOT NULL,
    fields TEXT NOT NULL,
    created_at INT NOT NULL DEFAULT strftime("%s", 'now'),
    updated_at INT NOT NULL DEFAULT strftime("%s", 'now'),
    FOREIGN KEY (model_id) REFERENCES models (uuid) ON DELETE SET NULL
);
