import sqlite3
from pathlib import Path

DB_PATH = Path("companies.db")


def get_conn():
    # Prevent SQLite from auto-creating database
    if not DB_PATH.exists():
        raise FileNotFoundError(
            f"Database file not found: {DB_PATH.resolve()}"
        )

    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def init_db():
    # Only call this when BUILDING the database, not in the UI
    conn = sqlite3.connect(DB_PATH)
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
