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


# ============================
# NORMALIZATION FUNCTIONS
# ============================

def clean_text(value):
    if value is None:
        return None
    if isinstance(value, str):
        value = value.strip()
        return value if value else None
    return value

def normalize_company_details(company):
    if not company:
        return None

    org_name = clean_text(company.get("orgName"))
    if not org_name:
        return None

    return {
        "org_name": org_name.upper(),
        "former_org_name": clean_text(company.get("formerOrgName")),

        "org_no": clean_text(company.get("orgNo")),
        "org_file_no": clean_text(company.get("orgFileNo")),

        "category_desc": clean_text(company.get("categoryDesc")),
        "sub_category_desc": clean_text(company.get("subCategoryDesc")),

        "org_category_code": clean_text(company.get("orgCategoryCode")),
        "org_sub_category_code": clean_text(company.get("orgSubCategoryCode")),

        "company_address": clean_text(company.get("companyAddress")),

        "org_last_status_code": clean_text(company.get("orgLastStaCd") or "").upper(),

        "org_type_code": clean_text(company.get("orgTypeCd")),

        "org_nature_code": clean_text(company.get("orgNatureCd")),
        "org_nature_cd_code": clean_text(company.get("orgNatureCdCode")),

        "org_incorp_date": clean_text(company.get("orgIncorpDate")),
        "effective_start_date": clean_text(company.get("effectiveStartDate")),
        "defunct_date": clean_text(company.get("defunctDate")),

        "total_comprehensive_income": clean_text(company.get("totalComprehensiveIncome")),
        "winding_up_status": clean_text(company.get("windingUpStatus")),
    }

def normalize_office_bearer_details(ob):
    if not ob:
        return None

    name = clean_text(ob.get("name"))
    if not name:
        return None

    # Remove "Director" word
    name = name.replace("Director", "").strip()

    # Capitalize country
    country = clean_text(ob.get("country")).upper()

    return {
        "name": name,
        "country": country,
        "position": clean_text(ob.get("position")),
        "address": clean_text(ob.get("address")),
        "appointed_date": clean_text(ob.get("appointedDate")),
    }



# ============================
# IMPORT FUNCTION
# ============================

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

            company_raw = data.get("companyDetails", {})
            office_bearers_raw = data.get("officeBearers", [])

            company = normalize_company_details(company_raw)
            if not company:
                continue

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
                company["org_name"],
                company["former_org_name"],

                company["org_no"],
                company["org_file_no"],

                company["category_desc"],
                company["sub_category_desc"],
                company["org_category_code"],
                company["org_sub_category_code"],

                company["company_address"],

                company["org_last_status_code"],
                company["org_type_code"],

                company["org_nature_code"],
                company["org_nature_cd_code"],

                company["org_incorp_date"],
                company["effective_start_date"],
                company["defunct_date"],

                company["total_comprehensive_income"],
                company["winding_up_status"]
            ))

            # Resolve company_id safely
            if cur.lastrowid:
                company_id = cur.lastrowid
            else:
                cur.execute("""
                    SELECT id FROM company
                    WHERE org_no = ? OR org_file_no = ?
                    LIMIT 1
                """, (company["org_no"], company["org_file_no"]))
                row = cur.fetchone()
                if not row:
                    continue
                company_id = row["id"]

            # Insert Company FTS
            cur.execute("""
                INSERT OR IGNORE INTO company_fts
                (rowid, org_name, former_org_name, company_address)
                VALUES (?, ?, ?, ?)
            """, (
                company_id,
                company["org_name"],
                company["former_org_name"],
                company["company_address"]
            ))

            inserted_companies += 1

            # ---- Insert Office Bearers ----
            for ob_raw in office_bearers_raw:
                ob = normalize_office_bearer_details(ob_raw)
                if not ob:
                    continue

                cur.execute("""
                    INSERT OR IGNORE INTO office_bearer
                    (company_id, name, country, position, address, appointed_date)
                    VALUES (?, ?, ?, ?, ?, ?)
                """, (
                    company_id,
                    ob["name"],
                    ob["country"],
                    ob["position"],
                    ob["address"],
                    ob["appointed_date"]
                ))

                cur.execute("""
                    SELECT id FROM office_bearer
                    WHERE company_id = ? AND name = ? AND position = ?
                """, (company_id, ob["name"], ob["position"]))

                bearer_row = cur.fetchone()
                if not bearer_row:
                    continue

                bearer_id = bearer_row["id"]

                # Insert FTS row
                cur.execute("""
                    INSERT OR IGNORE INTO office_bearer_fts
                    (rowid, name, country, position)
                    VALUES (?, ?, ?, ?)
                """, (
                    bearer_id,
                    ob["name"],
                    ob["country"],
                    ob["position"]
                ))

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
