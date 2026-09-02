# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- `toodoo list` - empty list now prints "No Tasks found"
- `toodoo clear` - clears all tasks
- JSON persistence to `toodoo.json` in the current directory.

## [0.2.1] - 2026-08-25

- `flip <task(s)>` - flips tasks state (replaces done/undone feature)

## [0.2.0] - 2026-08-24

- `add <text(s)>` - add 1 or more tasks
- `remove <id(s)>` - remove 1 or more tasks
- `done <id(s)>` - mark 1 or more tasks as complete
- `undone <id(s)>` - unmark 1 or more tasks as incomplete

## [0.1.2] - 2026-08-23

- `change <id, text>` — change task; errors if id doesn't exist.
- `remove <id>` — ids are reordered after each deletion.

## [0.1.1] - 2026-07-24

- `undone <id>` — mark a task incomplete; errors if the id doesn't exist.
- change help section

## [0.1.0] - 2026-07-23

Initial release.

### Added

- `add <text>` — append a task to the list.
- `list` — print all tasks with their id and completion status.
- `done <id>` — mark a task complete; errors if the id doesn't exist.
- `remove <id>` — delete a task by id.
- JSON persistence to `todo.json` in the current directory.
- A missing `todo.json` is treated as an empty list, so the first run works on
  a clean machine.
- Read or parse failures abort without writing, so a transient error can't
  overwrite an existing list with an empty one.
- Ids are assigned as `max(existing id) + 1`, so an id is never reused after
  its task is removed.

<!-- [0.1.0]: https://github.com/Lighx0x/toodoo/releases/tag/v0.1.0 -->
<!-- [0.1.1]: https://github.com/Lighx0x/toodoo/releases/tag/v0.1.1 -->
<!-- [0.1.2]: https://github.com/Lighx0x/toodoo/releases/tag/v0.1.2 -->
<!-- [0.2.0]: https://github.com/Lighx0x/toodoo/releases/tag/v0.2.0 -->
<!-- [0.2.1]: https://github.com/Lighx0x/toodoo/releases/tag/v0.2.1 -->
