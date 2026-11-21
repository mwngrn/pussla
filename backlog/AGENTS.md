# Working in this backlog

This folder is an m2d board: one Markdown note per story, in the m2d
format. Edit the notes directly; no tool is needed. The board note
`backlog.md` holds the columns and the `id_prefix`. It is a board, not a
product: the product is described elsewhere (README.md, the owner's
note).

This file is shared. Copy it unchanged from the m2d format repository
(`backlog/AGENTS.md`, with `CLAUDE.md` beside it). Rules for one
repository go in that repository's own AGENTS.md.

## A story

- A note in this folder: frontmatter `id`, `aliases` (the same id) and
  `priority`, then `# Title` and a status line `#story/<state> Summary`.
- States: a bare `#story` is the funnel (an idea, not yet accepted), then
  `backlog`, `do`, `review`, `done`, as the columns in `backlog.md`.
- New id: the board's `id_prefix`, `-` and four random characters from
  `0123456789ABCDEFGHJKMNPQRSTVWXYZ`, not used in this folder. Put it in
  `aliases` too. An id never changes and is never reused.
- `priority` is the order on the board, lower first. A new story gets the
  highest value plus one unless you are told otherwise; a fraction puts
  it between two others.
- Who works on it: `@name` before the first `##` heading.
- Sections: `## Description`, `## Acceptance Criteria`,
  `## Implementation Notes`, `## Final Summary`.
- Criteria are plain bullets, and a met one gets a `✓` (`- ✓ Login
  works`). Write `- [ ]` only for real actions, since they show up as
  tasks in m2d's views. Never put criteria in a code block.
- Dependencies: `Requires < [[Other story]]` under the status line.
- An epic is a note with `#epic/<state>` and a folder of the same name
  beside it, which holds its stories.

## Working on a story

- Starting: set `#story/do` and write in Implementation Notes which
  session or agent you are. Do not edit a story another session has in
  `#story/do`, and create stories one at a time.
- Done in code but not tried by a person: `#story/review`, and write what
  to try and where.
- Finished: `#story/done` and a short Final Summary. A story that will
  not be done is deleted, not marked.
- Update the story in the same commit as the work, or right after.
  Commit messages reference its id: `(PREFIX-7KQ3)`.
- Anything still open, also what only a person can do (try it, decide),
  is written in the story, not only in a chat or another note.

## Presenting stories to a person

A random id is for tools and commits, not for people to remember. When you
mention a story to a person (in a chat, a report or a commit summary):

- Lead with the title and put the id in parentheses after it: `Calendar
  for dates (M2D-PN58)`. Never give a bare id. In notes, link by note
  name, `[[Calendar for dates]]`.
- When you list several, group them by column (funnel, backlog, do,
  review, done) and number the lines in your answer, so the person can
  reply "take 2". Numbers belong to that answer only; they are never
  written into a note.
- Keep the list short: what is in `do` and `review` first, then the next
  few in `backlog`. Say how many more there are instead of listing all.

The full rules are part 05 of the m2d format specification, and the m2d
agent skill.
