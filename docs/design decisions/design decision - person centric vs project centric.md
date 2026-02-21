Design Choice: Person-centric vs Project-centric Storage

Status

Accepted

Context

We need to decide whether to store allocation data based on the project (one file per project, listing all people) or based on the person (one file per person/alias, listing all their projects).

Decision

We will use Person-centric storage for allocations. Each unique alias will have its own file in the /allocations/ directory (e.g., /allocations/fishcatcher.yaml).
Project metadata and narrative context are stored separately in /projects as Markdown files with YAML frontmatter.

Rationale

1. Minimizing Git Merge Conflicts

In an organization of 100+ people, multiple consultants often update their schedules simultaneously (e.g., every Friday). If assignments were stored in project files, a single popular project could become a bottleneck for commits, leading to constant merge conflicts. Person-centric files ensure that an individual's updates never collide with another's.

2. Validation Efficiency (Total Load)

The primary constraint in Pussla is that a person's total load across all projects must not exceed 100%. With person-centric files, validating this constraint only requires reading a single file. Project-centric storage would require scanning the entire repository to calculate a single person's total workload.

3. Clear Data Ownership

A person-centric file acts as a "personal schedule." It is clear who is responsible for the accuracy of the data—the individual or their direct manager.

Implementation

While the storage is person-centric, the view is flexible. The Pussla Engine (parser) is responsible for pivoting this data to provide:

Project Views: "Who is working on Project X?" (Aggregated from all person files).

Team Views: "What is the capacity of the Backend Team?"

Company Views: "What is our total utilization?"

Consequences

File Count: We will have approximately 100+ files in the allocations folder. This is well within the performance limits of Git and our Python parser.

Discovery: To find who is on a project without using the GUI, developers can use standard CLI tools like grep -r "Project Name" ./allocations.
