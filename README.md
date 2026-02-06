This repository contains:
- A script for parsing PDF files into JSON
- A GUI for searching and viewing the parsed information

## Features

- Seach company by name
- View directors of a company
- Search director by name or country
- Sort columns by clicking on column header
- Filter company by category, status, or registration date
- Filter with negation for search director by country (e.g., `not like <country>`)

## Setup

At root level, create a .env:


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
