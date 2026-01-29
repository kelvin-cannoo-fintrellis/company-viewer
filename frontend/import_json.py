import json
import os
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
        former_name TEXT,
        address TEXT,
        category TEXT,
        status TEXT,
        incorp_date TEXT,
        file_no TEXT UNIQUE
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
        former_name,
        address,
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

    CREATE INDEX idx_office_bearer_country ON office_bearer(country);
    CREATE INDEX idx_company_name ON company(org_name);
    """)

    conn.commit()


def load_json(path):
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def import_directory(json_dir):
    conn = sqlite3.connect(DB_PATH)
    cur = conn.cursor()

    init_db(conn)

    inserted_companies = 0
    inserted_bearers = 0

    json_files = list(Path(json_dir).glob("*.json"))

    if not json_files:
        sys.exit(f"❌ No JSON files found in {json_dir}")

    print(f"📂 Found {len(json_files)} JSON files")

    conn.execute("BEGIN")

    for file_path in json_files:
        try:
            data = load_json(file_path)

            company = data.get("companyDetails", {})
            office_bearers = data.get("officeBearers", [])

            org_name = company.get("orgName")
            former_name = company.get("formerOrgName")
            address = company.get("companyAddress")
            category = company.get("categoryDesc")
            status = company.get("orgLastStaCd")
            incorp_date = company.get("orgIncorpDate")
            file_no = company.get("orgFileNo") or data.get("filename")

            if not org_name:
                continue

            # Insert company
            cur.execute("""
                INSERT OR IGNORE INTO company
                (org_name, former_name, address, category, status, incorp_date, file_no)
                VALUES (?, ?, ?, ?, ?, ?, ?)
            """, (
                org_name, former_name, address, category,
                status, incorp_date, file_no
            ))

            company_id = cur.lastrowid

            # FTS index
            cur.execute("""
                INSERT INTO company_fts (rowid, org_name, former_name, address)
                VALUES (?, ?, ?, ?)
            """, (company_id, org_name, former_name, address))

            inserted_companies += 1

            # Insert office bearers
            for ob in office_bearers:
                name = ob.get("name")
                country = ob.get("country")
                position = ob.get("position")
                ob_address = ob.get("address")
                appointed = ob.get("appointedDate")

                if not name:
                    continue

                cur.execute("""
                    INSERT INTO office_bearer
                    (company_id, name, country, position, address, appointed_date)
                    VALUES (?, ?, ?, ?, ?, ?)
                """, (
                    company_id, name, country,
                    position, ob_address, appointed
                ))

                bearer_id = cur.lastrowid

                cur.execute("""
                    INSERT INTO office_bearer_fts
                    (rowid, name, country, position)
                    VALUES (?, ?, ?, ?)
                """, (bearer_id, name, country, position))

                inserted_bearers += 1

        except Exception as e:
            print(f"⚠️ Failed {file_path.name}: {e}")

    conn.commit()
    conn.close()

    print(f"✅ Companies inserted: {inserted_companies}")
    print(f"✅ Office bearers inserted: {inserted_bearers}")
    print(f"📦 Database built: {DB_PATH}")


if __name__ == "__main__":
    if len(sys.argv) < 2:
        sys.exit("Usage: uv run python import_json.py <json_directory>")

    import_directory(sys.argv[1])
