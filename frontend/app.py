import sys
from PySide6.QtWidgets import (
    QApplication, QWidget, QVBoxLayout, QHBoxLayout,
    QLineEdit, QPushButton, QTableWidget,
    QTableWidgetItem, QLabel, QComboBox, QMessageBox, QGroupBox
)
from db import get_conn


class App(QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("Company & Director Search")
        self.resize(1000, 560)

        layout = QVBoxLayout(self)

        # Status label
        self.status = QLabel("Ready")
        layout.addWidget(self.status)

        # ========== COMPANY SEARCH ==========
        company_group = QGroupBox("Company Search")
        company_layout = QHBoxLayout()

        self.company_search = QLineEdit()
        self.company_search.setPlaceholderText("Search company name...")
        self.company_search.returnPressed.connect(self.search_companies)
        company_layout.addWidget(self.company_search)

        self.company_btn = QPushButton("Search Companies")
        self.company_btn.clicked.connect(self.search_companies)
        company_layout.addWidget(self.company_btn)

        company_group.setLayout(company_layout)
        layout.addWidget(company_group)

        # ========== DIRECTOR SEARCH ==========
        director_group = QGroupBox("Director Search")
        director_layout = QHBoxLayout()

        self.director_type = QComboBox()
        self.director_type.addItems([
            "By Name",
            "By Country",
            "By Address"
        ])
        director_layout.addWidget(self.director_type)

        self.director_search = QLineEdit()
        self.director_search.setPlaceholderText("Search directors...")
        self.director_search.returnPressed.connect(self.search_directors)
        director_layout.addWidget(self.director_search)

        self.director_btn = QPushButton("Search Directors")
        self.director_btn.clicked.connect(self.search_directors)
        director_layout.addWidget(self.director_btn)

        director_group.setLayout(director_layout)
        layout.addWidget(director_group)

        # ========== RESULTS TABLE ==========
        self.table = QTableWidget(0, 5)
        self.table.setHorizontalHeaderLabels([
            "Company",
            "Director Name",
            "Country",
            "Position",
            "Address"
        ])
        layout.addWidget(self.table)

        # DB check
        self.db_ok = self.check_db()

    def check_db(self):
        try:
            conn = get_conn()
            conn.execute("SELECT 1")
            conn.close()
            self.status.setText("Database loaded")
            return True
        except Exception as e:
            QMessageBox.critical(self, "Database Error", str(e))
            self.company_btn.setEnabled(False)
            self.director_btn.setEnabled(False)
            self.status.setText("Database missing or invalid")
            return False

    # ================= COMPANY SEARCH =================
    def search_companies(self):
        if not self.db_ok:
            return

        query = self.company_search.text().strip()

        if not query:
            QMessageBox.warning(self, "Input Needed", "Enter a company name")
            return

        conn = get_conn()
        cur = conn.cursor()

        cur.execute("""
        SELECT c.org_name, NULL, NULL, NULL, NULL
        FROM company c
        WHERE c.org_name LIKE ?
           OR c.former_name LIKE ?
        ORDER BY c.org_name
        LIMIT 300
        """, (f"%{query}%", f"%{query}%"))

        rows = cur.fetchall()
        conn.close()

        self.populate_table(rows)

    # ================= DIRECTOR SEARCH =================
    def search_directors(self):
        if not self.db_ok:
            return

        query = self.director_search.text().strip()

        if not query:
            QMessageBox.warning(self, "Input Needed", "Enter director search text")
            return

        mode = self.director_type.currentText()

        conn = get_conn()
        cur = conn.cursor()

        base_sql = """
        SELECT c.org_name, ob.name, ob.country, ob.position, ob.address
        FROM office_bearer ob
        JOIN company c ON c.id = ob.company_id
        WHERE UPPER(ob.position) = 'DIRECTOR'
        """

        if mode == "By Name":
            sql = base_sql + " AND ob.name LIKE ? ORDER BY ob.name LIMIT 300"
            params = (f"%{query}%",)

        elif mode == "By Country":
            sql = base_sql + " AND ob.country LIKE ? ORDER BY ob.name LIMIT 300"
            params = (f"%{query}%",)

        else:  # Address
            sql = base_sql + " AND ob.address LIKE ? ORDER BY ob.name LIMIT 300"
            params = (f"%{query}%",)

        cur.execute(sql, params)
        rows = cur.fetchall()
        conn.close()

        self.populate_table(rows)

    # ================= TABLE POPULATOR =================
    def populate_table(self, rows):
        self.table.setRowCount(0)

        if not rows:
            self.status.setText("No results found")
            return

        self.status.setText(f"{len(rows)} result(s) found")
        self.table.setRowCount(len(rows))

        for r, row in enumerate(rows):
            for c in range(5):
                self.table.setItem(r, c, QTableWidgetItem(str(row[c] or "")))


if __name__ == "__main__":
    app = QApplication(sys.argv)
    window = App()
    window.show()
    sys.exit(app.exec())
