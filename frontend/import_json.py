import json
import sys
import sqlite3
from pathlib import Path

DB_PATH = "companies.db"


def init_db(conn):
    cur = conn.cursor()

    cur.executescript("""
    DROP TABLE IF EXISTS company;
    DROP TABLE IF EXISTS office_bearer;
    DROP TABLE IF EXISTS company_fts;
    DROP TABLE IF EXISTS office_bearer_fts;

    CREATE TABLE company (
        id INTEGER PRIMARY KEY AUTOINCREMENT,

        org_name TEXT,
        former_org_name TEXT,

        org_no TEXT UNIQUE,
        org_file_no TEXT,

        category_desc TEXT,
        sub_category_desc TEXT,
        org_category_code TEXT,
        org_sub_category_code TEXT,

        company_address TEXT,

        org_last_status_code TEXT,
        org_type_code TEXT,

        org_nature_code TEXT,
        org_nature_cd_code TEXT,

        org_incorp_date TEXT,
        effective_start_date TEXT,
        defunct_date TEXT,

        total_comprehensive_income TEXT,
        winding_up_status TEXT
    );

    CREATE TABLE office_bearer (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        company_id INTEGER,
        name TEXT,
        country TEXT,
        position TEXT,
        address TEXT,
        appointed_date TEXT,
        FOREIGN KEY(company_id) REFERENCES company(id)
    );

    CREATE VIRTUAL TABLE company_fts USING fts5(
        org_name,
        former_org_name,
        company_address,
        content='company',
        content_rowid='id'
    );

    CREATE VIRTUAL TABLE office_bearer_fts USING fts5(
        name,
        country,
        position,
        content='office_bearer',
        content_rowid='id'
    );

    CREATE INDEX idx_company_name ON company(org_name);
    CREATE INDEX idx_office_bearer_country ON office_bearer(country);
    """)

    conn.commit()


def load_json(path):
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def import_directory(json_dir):
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    cur = conn.cursor()

    init_db(conn)

    inserted_companies = 0
    inserted_bearers = 0
    skipped_files = 0

    json_files = list(Path(json_dir).glob("*.json"))

    if not json_files:
        sys.exit(f"❌ No JSON files found in {json_dir}")

    print(f"📂 Found {len(json_files)} JSON files")
    print("⏳ Importing...")

    conn.execute("BEGIN")

    for file_path in json_files:
        try:
            data = load_json(file_path)

            company = data.get("companyDetails", {})
            office_bearers = data.get("officeBearers", [])

            # ---- Extract company fields ----
            org_name = company.get("orgName")
            if not org_name:
                continue

            former_org_name = company.get("formerOrgName")

            company_address = company.get("companyAddress")
            category_desc = company.get("categoryDesc")
            sub_category_desc = company.get("subCategoryDesc")

            org_category_code = company.get("orgCategoryCode")
            org_sub_category_code = company.get("orgSubCategoryCode")

            status = company.get("orgLastStaCd")
            incorp_date = company.get("orgIncorpDate")

            org_no = company.get("orgNo")
            org_file_no = company.get("orgFileNo")
            file_no = org_no or org_file_no or data.get("filename")

            effective_start_date = company.get("effectiveStartDate")
            defunct_date = company.get("defunctDate")

            org_type_code = company.get("orgTypeCd")

            org_nature_code = company.get("orgNatureCd")
            org_nature_cd_code = company.get("orgNatureCdCode")

            total_comprehensive_income = company.get("totalComprehensiveIncome")
            winding_up_status = company.get("windingUpStatus")

            # ---- Insert company ----
            cur.execute("""
                INSERT OR IGNORE INTO company (
                    org_name,
                    former_org_name,

                    org_no,
                    org_file_no,

                    category_desc,
                    sub_category_desc,
                    org_category_code,
                    org_sub_category_code,

                    company_address,

                    org_last_status_code,
                    org_type_code,

                    org_nature_code,
                    org_nature_cd_code,

                    org_incorp_date,
                    effective_start_date,
                    defunct_date,

                    total_comprehensive_income,
                    winding_up_status
                )
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """, (
                org_name,
                former_org_name,

                org_no,
                org_file_no,

                category_desc,
                sub_category_desc,
                org_category_code,
                org_sub_category_code,

                company_address,

                status,
                org_type_code,

                org_nature_code,
                org_nature_cd_code,

                incorp_date,
                effective_start_date,
                defunct_date,

                total_comprehensive_income,
                winding_up_status
            ))

            # --- Resolve company_id reliably ---
            if cur.lastrowid:
                company_id = cur.lastrowid
            else:
                cur.execute("""
                    SELECT id FROM company
                    WHERE org_no = ? OR org_file_no = ?
                    LIMIT 1
                """, (org_no, org_file_no))
                row = cur.fetchone()
                if not row:
                    continue
                company_id = row["id"]

            # ---- Insert Company FTS ----
            cur.execute("""
                INSERT OR IGNORE INTO company_fts
                (rowid, org_name, former_org_name, company_address)
                VALUES (?, ?, ?, ?)
            """, (company_id, org_name, former_org_name, company_address))

            inserted_companies += 1

            # ---- Insert Office Bearers ----
            for ob in office_bearers:
                name = ob.get("name")
                if not name:
                    continue

                country = ob.get("country")
                position = ob.get("position")
                ob_address = ob.get("address")
                appointed = ob.get("appointedDate")

                cur.execute("""
                    INSERT OR IGNORE INTO office_bearer
                    (company_id, name, country, position, address, appointed_date)
                    VALUES (?, ?, ?, ?, ?, ?)
                """, (
                    company_id,
                    name,
                    country,
                    position,
                    ob_address,
                    appointed
                ))

                # Resolve bearer ID safely
                cur.execute("""
                    SELECT id FROM office_bearer
                    WHERE company_id = ? AND name = ? AND position = ?
                """, (company_id, name, position))

                bearer_row = cur.fetchone()
                if not bearer_row:
                    continue

                bearer_id = bearer_row["id"]

                # Insert FTS row
                cur.execute("""
                    INSERT OR IGNORE INTO office_bearer_fts
                    (rowid, name, country, position)
                    VALUES (?, ?, ?, ?)
                """, (bearer_id, name, country, position))

                inserted_bearers += 1

        except Exception as e:
            skipped_files += 1
            print(f"⚠️ Failed {file_path.name}: {e}")

    conn.commit()
    conn.close()

    print("\n✅ Import complete")
    print(f"🏢 Companies inserted: {inserted_companies}")
    print(f"👥 Office bearers inserted: {inserted_bearers}")
    print(f"⚠️ Skipped files: {skipped_files}")
    print(f"📦 Database built: {DB_PATH}")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        sys.exit("Usage: uv run python import_json.py <json_directory>")

    import_directory(sys.argv[1])
