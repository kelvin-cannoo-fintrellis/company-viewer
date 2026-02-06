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
        self.resize(1250, 720)

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

        # ================= CATEGORY FILTER =================
        self.category_group = QGroupBox("Category")
        category_layout = QVBoxLayout()

        self.cat_domestic = QCheckBox("DOMESTIC")
        self.cat_global = QCheckBox("GLOBAL")
        self.cat_foreign_dom = QCheckBox("FOREIGN(DOM BRANCH)")
        self.cat_foreign_gbc = QCheckBox("FOREIGN(GBC BRANCH)")
        self.cat_other = QCheckBox("Other / Unknown")

        category_layout.addWidget(self.cat_domestic)
        category_layout.addWidget(self.cat_global)
        category_layout.addWidget(self.cat_foreign_dom)
        category_layout.addWidget(self.cat_foreign_gbc)
        category_layout.addWidget(self.cat_other)

        self.category_group.setLayout(category_layout)
        company_layout.addWidget(self.category_group)

        # ================= STATUS FILTER =================
        self.status_group = QGroupBox("Status")
        status_layout = QVBoxLayout()

        self.status_live = QCheckBox("Live")
        self.status_defunct = QCheckBox("Defunct")
        self.status_dissolved = QCheckBox("Dissolved")
        self.status_other = QCheckBox("Other / Unknown")

        status_layout.addWidget(self.status_live)
        status_layout.addWidget(self.status_defunct)
        status_layout.addWidget(self.status_dissolved)
        status_layout.addWidget(self.status_other)

        self.status_group.setLayout(status_layout)
        company_layout.addWidget(self.status_group)

        # ================= NATURE FILTER =================
        self.nature_group = QGroupBox("Nature")
        nature_layout = QVBoxLayout()

        self.nature_public = QCheckBox("Public")
        self.nature_private = QCheckBox("Private")
        self.nature_civil = QCheckBox("Civil")
        self.nature_other = QCheckBox("Other / Unknown")

        nature_layout.addWidget(self.nature_public)
        nature_layout.addWidget(self.nature_private)
        nature_layout.addWidget(self.nature_civil)
        nature_layout.addWidget(self.nature_other)

        self.nature_group.setLayout(nature_layout)
        company_layout.addWidget(self.nature_group)

        # ================= DATE FILTER =================
        date_group = QGroupBox("Incorporation Date")
        date_layout = QHBoxLayout()

        self.date_from = QLineEdit()
        self.date_from.setPlaceholderText("From (DD/MM/YYYY)")
        date_layout.addWidget(self.date_from)

        self.date_to = QLineEdit()
        self.date_to.setPlaceholderText("To (DD/MM/YYYY)")
        date_layout.addWidget(self.date_to)

        date_group.setLayout(date_layout)
        company_layout.addWidget(date_group)

        # Search button
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

        # ========== COMPANY TABLE ==========
        layout.addWidget(QLabel("Companies"))

        self.company_table = QTableWidget()
        self.company_table.setSelectionBehavior(QTableWidget.SelectRows)
        self.company_table.setEditTriggers(QTableWidget.NoEditTriggers)
        self.company_table.setSortingEnabled(True)
        self.company_table.itemSelectionChanged.connect(self.load_directors_for_selected_company)
        layout.addWidget(self.company_table)

        # ========== DIRECTORS TABLE ==========
        layout.addWidget(QLabel("Directors"))

        self.director_table = QTableWidget()
        self.director_table.setEditTriggers(QTableWidget.NoEditTriggers)
        self.director_table.setSortingEnabled(True)
        layout.addWidget(self.director_table)

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
        date_from = self.date_from.text().strip()
        date_to = self.date_to.text().strip()

        # TODO: Replace org_category_code with category_desc later once database issue is fixed
        sql = """
        SELECT 
            id,
            org_name,
            org_no,
            org_last_status_code,
            org_incorp_date,
            org_type_code,
            org_category_code,
            company_address,
            former_org_name,
            org_nature_code
        FROM company
        WHERE 1=1
        """
        params = []

        # Name search
        if query:
            sql += " AND (org_name LIKE ? OR former_org_name LIKE ?)"
            params += [f"%{query}%", f"%{query}%"]

        # ================= CATEGORY FILTER =================
        selected_categories = []

        if self.cat_domestic.isChecked():
            selected_categories.append("DOMESTIC")
        if self.cat_global.isChecked():
            selected_categories.append("GLOBAL")
        if self.cat_foreign_dom.isChecked():
            selected_categories.append("FOREIGN(DOM BRANCH)")
        if self.cat_foreign_gbc.isChecked():
            selected_categories.append("FOREIGN(GBC BRANCH)")

        include_cat_other = self.cat_other.isChecked()

        if selected_categories or include_cat_other:
            cat_conditions = []

            if selected_categories:
                placeholders = ",".join(["?"] * len(selected_categories))
                cat_conditions.append(f"UPPER(org_category_code) IN ({placeholders})")
                params.extend(selected_categories)

            if include_cat_other:
                cat_conditions.append("""
                    org_category_code IS NULL
                    OR TRIM(org_category_code) = ''
                    OR UPPER(org_category_code) NOT IN (
                        'DOMESTIC','GLOBAL','FOREIGN(DOM BRANCH)','FOREIGN(GBC BRANCH)'
                    )
                """)

            sql += " AND (" + " OR ".join(cat_conditions) + ")"

        # ================= STATUS FILTER =================
        selected_status = []

        if self.status_live.isChecked():
            selected_status.append("LIVE")
        if self.status_defunct.isChecked():
            selected_status.append("DEFUNCT")
        if self.status_dissolved.isChecked():
            selected_status.append("DISSOLVED")

        include_status_other = self.status_other.isChecked()

        if selected_status or include_status_other:
            status_conditions = []

            for s in selected_status:
                if s == "LIVE":
                    status_conditions.append("(UPPER(org_last_status_code) LIKE '%ACTIVE%' OR UPPER(org_last_status_code) = 'LIVE')")
                elif s == "DEFUNCT":
                    status_conditions.append("UPPER(org_last_status_code) LIKE '%DEFUNCT%'")
                elif s == "DISSOLVED":
                    status_conditions.append("UPPER(org_last_status_code) LIKE '%DISSOLVED%'")

            if include_status_other:
                status_conditions.append("""
                    org_last_status_code IS NULL
                    OR TRIM(org_last_status_code) = ''
                    OR (
                        UPPER(org_last_status_code) NOT LIKE '%ACTIVE%'
                        AND UPPER(org_last_status_code) NOT LIKE '%LIVE%'
                        AND UPPER(org_last_status_code) NOT LIKE '%DEFUNCT%'
                        AND UPPER(org_last_status_code) NOT LIKE '%DISSOLVED%'
                    )
                """)

            sql += " AND (" + " OR ".join(status_conditions) + ")"

        # ================= NATURE FILTER =================
        selected_natures = []

        if self.nature_public.isChecked():
            selected_natures.append("PUBLIC")
        if self.nature_private.isChecked():
            selected_natures.append("PRIVATE")
        if self.nature_civil.isChecked():
            selected_natures.append("CIVIL")

        include_nature_other = self.nature_other.isChecked()

        if selected_natures or include_nature_other:
            nature_conditions = []

            if selected_natures:
                placeholders = ",".join(["?"] * len(selected_natures))
                nature_conditions.append(f"UPPER(org_nature_code) IN ({placeholders})")
                params.extend(selected_natures)

            if include_nature_other:
                nature_conditions.append("""
                    org_nature_code IS NULL
                    OR TRIM(org_nature_code) = ''
                    OR UPPER(org_nature_code) NOT IN ('PUBLIC','PRIVATE','CIVIL')
                """)

            sql += " AND (" + " OR ".join(nature_conditions) + ")"

        # ================= DATE FILTER =================
        def normalize(expr):
            return f"substr({expr}, 7, 4) || substr({expr}, 4, 2) || substr({expr}, 1, 2)"

        if date_from:
            sql += f" AND {normalize('org_incorp_date')} >= ?"
            params.append(date_from[6:10] + date_from[3:5] + date_from[0:2])

        if date_to:
            sql += f" AND {normalize('org_incorp_date')} <= ?"
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
            "Former Name",
            "Nature"
        ]

        self.populate_company_table(rows, headers)
        self.director_table.clear()
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

        self.populate_director_table(rows, headers)
        self.status.setText(f"{len(rows)} director result(s).")

    # ================= LOAD DIRECTORS =================
    def load_directors_for_selected_company(self):
        selected = self.company_table.selectedItems()
        if not selected:
            return

        row = selected[0].row()
        company_id = self.company_table.item(row, 0).text()
        company_name = self.company_table.item(row, 1).text()

        self.load_directors(company_id, company_name)

    def load_directors(self, company_id, company_name):
        sql = """
        SELECT name, country, position, address
        FROM office_bearer
        WHERE company_id = ?
          AND UPPER(position) = 'DIRECTOR'
        ORDER BY name
        """

        conn = get_conn()
        cur = conn.cursor()
        cur.execute(sql, (company_id,))
        rows = cur.fetchall()
        conn.close()

        headers = ["Director Name", "Country", "Position", "Address"]
        self.populate_director_table(rows, headers)

        if rows:
            self.status.setText(f"{len(rows)} directors found for {company_name}")
        else:
            self.status.setText(f"No directors found for {company_name}")

    # ================= TABLE POPULATORS =================
    def populate_company_table(self, rows, headers):
        self.company_table.setSortingEnabled(False)
        self.company_table.clear()
        self.company_table.setRowCount(0)
        self.company_table.setColumnCount(len(headers))
        self.company_table.setHorizontalHeaderLabels(headers)

        if not rows:
            return

        self.company_table.setRowCount(len(rows))

        for r, row in enumerate(rows):
            for c in range(len(headers)):
                self.company_table.setItem(r, c, QTableWidgetItem(str(row[c] or "")))

        self.company_table.setSortingEnabled(True)

    def populate_director_table(self, rows, headers):
        self.director_table.setSortingEnabled(False)
        self.director_table.clear()
        self.director_table.setRowCount(0)
        self.director_table.setColumnCount(len(headers))
        self.director_table.setHorizontalHeaderLabels(headers)

        if not rows:
            return

        self.director_table.setRowCount(len(rows))

        for r, row in enumerate(rows):
            for c in range(len(headers)):
                self.director_table.setItem(r, c, QTableWidgetItem(str(row[c] or "")))

        self.director_table.setSortingEnabled(True)


if __name__ == "__main__":
    app = QApplication(sys.argv)
    window = App()
    window.show()
    sys.exit(app.exec())
