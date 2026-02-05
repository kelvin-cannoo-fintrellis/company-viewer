import sys
from PySide6.QtWidgets import (
    QApplication, QWidget, QVBoxLayout, QHBoxLayout,
    QLineEdit, QPushButton, QTableWidget,
    QTableWidgetItem, QLabel, QComboBox, QMessageBox,
    QGroupBox, QCheckBox
)
from db import get_conn


class App(QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("Company & Director Search")
        self.resize(1250, 650)

        layout = QVBoxLayout(self)

        # Status label
        self.status = QLabel("Ready")
        layout.addWidget(self.status)

        # ========== COMPANY SEARCH ==========
        company_group = QGroupBox("Company Search")
        company_layout = QHBoxLayout()

        self.company_search = QLineEdit()
        self.company_search.setPlaceholderText("Search company name (leave empty to list all)")
        self.company_search.returnPressed.connect(self.search_companies)
        company_layout.addWidget(self.company_search)

        self.category_filter = QComboBox()
        self.category_filter.addItems([
            "All Categories",
            "DOMESTIC",
            "GLOBAL",
            "FOREIGN(DOM BRANCH)"
        ])
        company_layout.addWidget(self.category_filter)

        # Status filter dropdown
        self.status_filter = QComboBox()
        self.status_filter.addItems([
            "All Status",
            "Live",
            "Defunct",
            "Dissolved"
        ])
        company_layout.addWidget(self.status_filter)

        # Date range filters
        self.date_from = QLineEdit()
        self.date_from.setPlaceholderText("From Date (DD/MM/YYYY)")
        company_layout.addWidget(self.date_from)

        self.date_to = QLineEdit()
        self.date_to.setPlaceholderText("To Date (DD/MM/YYYY)")
        company_layout.addWidget(self.date_to)

        self.company_btn = QPushButton("Search Companies")
        self.company_btn.clicked.connect(self.search_companies)
        company_layout.addWidget(self.company_btn)

        company_group.setLayout(company_layout)
        layout.addWidget(company_group)

        # ========== DIRECTOR SEARCH ==========
        director_group = QGroupBox("Director Search")
        director_layout = QHBoxLayout()

        self.director_type = QComboBox()
        self.director_type.addItems(["By Name", "By Country"])
        director_layout.addWidget(self.director_type)

        self.director_search = QLineEdit()
        self.director_search.setPlaceholderText("Search directors... (supports: not like mauritius)")
        self.director_search.returnPressed.connect(self.search_directors)
        director_layout.addWidget(self.director_search)

        self.director_btn = QPushButton("Search Directors")
        self.director_btn.clicked.connect(self.search_directors)
        director_layout.addWidget(self.director_btn)

        director_group.setLayout(director_layout)
        layout.addWidget(director_group)

        # ========== RESULTS TABLE ==========
        self.table = QTableWidget()
        layout.addWidget(self.table)

        self.db_ok = self.check_db()

    # ================= DB CHECK =================
    def check_db(self):
        try:
            conn = get_conn()
            cur = conn.cursor()
            cur.execute("SELECT COUNT(*) FROM company")
            count = cur.fetchone()[0]
            conn.close()

            self.status.setText(f"Database loaded — {count:,} companies")
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
        category = self.category_filter.currentText()
        status_filter = self.status_filter.currentText()
        date_from = self.date_from.text().strip()
        date_to = self.date_to.text().strip()

        sql = """
        SELECT 
            id,
            org_name,
            org_no,
            org_last_status_code,
            org_incorp_date,
            org_type_code,
            category_desc,
            company_address,
            former_org_name
        FROM company
        WHERE 1=1
        """
        params = []

        # Name search
        if query:
            sql += " AND (org_name LIKE ? OR former_org_name LIKE ?)"
            params += [f"%{query}%", f"%{query}%"]

        # Category filter
        if category != "All Categories":
            sql += " AND org_category_code = ?"
            params.append(category)

        # Status filter logic
        if status_filter == "Live":
            sql += " AND (UPPER(org_last_status_code) LIKE '%ACTIVE%' OR UPPER(org_last_status_code) = 'LIVE')"
        elif status_filter == "Defunct":
            sql += " AND UPPER(org_last_status_code) LIKE '%DEFUNCT%'"
        elif status_filter == "Dissolved":
            sql += " AND UPPER(org_last_status_code) LIKE '%DISSOLVED%'"

        # ================= DATE RANGE FILTER =================
        # Convert DD/MM/YYYY to YYYYMMDD for correct ordering
        def normalize_date(expr):
            return f"substr({expr}, 7, 4) || substr({expr}, 4, 2) || substr({expr}, 1, 2)"

        if date_from:
            sql += f" AND {normalize_date('org_incorp_date')} >= ?"
            params.append(date_from[6:10] + date_from[3:5] + date_from[0:2])

        if date_to:
            sql += f" AND {normalize_date('org_incorp_date')} <= ?"
            params.append(date_to[6:10] + date_to[3:5] + date_to[0:2])

        sql += " ORDER BY org_name"

        conn = get_conn()
        cur = conn.cursor()
        cur.execute(sql, params)
        rows = cur.fetchall()
        conn.close()

        headers = [
            "ID",
            "Company Name",
            "Registration No",
            "Status",
            "Incorporation Date",
            "Company Type",
            "Category",
            "Address",
            "Former Name"
        ]

        self.populate_table(rows, headers)
        self.status.setText(f"{len(rows)} company result(s).")

    # ================= DIRECTOR SEARCH =================
    def search_directors(self):
        if not self.db_ok:
            return

        raw_query = self.director_search.text().strip()
        if not raw_query:
            QMessageBox.warning(self, "Input Needed", "Enter director search text")
            return

        mode = self.director_type.currentText()
        negate = False
        query = raw_query

        # Detect "not like"
        if raw_query.lower().startswith("not like "):
            negate = True
            query = raw_query[9:].strip()

        base_sql = """
        SELECT c.org_name, ob.name, ob.country, ob.position, ob.address
        FROM office_bearer ob
        JOIN company c ON c.id = ob.company_id
        WHERE UPPER(ob.position) = 'DIRECTOR'
        """

        column = "ob.name"
        if mode == "By Country":
            column = "ob.country"

        if negate:
            sql = base_sql + f" AND {column} NOT LIKE ? ORDER BY ob.name"
        else:
            sql = base_sql + f" AND {column} LIKE ? ORDER BY ob.name"

        conn = get_conn()
        cur = conn.cursor()
        cur.execute(sql, (f"%{query}%",))
        rows = cur.fetchall()
        conn.close()

        headers = ["Company", "Director Name", "Country", "Position", "Address"]
        self.populate_table(rows, headers)

    # ================= TABLE POPULATOR =================
    def populate_table(self, rows, headers):
        self.table.clear()
        self.table.setRowCount(0)
        self.table.setColumnCount(len(headers))
        self.table.setHorizontalHeaderLabels(headers)

        if not rows:
            self.status.setText("No results found")
            return

        self.status.setText(f"{len(rows)} result(s) found")
        self.table.setRowCount(len(rows))

        for r, row in enumerate(rows):
            for c in range(len(headers)):
                self.table.setItem(r, c, QTableWidgetItem(str(row[c] or "")))


if __name__ == "__main__":
    app = QApplication(sys.argv)
    window = App()
    window.show()
    sys.exit(app.exec())
