-- =========================
-- COMPANY INDEXES
-- =========================

-- Exact / filter lookups
CREATE INDEX idx_company_org_no
    ON company(org_no);

CREATE INDEX idx_company_status
    ON company(status);

CREATE INDEX idx_company_incorp_date
    ON company(incorp_date);

-- Full-text search
CREATE INDEX idx_company_name_fts
    ON company
    USING GIN (
        to_tsvector(
            'english',
            unaccent(coalesce(org_name, ''))
        )
    );

CREATE INDEX idx_company_former_name_fts
    ON company
    USING GIN (
        to_tsvector(
            'english',
            unaccent(coalesce(former_org_name, ''))
        )
    );

-- Trigram (fast ILIKE / partial matches)
CREATE INDEX idx_company_name_trgm
    ON company
    USING GIN (org_name gin_trgm_ops);

-- =========================
-- BUSINESS ACTIVITY INDEXES
-- =========================

CREATE INDEX idx_business_activity_company_id
    ON business_activity(company_id);

-- 🔥 Core business search
CREATE INDEX idx_business_nature_fts
    ON business_activity
    USING GIN (
        to_tsvector(
            'english',
            unaccent(coalesce(business_nature, ''))
        )
    );

-- Optional partial matching
CREATE INDEX idx_business_nature_trgm
    ON business_activity
    USING GIN (business_nature gin_trgm_ops);

-- =========================
-- OFFICE BEARER INDEXES
-- =========================

CREATE INDEX idx_office_bearer_company_id
    ON office_bearer(company_id);

CREATE INDEX idx_office_bearer_name_fts
    ON office_bearer
    USING GIN (
        to_tsvector(
            'english',
            unaccent(coalesce(name, ''))
        )
    );

CREATE INDEX idx_office_bearer_name_trgm
    ON office_bearer
    USING GIN (name gin_trgm_ops);

-- =========================
-- SHAREHOLDER INDEXES
-- =========================

CREATE INDEX idx_shareholder_company_id
    ON shareholder(company_id);

CREATE INDEX idx_shareholder_name_fts
    ON shareholder
    USING GIN (
        to_tsvector(
            'english',
            unaccent(coalesce(name, ''))
        )
    );

CREATE INDEX idx_shareholder_name_trgm
    ON shareholder
    USING GIN (name gin_trgm_ops);

-- =========================
-- SEARCH DOCUMENT INDEXES
-- =========================

-- 🔥 Primary full-text search index
CREATE INDEX idx_search_document_fts
    ON search_document
    USING GIN (
        to_tsvector(
            'english',
            unaccent(
                coalesce(org_name,'') || ' ' ||
                coalesce(former_org_name,'') || ' ' ||
                coalesce(business_natures_text,'') || ' ' ||
                coalesce(office_bearers_text,'') || ' ' ||
                coalesce(shareholders_text,'') || ' ' ||
                coalesce(addresses_text,'')
            )
        )
    );
