needs_content = """
.. req:: Git-baserad planering
   :id: REQ_PUSSLA_001
   :status: open
   :tags: core, workflow

   Systemet ska använda Git för all resursplanering för att säkerställa spårbarhet, versionshantering och möjligheten till Pull Requests vid ändringar.

.. req:: Arkitektur i två lager
   :id: REQ_PUSSLA_002
   :status: open
   :tags: security, architecture

   Systemet måste separera personuppgifter (PII) i ett "Identity Layer" och anonymiserad data i ett "Allocation Layer" för att följa Privacy-by-Design.

.. req:: AI-säker allokering
   :id: REQ_PUSSLA_003
   :status: open
   :tags: ai, privacy

   Allokeringslagret får endast använda anonyma alias. Detta tillåter analys via tredjeparts-AI utan att exponera personnamn.

.. req:: Validering av arbetsbelastning
   :id: REQ_PUSSLA_004
   :status: open
   :tags: validation

   Systemet ska via en linter varna om en individs totala allokering överstiger 100% per tidsperiod.

.. req:: Skydd mot PII-läckage
   :id: REQ_PUSSLA_005
   :status: open
   :tags: security, ci-cd

   CI/CD-pipelinen måste automatiskt verifiera att inga namn eller personuppgifter har skrivits in i de publika allokeringsfilerna i YAML-format.

.. req:: Kontext via Markdown
   :id: REQ_PUSSLA_006
   :status: open
   :tags: ux

   Varje resurs ska ha en tillhörande Markdown-fil där kvalitativ information om mål och personlig utveckling kan dokumenteras, inte bara siffror.

.. req:: Projektmetadata i frontmatter
   :id: REQ_PUSSLA_008
   :status: open
   :tags: architecture, data-model

   Varje projekt ska ha en egen Markdown-fil med YAML-frontmatter (t.ex. owner_alias, start_week, end_week, status) samt en fri Markdown-kropp för scope och risker.

.. req:: Delbar AI-mapp utan identitet
   :id: REQ_PUSSLA_009
   :status: open
   :tags: ai, privacy, structure

   Test- och demonstrationsdata ska kunna delas via en enskild mapp `tst-data/planing/` som innehåller `allocations/` och `projects/`, medan `tst-data/identity/` hålls separat.

.. req:: Dashboard-visualisering
   :id: REQ_PUSSLA_007
   :status: open
   :tags: frontend

   Systemet ska kunna generera en visuell heatmap och kvartalsprognoser baserat på de underliggande textfilerna.
"""
