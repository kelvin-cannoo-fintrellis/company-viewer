import json
from db import get_conn, init_db

def import_json(path):
    init_db()
    conn = get_conn()
    cur = conn.cursor()

    with open(path, "r", encoding="utf-8") as f:
        companies = json.load(f)

    for c in companies:
        cur.execute(
            "INSERT INTO company (name, industry, pdf_path) VALUES (?, ?, ?)",
            (c["name"], c["industry"], c["pdf"])
        )
        rowid = cur.lastrowid
        cur.execute(
            "INSERT INTO company_fts (rowid, name, industry) VALUES (?, ?, ?)",
            (rowid, c["name"], c["industry"])
        )

    conn.commit()
    conn.close()

