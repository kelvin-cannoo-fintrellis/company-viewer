This repository contains:
- A script for parsing PDF files into JSON
- A GUI for searching and viewing the parsed information

## Project Structure

```text
.
├── pdf/            # Input PDFs (must be added manually)
├── output_json/    # Generated JSON files
├── frontend/       # GUI and database scripts
````

## Prerequisites

* Rust (cargo)
* Python with `uv`

## Installation

After cloning the repository:

```bash
cd company-viewer
cp .env.example .env
cargo build
```

> Update `.env` with any required environment variables before running the parser.

## Usage

### Parse PDFs to JSON

Place all PDF files in a `pdf/` directory at the project root, then run:

```bash
cargo run
```

This will generate an `output_json/` directory containing the parsed JSON files. If you have set `DEBUGGING=true` in your `.env`, `output_markdown/` directory will also be created.

### Build the SQLite database

```bash
cd frontend
uv run python import_json.py ../output_json
```

### Run the GUI

```bash
uv run app.py
```

The GUI will launch using the generated SQLite database.
