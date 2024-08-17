CREATE TABLE admin_configurations (
    id SERIAL PRIMARY KEY,
    readonly BOOLEAN NOT NULL DEFAULT FALSE,
    required BOOLEAN NOT NULL DEFAULT FALSE,
    hidden BOOLEAN NOT NULL DEFAULT FALSE,
    note TEXT
);

CREATE TABLE relationships (
    id SERIAL PRIMARY KEY,
    relation_table VARCHAR(255) NOT NULL,
    relation_field VARCHAR(255) NOT NULL,
    delete_event VARCHAR(50) NOT NULL CHECK (delete_event IN ('Cascade', 'Nullify', 'Orphan'))
);

CREATE TABLE fields (
    id SERIAL PRIMARY KEY,
    collection INTEGER NOT NULL REFERENCES collection(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    default_value TEXT,  -- Renamed to avoid SQL keyword conflict
    nullable BOOLEAN NOT NULL DEFAULT FALSE,
    is_unique BOOLEAN NOT NULL DEFAULT FALSE,
    admin_configuration_id INTEGER REFERENCES admin_configurations(id) ON DELETE SET NULL,
    relationship_id INTEGER REFERENCES relationships(id) ON DELETE SET NULL
);

