import sys
from PySide6.QtWidgets import (
    QApplication, QWidget, QVBoxLayout, QHBoxLayout,
    QLineEdit, QPushButton, QTableWidget,
    QTableWidgetItem, QLabel, QComboBox, QMessageBox
)
from db import get_conn


class App(QWidget):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("Company & Office Bearer Search")
        self.resize(900, 500)

        layout = QVBoxLayout(self)

        # Status label
        self.status = QLabel("Ready")
        layout.addWidget(self.status)

        # Search controls row
        controls = QHBoxLayout()

        self.search_type = QComboBox()
        self.search_type.addItems([
            "Company Name (Fuzzy)",
            "Office Bearer Name",
            "Office Bearer Country"
        ])
        controls.addWidget(self.search_type)

        self.search = QLineEdit()
        self.search.setPlaceholderText("Type search text...")
        self.search.returnPressed.connect(self.run_search)  # ENTER key triggers search
        self.search.setFocus() # auto-focus search bar on startup
        self.search.textChanged.connect(self.run_search) # live search while typing

        controls.addWidget(self.search)

        self.btn = QPushButton("Search")
        self.btn.clicked.connect(self.run_search)
        controls.addWidget(self.btn)

        layout.addLayout(controls)

        # Results table
        self.table = QTableWidget(0, 4)
        self.table.setHorizontalHeaderLabels([
            "Company",
            "Office Bearer",
            "Country",
            "Position"
        ])
        layout.addWidget(self.table)

        # Verify database exists on startup
        self.db_ok = self.check_db()

    def check_db(self):
        """Ensure database exists and is readable."""
        try:
            conn = get_conn()
            conn.execute("SELECT 1")
            conn.close()
            self.status.setText("Database loaded successfully")
            return True

        except Exception as e:
            QMessageBox.critical(
                self,
                "Database Error",
                "Database could not be loaded.\n\n"
                "Please ensure the database file exists.\n\n"
                f"Details:\n{e}"
            )

            self.status.setText("Database missing or invalid")
            self.btn.setEnabled(False)
            return False

    def run_search(self):
        if not self.db_ok:
            QMessageBox.warning(self, "Database Error", "Database is not available")
            return

        query = self.search.text().strip()

        if not query:
            # QMessageBox.warning(self, "Input Needed", "Please enter search text")
            return

        try:
            conn = get_conn()
            cur = conn.cursor()
        except Exception as e:
            QMessageBox.critical(self, "Database Error", f"Could not open DB:\n{e}")
            return

        mode = self.search_type.currentText()

        try:
            # 1️⃣ Company Name (FUZZY)
            if mode.startswith("Company Name"):
                cur.execute("""
                SELECT c.org_name, NULL, NULL, NULL
                FROM company c
                WHERE c.org_name LIKE ?
                   OR c.former_name LIKE ?
                ORDER BY c.org_name
                LIMIT 300
                """, (f"%{query}%", f"%{query}%"))

            # 2️⃣ Office Bearer Name (FUZZY)
            elif mode == "Office Bearer Name":
                cur.execute("""
                SELECT c.org_name, ob.name, ob.country, ob.position
                FROM office_bearer ob
                JOIN company c ON c.id = ob.company_id
                WHERE ob.name LIKE ?
                ORDER BY ob.name
                LIMIT 300
                """, (f"%{query}%",))

            # 3️⃣ Office Bearer Country
            else:
                cur.execute("""
                SELECT c.org_name, ob.name, ob.country, ob.position
                FROM office_bearer ob
                JOIN company c ON c.id = ob.company_id
                WHERE ob.country LIKE ?
                ORDER BY c.org_name
                LIMIT 300
                """, (f"%{query}%",))

            rows = cur.fetchall()
            conn.close()

        except Exception as e:
            QMessageBox.critical(self, "Search Error", f"Search failed:\n{e}")
            return

        # Clear & fill table
        self.table.setRowCount(0)

        if not rows:
            self.status.setText("No results found")
            return

        self.status.setText(f"{len(rows)} result(s) found")
        self.table.setRowCount(len(rows))

        for r, row in enumerate(rows):
            for c in range(4):
                self.table.setItem(r, c, QTableWidgetItem(str(row[c] or "")))


if __name__ == "__main__":
    app = QApplication(sys.argv)
    window = App()
    window.show()
    sys.exit(app.exec())
