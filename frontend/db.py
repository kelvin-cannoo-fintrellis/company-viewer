import sqlite3
from pathlib import Path

DB_PATH = Path("companies.db")

def get_conn():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def init_db():
    conn = get_conn()
    cur = conn.cursor()

    cur.execute("""
    CREATE TABLE IF NOT EXISTS company (
        id INTEGER PRIMARY KEY,
        name TEXT,
        industry TEXT,
        pdf_path TEXT
    )
    """)

    cur.execute("""
    CREATE VIRTUAL TABLE IF NOT EXISTS company_fts
    USING fts5(name, industry, content='company', content_rowid='id')
    """)

    conn.commit()
    conn.close()

