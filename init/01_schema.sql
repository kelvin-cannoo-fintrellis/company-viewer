-- =========================
-- EXTENSIONS
-- =========================
CREATE EXTENSION IF NOT EXISTS pg_trgm;
CREATE EXTENSION IF NOT EXISTS unaccent;

-- =========================
-- COMPANY (1 row per PDF)
-- =========================
CREATE TABLE company (
    id BIGSERIAL PRIMARY KEY,

    org_no TEXT,
    org_name TEXT NOT NULL,
    former_org_name TEXT,

    org_category_code TEXT,
    org_sub_category_code TEXT,
    org_type_code TEXT,
    org_nature_code TEXT,

    status TEXT,
    company_address TEXT,

    incorp_date DATE,
    effective_start_date DATE,
    defunct_date DATE,

    file_no TEXT,
    filename TEXT,

    created_at TIMESTAMP WITHOUT TIME ZONE DEFAULT now()
);

-- =========================
-- BUSINESS ACTIVITIES
-- =========================
CREATE TABLE business_activity (
    id BIGSERIAL PRIMARY KEY,
    company_id BIGINT NOT NULL REFERENCES company(id) ON DELETE CASCADE,

    bus_file_no TEXT,
    business_reg_no TEXT,
    business_name TEXT,

    business_nature TEXT NOT NULL,
    business_type TEXT,
    main_address TEXT,
    status TEXT
);

-- =========================
-- OFFICE BEARERS
-- =========================
CREATE TABLE office_bearer (
    id BIGSERIAL PRIMARY KEY,
    company_id BIGINT NOT NULL REFERENCES company(id) ON DELETE CASCADE,

    name TEXT NOT NULL,
    position TEXT,
    entity_type TEXT,

    appointed_date TEXT,
    address TEXT
);

-- =========================
-- SHAREHOLDERS
-- =========================
CREATE TABLE shareholder (
    id BIGSERIAL PRIMARY KEY,
    company_id BIGINT NOT NULL REFERENCES company(id) ON DELETE CASCADE,

    name TEXT NOT NULL,
    entity_type TEXT,

    num_shares NUMERIC,
    share_type TEXT,
    currency TEXT
);

-- =========================
-- SEARCH DOCUMENT (DENORMALIZED)
-- =========================
CREATE TABLE search_document (
    company_id BIGINT PRIMARY KEY REFERENCES company(id) ON DELETE CASCADE,

    org_name TEXT,
    former_org_name TEXT,

    business_natures_text TEXT,
    office_bearers_text TEXT,
    shareholders_text TEXT,
    addresses_text TEXT
);
