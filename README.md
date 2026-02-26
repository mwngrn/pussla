![Logo](/docs/pussla-logo.png)


# 🧩 Pussla

> **"Bitarna på plats."** (Getting the pieces in place.)
>
> *Flat-pack resource planning for modern teams who prefer code over spreadsheets.*

Welcome to **Pussla** – our internal "Planning-as-Code" system. We built this because humans aren't rows in an Excel sheet, and resource planning shouldn't be a nightmare. We manage our 100+ colleagues using the same tools we use to build software: **Git, Markdown, and YAML.**

---

## 🇸🇪 The "Pussla" Philosophy

1. **Planning-as-Code:** If it's not in Git, it didn't happen.
2. **Context Matters:** Numbers tell us *what*, but Markdown tells us *why*.
3. **Privacy by Design:** We protect our people's data while staying AI-ready.
4. **Lagom is Best:** Not too little data, not too much—just enough to keep the projects moving.

---

## 🏗 System Architecture

Pussla is built on a decoupled architecture to ensure **Privacy by Design**.

### 1. The Identity Layer (`tst-data/identity/`)
*Contains PII (Personally Identifiable Information).*
This folder is restricted. It maps an anonymous `alias` to a real human being.
* **Format:** Markdown with YAML Frontmatter.
* **Example:** `tst-data/identity/FishCatcher.md` contains "Erik Andersson".

### 2. The Allocation Layer (`tst-data/planning/allocations/`)
*The "AI-Safe" zone.*
This is where the actual planning happens. It uses only `alias` values, meaning you can run AI analysis on this folder without exposing names to third-party models.
* **Format:** YAML.
* **Granularity:** Weekly slots, aggregated into months and quarters.

### 3. The Project Layer (`tst-data/planning/projects/`)
*Project context without PII.*
This folder stores project metadata and narrative context using YAML frontmatter + Markdown body.
* **Format:** Markdown with YAML Frontmatter.
* **Use:** Scope, risks, owner alias, team aliases, and time bounds.

---

## 🛠 Getting Started

### 1. Find your piece
To see your own allocation, look for your alias in `tst-data/planning/allocations/`.
> *Obs! Glöm inte att kolla så du inte är överbokad.* (Note: Don't forget to check that you aren't overbooked.)

### 2. Making changes
1. Create a new branch: `git checkout -b feature/update-plan-q3`.
2. Edit the relevant `.yaml` or `.md` file.
3. If the change is tied to a Backlog task, reference it in the commit message (for example: `git commit -m "Implemented task-3: update allocation logic"`).
4. Open a Pull Request for the Resource Manager to review.

### 3. Validation (The "Linter")
Every push triggers a CI/CD pipeline that checks for:
* **Overbooking:** If total `load > 100%`, you'll get a warning.
* **PII Leaks:** Ensures no names have accidentally been written in the public allocation files.

---

## 📊 Visualizing the Puzzle

While the data lives in text files, we view it through the **Pussla Dashboard**:
* **Heatmap View:** Weekly utilization for all 100+ consultants.
* **Quarterly Forecast:** Aggregated data for sales and management.
* **Privacy Mode:** A toggle that shows aliases (`FishCatcher`) or names (`Erik A.`) depending on your permissions.

### Run Dashboard Locally (No Deploy)
You can run the dashboard as a local web GUI, similar to `backlog.md`, without deploying to any web server.

1. Start the local server:
   `python3 src/dashboard/run_dashboard.py`
2. Open:
   `http://127.0.0.1:8080`

Optional flags:
* `--port 8090`
* `--port 0` (auto-select a free local port)
* `--data-dir /path/to/data-root` (expects `planning/` and `identity/` under that folder; legacy `planing/` is also supported)
* `--planning-dir tst-data/planning`
* `--identity-dir tst-data/identity`

---

## 🧪 Sample Data Layout

For testing and AI-safe sharing:
* Shareable folder: `tst-data/planning/`
* Public planning data: `tst-data/planning/allocations/` and `tst-data/planning/projects/`
* Private identity mapping (not shared): `tst-data/identity/`

---

## 🤝 Support
Need help fitting the pieces together? See `docs/` and the design decisions there, or contact the Pussla maintainer.

---

## ♥️ Contributing 
- Why is something done in a specific way: see `docs/design decisions/`.
- Requirements are in Sphinx-needs format in `reqs/`.
- Way of Working (requirements, backlog, docs responsibilities) is defined in `docs/way-of-working.md`.

### Compiling the requirements 
Run the `setup-dev-env.sh` to install sphinx-needs and need libraries. To build HTML, go to reqs folder and run `m̀ake html`, to build pdf - go to the `source` subfolder and `sphinx-build -M simplepdf . _build`.  

**"Nu kör vi!"** (Let's go!)
