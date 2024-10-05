-- Table: public.collection

-- DROP TABLE IF EXISTS public.collection;

CREATE TABLE collection (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    singleton BOOLEAN NOT NULL DEFAULT FALSE
);


ALTER TABLE IF EXISTS public.collection
    OWNER to postgres;

--REVOKE ALL ON TABLE public.collection FROM MiloCMSUser; What is this? it doesnt seem to do anything

GRANT DELETE, INSERT, SELECT, UPDATE ON TABLE public.collection TO "MiloCMSUser";

-- Somehow one of these made it so I could actually use the insert command on the collection table

-- Grant insert permission on the table
GRANT INSERT ON TABLE public.collection TO "MiloCMSUser";

-- Grant select permission on the id column, if using RETURNING
GRANT SELECT ON TABLE public.collection TO "MiloCMSUser";

-- Grant permission on the sequence if using an auto-incrementing id
GRANT USAGE, SELECT ON SEQUENCE public.collection_id_seq TO "MiloCMSUser";

-- Optionally, grant schema-level permissions if necessary
GRANT USAGE ON SCHEMA public TO "MiloCMSUser";

--

GRANT ALL ON TABLE public.collection TO postgres;

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

