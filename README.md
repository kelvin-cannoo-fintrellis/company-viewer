At root level, create a .env:

```
LLM_BACKEND=ollama
# or: openai

OPENAI_API_KEY=
OPENAI_MODEL=gpt-4.1-mini
OLLAMA_MODEL=qwen2.5:3b
OLLAMA_URL=http://localhost:11434
```

To run script for parsing the PDFs:

```bash
# Uses all defaults
cargo run

# Override some or all arguments
cargo run -- \
  --input-dir pdfs \
  --output-json-dir json_out \
  --output-markdown-dir md_out
```

After parsing PDFs, to create SQLite database from the JSONs:

```bash
uv run python import_json.py ../output_json
```

Run GUI:

```bash
uv run app.py
```
