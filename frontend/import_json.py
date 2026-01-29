import json
import sys
import argparse
from db import get_conn, init_db


def load_companies(path: str):
    try:
        with open(path, "r", encoding="utf-8") as f:
            data = json.load(f)

        if not isinstance(data, list):
            raise ValueError("JSON root must be a list")

        return data

    except FileNotFoundError:
        sys.exit(f"❌ File not found: {path}")

    except json.JSONDecodeError as e:
        sys.exit(f"❌ Invalid JSON: {e}")


def import_json(path: str, force=False):
    init_db()
    companies = load_companies(path)

    conn = get_conn()
    cur = conn.cursor()

    inserted = 0
    updated = 0
    skipped = 0

    try:
        conn.execute("BEGIN")

        for c in companies:
            name = c.get("name")
            industry = c.get("industry")
            pdf = c.get("pdf")

            if not name or not industry:
                print(f"⚠️ Skipping invalid entry: {c}")
                skipped += 1
                continue

            cur.execute("SELECT id FROM company WHERE name = ?", (name,))
            existing = cur.fetchone()

            # UPDATE if exists and force enabled
            if existing and force:
                company_id = existing[0]

                cur.execute(
                    """
                    UPDATE company
                    SET industry = ?, pdf_path = ?
                    WHERE id = ?
                    """,
                    (industry, pdf, company_id)
                )

                cur.execute(
                    """
                    UPDATE company_fts
                    SET name = ?, industry = ?
                    WHERE rowid = ?
                    """,
                    (name, industry, company_id)
                )

                updated += 1
                continue

            # Skip duplicates if no force
            if existing:
                skipped += 1
                continue

            # Insert new company
            cur.execute(
                """
                INSERT INTO company (name, industry, pdf_path)
                VALUES (?, ?, ?)
                """,
                (name, industry, pdf)
            )

            rowid = cur.lastrowid

            cur.execute(
                """
                INSERT INTO company_fts (rowid, name, industry)
                VALUES (?, ?, ?)
                """,
                (rowid, name, industry)
            )

            inserted += 1

        conn.commit()

        print(
            f"✅ Import complete — {inserted} added, "
            f"{updated} updated, {skipped} skipped"
        )

    except Exception as e:
        conn.rollback()
        sys.exit(f"❌ Import failed: {e}")

    finally:
        conn.close()


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Import companies.json into the database"
    )

    parser.add_argument(
        "file",
        nargs="?",
        default="companies.json",
        help="Path to JSON file (default: companies.json)"
    )

    parser.add_argument(
        "--force",
        action="store_true",
        help="Overwrite existing companies"
    )

    args = parser.parse_args()

    import_json(args.file, force=args.force)
