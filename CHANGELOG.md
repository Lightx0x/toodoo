# Changelog

All notable changes to this project are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[0.1.0]: https://github.com/Lighx0x/toodoo/releases/tag/v0.1.0
