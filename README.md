![[./docs/pussla-logo.png]] 

🧩 Pussla

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

### 1. The Identity Layer (`/identity/`)
*Contains PII (Personally Identifiable Information).*
This folder is restricted. It maps an anonymous `alias` to a real human being.
* **Format:** Markdown with YAML Frontmatter.
* **Example:** `identity/FishCatcher.md` contains "Erik Andersson".

### 2. The Allocation Layer (`/allocations/`)
*The "AI-Safe" zone.*
This is where the actual planning happens. It uses only `alias` values, meaning you can run AI analysis on this folder without exposing names to third-party models.
* **Format:** YAML.
* **Granularity:** Weekly slots, aggregated into months and quarters.

### 3. The Project Layer (`/projects/`)
*Project context without PII.*
This folder stores project metadata and narrative context using YAML frontmatter + Markdown body.
* **Format:** Markdown with YAML Frontmatter.
* **Use:** Scope, risks, owner alias, team aliases, and time bounds.

---

## 🛠 Getting Started

### 1. Find your piece
To see your own allocation, look for your alias in the `/allocations/` folder.
> *Obs! Glöm inte att kolla så du inte är överbokad.* (Note: Don't forget to check that you aren't overbooked.)

### 2. Making changes
1. Create a new branch: `git checkout -b feature/update-plan-q3`.
2. Edit the relevant `.yaml` or `.md` file.
3. Commit and push: `git commit -m "Allokera FishCatcher till Project X"`.
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

---

## 🧪 Sample Data Layout

For testing and AI-safe sharing:
* Shareable folder: `tst-data/planing/`
* Public planning data: `tst-data/planing/allocations/` and `tst-data/planing/projects/`
* Private identity mapping (not shared): `tst-data/identity/`

---

## ⚠️ Warning Label (IKEA style)
* **Assembly required:** You must manually update your allocations once a week.
* **Do not force it:** If a piece (consultant) doesn't fit a project, don't just increase the `load` to 120%. Talk to them instead.
* **Keep away from spreadsheets:** Importing an Excel file into Pussla is strictly forbidden and may cause the developers to cry.

---

## 🤝 Support
Need help fitting the pieces together? 

**"Nu kör vi!"** (Let's go!)
"""

with open("README.md", "w", encoding="utf-8") as f:
    f.write(content)
