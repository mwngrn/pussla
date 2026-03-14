Foundation Requirements
=======================

This page contains cross-cutting requirements that define core architecture,
privacy boundaries, validation expectations, and planning-as-code data handling.

.. req:: Git-based planning
   :id: REQ_PUSSLA_001
   :status: open
   :tags: core, workflow

   The system shall use Git for all resource planning to ensure traceability, version control, and pull-request-based review of changes.

.. req:: Two-layer architecture
   :id: REQ_PUSSLA_002
   :status: open
   :tags: security, architecture

   The system must separate PII into an Identity Layer and anonymized alias-based data into a Planning Layer to follow Privacy by Design.

.. req:: AI-safe allocation
   :id: REQ_PUSSLA_003
   :status: open
   :tags: ai, privacy

   The Planning Layer may only use anonymous aliases. This enables third-party AI analysis without exposing personal names.

.. req:: Workload validation
   :id: REQ_PUSSLA_004
   :status: open
   :tags: validation

   The system shall use a linter to warn when an individual's total planned hours exceed weekly capacity hours for any time period.
   The system shall also expose derived over-allocation percentage (greater than 100%) in dashboards and reports.
   Over-allocation warnings shall not block saving planning data.

.. req:: PII leak protection
   :id: REQ_PUSSLA_005
   :status: open
   :tags: security, ci-cd

   The CI/CD pipeline must automatically verify that no names or personal data are written in public planning files.

.. req:: Context via Markdown
   :id: REQ_PUSSLA_006
   :status: open
   :tags: ux

   Each resource shall have an associated Markdown file where qualitative information about goals and development can be documented, not only numeric data.

.. req:: Project metadata in frontmatter
   :id: REQ_PUSSLA_008
   :status: open
   :tags: architecture, data-model

   Each project shall have its own Markdown file with YAML frontmatter (for example: owner_alias, start_week, end_week, status) and a free-form Markdown body for scope and risks.

.. req:: Shareable AI folder without identity data
   :id: REQ_PUSSLA_009
   :status: open
   :tags: ai, privacy, structure

   Test and demo data shall be shareable via a single folder, ``tst-data/planning/``, containing at minimum ``people/``, ``roles/``, and ``projects/``, while ``tst-data/identity/`` remains separate.
   The old ``tst-data/planning/allocations/`` path shall be treated as deprecated and not used as canonical storage.
   Legacy path ``tst-data/planing/`` may be supported for backward compatibility.

.. req:: Non-developer installation bootstrap
   :id: REQ_PUSSLA_041
   :status: open
   :tags: deployment, installation, bootstrap

   The system shall support a bootstrap/setup flow suitable for non-developer users.
   The bootstrap shall create required planning artifacts when missing, including ``tst-data/planning/skills.md`` and a default role catalog in ``tst-data/planning/roles/``.
   Bootstrap defaults shall include at least one valid role definition (for example ``Consultant``) and a valid skills catalog skeleton with canonical skills and synonyms sections.

.. req:: Idempotent setup and safe defaults
   :id: REQ_PUSSLA_042
   :status: open
   :tags: deployment, installation, reliability

   Running setup/bootstrap multiple times shall be idempotent.
   Existing user-maintained planning files shall not be overwritten without explicit user confirmation or a dedicated force option.
   Missing required artifacts shall be created with valid default content.

.. req:: Installation readiness validation
   :id: REQ_PUSSLA_043
   :status: open
   :tags: deployment, validation, ux

   The system shall provide a post-install validation step that verifies required planning structure and files for runtime readiness.
   Validation output shall clearly identify missing directories/files and provide actionable remediation guidance for non-developer operators.

.. req:: Hybrid startup mode for local dashboard
   :id: REQ_PUSSLA_044
   :status: open
   :tags: deployment, ux, cli

   The system shall support a hybrid startup mode for local dashboard execution.
   The dashboard shall support auto-detecting data roots from the current working directory for simple non-developer usage.
   The dashboard shall also support explicit startup by passing a data root via command arguments (for example ``--data-dir``) for advanced and scripted usage.
   The selected data root shall be used to resolve the planning and identity folders required by the dashboard.

.. req:: Startup path precedence and deterministic behavior
   :id: REQ_PUSSLA_045
   :status: open
   :tags: deployment, cli, reliability

   When an explicit data root argument is provided, it shall take precedence over auto-detection.
   Startup path resolution behavior shall be deterministic and documented so users can predict which dataset is loaded.
   Startup output shall show the resolved planning and identity paths.

.. req:: Actionable startup errors for missing data roots
   :id: REQ_PUSSLA_046
   :status: open
   :tags: deployment, ux, validation

   If startup cannot resolve a valid data root with the required planning and identity structure, the system shall fail with actionable error messages.
   Error messages shall include expected directory/file structure and example commands for both auto-detected and explicit data-root startup modes.

.. req:: React dashboard startup launches default browser
   :id: REQ_PUSSLA_051
   :status: open
   :tags: deployment, ux, dashboard

   When ``run-react-dashboard.sh`` is started successfully, it shall launch the dashboard URL in the user's default web browser.

.. req:: Publish planning changes from dashboard
   :id: REQ_PUSSLA_052
   :status: open
   :tags: dashboard, ux, collaboration, workflow

   The dashboard shall allow a user to publish saved planning changes to the shared planning repository from the GUI without requiring terminal usage.
   The publish flow shall collect a user-facing change summary, create a versioned save of the current planning data, and send it to the configured shared repository.
   The dashboard shall report publish success or failure with actionable guidance when the operation cannot be completed.

.. req:: Refresh planning data from shared source
   :id: REQ_PUSSLA_053
   :status: open
   :tags: dashboard, ux, collaboration, workflow

   The dashboard shall allow a user to refresh local planning data from the shared planning repository from the GUI without requiring terminal usage.
   The refresh flow shall retrieve and apply remote changes before continued editing, or explain why it cannot do so safely.
   The dashboard shall clearly present when local unpublished changes would be affected by the refresh.

.. req:: Tauri desktop shell with external data folder
   :id: REQ_PUSSLA_054
   :status: open
   :tags: desktop, deployment, architecture

   The system shall provide a Tauri-based desktop application shell for non-technical users.
   The desktop shell shall start the local service layer automatically and present the application in an app window.
   The desktop shell shall use an external user-selected data folder rather than storing planning data inside the installed application.

.. req:: First-run choice to open or create data folder
   :id: REQ_PUSSLA_055
   :status: open
   :tags: desktop, ux, bootstrap

   On first run, the desktop application shall guide the user to either open an existing Pussla data folder or create a new one.
   The first-run flow shall not require terminal usage or manual file creation by the user.

.. req:: Bootstrap creates complete default data structure
   :id: REQ_PUSSLA_056
   :status: open
   :tags: desktop, bootstrap, data-model

   Creating a new Pussla data folder from the desktop application shall create the required ``planning/`` and ``identity/`` structure.
   Bootstrap shall create at minimum ``planning/people/``, ``planning/projects/``, ``planning/roles/``, ``planning/skills.md``, and ``identity/``.
   Bootstrap shall include valid starter content, including at least one default role and a valid skills catalog skeleton.

.. req:: Persist active data folder in local app settings
   :id: REQ_PUSSLA_057
   :status: open
   :tags: desktop, ux, state

   The desktop application shall remember the last selected active data folder in local application settings.
   On later startup, the application shall reuse that folder until the user chooses another one.

.. req:: Invalid data folder blocks planning and offers repair guidance
   :id: REQ_PUSSLA_058
   :status: open
   :tags: desktop, validation, ux

   If the selected data folder is missing required structure or files, the desktop application shall block entry into normal planning views.
   The application shall explain what is missing and offer actionable repair or bootstrap guidance.

.. req:: Shared sync setup is separate from first-run bootstrap
   :id: REQ_PUSSLA_059
   :status: open
   :tags: desktop, workflow, collaboration

   The desktop application's first-run bootstrap shall not require the user to configure shared collaboration or Git remote settings.
   Shared sync setup shall be a later explicit workflow after the data folder is created or selected.

.. req:: Linux-first desktop delivery
   :id: REQ_PUSSLA_060
   :status: open
   :tags: desktop, deployment, release

   The first supported desktop packaging target for non-technical users shall be Linux.
   Additional desktop platforms may be added later, but Linux shall be the initial delivery and verification target for installer and runtime behavior.
