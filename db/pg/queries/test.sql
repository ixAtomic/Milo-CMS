-- Drop tables if they already exist to ensure the script runs cleanly
DROP TABLE IF EXISTS fields;
DROP TABLE IF EXISTS relationships;
DROP TABLE IF EXISTS admin_configurations;
DROP TABLE IF EXISTS collection;

-- Create the collections table
CREATE TABLE collection (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    singleton BOOLEAN NOT NULL DEFAULT FALSE
);


ALTER TABLE IF EXISTS public.collection
    OWNER to postgres;

--REVOKE ALL ON TABLE public.collection FROM MiloCMSUser; What is this? it doesnt seem to do anything

GRANT DELETE, INSERT, SELECT, UPDATE ON TABLE public.collection TO "MiloCMSUser";

GRANT ALL ON TABLE public.collection TO postgres;

-- Create the admin_configurations table
CREATE TABLE admin_configurations (
    id SERIAL PRIMARY KEY,
    readonly BOOLEAN NOT NULL DEFAULT FALSE,
    required BOOLEAN NOT NULL DEFAULT FALSE,
    hidden BOOLEAN NOT NULL DEFAULT FALSE,
    note TEXT
);

ALTER TABLE IF EXISTS public.admin_configurations
    OWNER to postgres;

GRANT DELETE, INSERT, SELECT, UPDATE ON TABLE public.admin_configurations TO "MiloCMSUser";

GRANT ALL ON TABLE public.collection TO postgres;

-- Create the relationships table
CREATE TABLE relationships (
    id SERIAL PRIMARY KEY,
    relation_table VARCHAR(255) NOT NULL,
    relation_field VARCHAR(255) NOT NULL,
    delete_event VARCHAR(50) NOT NULL CHECK (delete_event IN ('Cascade', 'Nullify', 'Orphan'))
);

ALTER TABLE IF EXISTS public.relationships
    OWNER to postgres;

GRANT DELETE, INSERT, SELECT, UPDATE ON TABLE public.relationships TO "MiloCMSUser";

GRANT ALL ON TABLE public.collection TO postgres;

-- Create the fields table
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

ALTER TABLE IF EXISTS public.fields
    OWNER to postgres;

GRANT DELETE, INSERT, SELECT, UPDATE ON TABLE public.fields TO "MiloCMSUser";

GRANT ALL ON TABLE public.collection TO postgres;

-- Insert test data into the collections table
INSERT INTO collection (name, singleton) VALUES 
('User Collection', FALSE),
('Settings Collection', TRUE);

-- Insert test data into the admin_configurations table
INSERT INTO admin_configurations (readonly, required, hidden, note) VALUES 
(FALSE, TRUE, FALSE, 'User name must be unique'),
(TRUE, FALSE, TRUE, 'This setting is read-only');

-- Insert test data into the relationships table
INSERT INTO relationships (relation_table, relation_field, delete_event) VALUES 
('orders', 'user_id', 'Cascade'),
('profiles', 'user_id', 'Nullify');

-- Insert test data into the fields table
INSERT INTO fields (collection, name, default_value, nullable, is_unique, admin_configuration_id, relationship_id) VALUES 
(1, 'username', 'guest', FALSE, TRUE, 1, 2),
(1, 'email', NULL, FALSE, TRUE, 1, NULL),
(2, 'theme', 'light', TRUE, FALSE, 2, NULL);

-- Query to test the data retrieval
SELECT 
    f.id AS field_id, 
    c.name AS collection_name, 
    f.name AS field_name, 
    f.default_value, 
    f.nullable, 
    f.is_unique, 
    ac.readonly, 
    ac.required, 
    ac.hidden, 
    ac.note, 
    r.relation_table, 
    r.relation_field, 
    r.delete_event 
FROM 
    fields f
LEFT JOIN 
    collection c ON f.collection = c.id
LEFT JOIN 
    admin_configurations ac ON f.admin_configuration_id = ac.id
LEFT JOIN 
    relationships r ON f.relationship_id = r.id;
