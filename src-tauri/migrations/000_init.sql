CREATE TABLE IF NOT EXISTS assets (
    uuid TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    created_at INT NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INT NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS assets_fields (
    uuid TEXT PRIMARY KEY NOT NULL,
    asset_id TEXT NOT NULL,
    name TEXT NOT NULL,
    kind TEXT NOT NULL,
    text_value TEXT,
    number_value REAL,
    date_value TEXT,
    created_at INT NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INT NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (asset_id) REFERENCES assets (uuid) ON DELETE CASCADE
);
