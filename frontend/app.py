import sys
import os
from PySide6.QtWidgets import (
    QApplication, QWidget, QVBoxLayout,
    QLineEdit, QPushButton, QTableWidget,
    QTableWidgetItem
)
from db import get_conn, init_db

class App(QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("Company Search")
        self.resize(700, 400)

        self.layout = QVBoxLayout(self)

        self.search = QLineEdit()
        self.search.setPlaceholderText("Search companies...")
        self.layout.addWidget(self.search)

        self.btn = QPushButton("Search")
        self.btn.clicked.connect(self.run_search)
        self.layout.addWidget(self.btn)

        self.table = QTableWidget(0, 3)
        self.table.setHorizontalHeaderLabels(["Name", "Industry", "PDF"])
        self.table.cellDoubleClicked.connect(self.open_pdf)
        self.layout.addWidget(self.table)

        init_db()

    def run_search(self):
        query = self.search.text()
        conn = get_conn()
        cur = conn.cursor()

        cur.execute("""
        SELECT c.name, c.industry, c.pdf_path
        FROM company_fts f
        JOIN company c ON c.id = f.rowid
        WHERE company_fts MATCH ?
        """, (query,))

        rows = cur.fetchall()
        conn.close()

        self.table.setRowCount(len(rows))
        for r, row in enumerate(rows):
            self.table.setItem(r, 0, QTableWidgetItem(row["name"]))
            self.table.setItem(r, 1, QTableWidgetItem(row["industry"]))
            self.table.setItem(r, 2, QTableWidgetItem(row["pdf_path"]))

    def open_pdf(self, row, col):
        path = self.table.item(row, 2).text()
        if os.path.exists(path):
            os.startfile(path)

if __name__ == "__main__":
    app = QApplication(sys.argv)
    window = App()
    window.show()
    sys.exit(app.exec())

