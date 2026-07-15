CREATE TABLE IF NOT EXISTS assets (
    uuid TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    created_at INT NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INT NOT NULL DEFAULT (strftime('%s', 'now'))
);

CREATE TRIGGER IF NOT EXISTS assets_update_timestamp AFTER UPDATE ON assets BEGIN UPDATE assets SET updated_at = strftime('%s', 'now') WHERE uuid = NEW.uuid; END;

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

CREATE TRIGGER IF NOT EXISTS assets_fields_update_timestamp AFTER UPDATE ON assets_fields BEGIN UPDATE assets_fields SET updated_at = strftime('%s', 'now') WHERE uuid = NEW.uuid; END;

CREATE TABLE IF NOT EXISTS relations (
    uuid TEXT PRIMARY KEY NOT NULL,
    description TEXT NOT NULL,
    source_asset_uuid TEXT NOT NULL,
    target_asset_uuid TEXT NOT NULL,
    created_at INT NOT NULL DEFAULT (strftime('%s', 'now')),
    updated_at INT NOT NULL DEFAULT (strftime('%s', 'now')),
    FOREIGN KEY (source_asset_uuid) REFERENCES assets (uuid) ON DELETE CASCADE,
    FOREIGN KEY (target_asset_uuid) REFERENCES assets (uuid) ON DELETE CASCADE
);

CREATE TRIGGER IF NOT EXISTS relations_update_timestamp AFTER UPDATE ON relations BEGIN UPDATE relations SET updated_at = strftime('%s', 'now') WHERE uuid = NEW.uuid; END;
