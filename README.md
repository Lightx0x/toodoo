## toodoo

A small command-line todo list, written in Rust. Tasks live in a JSON file next to wherever you run it, so they survive between invocations.

## Install

```bash
cargo install toodoo
```

Or from source:

```bash
git clone https://github.com/Lightx0x/toodoo 
cd toodoo
cargo install --path .
```

## Usage

```bash
toodoo add "Buy milk"        # add a task
toodoo list                  # show all tasks
toodoo done 1                # mark task 1 as complete
toodoo undone 1              # mark task 1 as incomplete
toodoo remove 1              # delete task 1
```

Example session:

```
$ toodoo add "Finish Rust project"
$ toodoo add "Test the remove feature"
$ toodoo add "Try deleting from list"
$ toodoo list
[ ] 1: Finish Rust project
[ ] 2: Test the remove feature
[ ] 3: Try deleting from list

$ toodoo done 1
$ toodoo remove 2
$ toodoo list
[✓] 1: Finish Rust project
[ ] 3: Try deleting from list

$ toodoo undone 1
$ toodoo list
[ ] 1: Finish Rust project
[ ] 3: Try deleting from list
```

`toodoo --help` lists every command; `toodoo <command> --help` explains one.

## Storage

Tasks are stored as JSON in `todo.json` in the current directory:

```json
[
  {
    "id": 1,
    "text": "Finish Rust project",
    "done": false
  }
]
```

If the file doesn't exist, `toodoo` treats that as an empty list rather than an
error — so the first run works on a clean machine. If the file exists but can't
be read or parsed, `toodoo` reports the problem and exits without writing,
rather than silently starting over and overwriting your data.

## Task ids

Ids are assigned as `max(existing id) + 1`, not `count + 1`. That means a
deleted id is never handed out again:

```
ids: 1, 2, 3  →  remove 2  →  ids: 1, 3  →  add  →  id 4
```

The numbering ends up gappy, which is deliberate. An id is meant to name one
task for as long as that task exists; reusing a removed id would let `done 3`
refer to two different tasks at two different times.

## Behaviour notes

- `done` on an id that doesn't exist is an error.
- `undone` on an id that doesn't exist is an error.
- `remove` on an id that doesn't exist succeeds silently — removing something
  that isn't there already achieves the goal.
- `list` never writes to disk.

## Development

```bash
cargo test      # unit tests, including persistence round-trips
cargo clippy    # no warnings
cargo fmt
```

Tests that touch the filesystem use `tempfile`, so each one gets its own
directory and cleans up automatically — including when a test fails partway
through.

## License

MIT toodoo
