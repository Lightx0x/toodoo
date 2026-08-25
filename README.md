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
toodoo add "Buy milk"                # add a task
toodoo add "Buy eggs" "Walk the dog" # add multiple tasks
toodoo list                          # show all tasks
toodoo flip 1                        # mark task 1 as done
toodoo flip 1 2 3                    # mark multiple tasks as done
toodoo flip 1                      # mark task 1 as incomplete
toodoo flip 1 2 3                  # mark multiple tasks as incomplete
toodoo remove 1                      # delete task 1
toodoo remove 1 2 3                  # delete multiple tasks
toodoo change 1 "Buy eggs"           # change task 1
```

Example session:

```
$ toodoo add "Finish Rust project" "Test the remove feature" "Try deleting from list"
$ toodoo list
[ ] 1: Finish Rust project
[ ] 2: Test the remove feature
[ ] 3: Try deleting from list

$ toodoo flip 1 2
$ toodoo list
[✓] 1: Finish Rust project
[✓] 2: Test the remove feature
[ ] 3: Try deleting from list

$ toodoo remove 2
$ toodoo list
[✓] 1: Finish Rust project
[ ] 2: Try deleting from list

$ toodoo flip 1
$ toodoo list
[ ] 1: Finish Rust project
[ ] 2: Try deleting from list

$ toodoo change 1 "Finish Other Project"
$ toodoo list
[ ] 1: Finish Other Project
[ ] 2: Try deleting from list
```

`toodoo --help` lists every command; `toodoo <command> --help` explains one.

## Task State

Tasks are stored as JSON in `todo.json` in the current directory:

```json
[
  {
    "id": 1,
    "text": "Finish Other project",
    "done": false
  }
]
```

If the file doesn't exist, `toodoo` treats that as an empty list rather than an
error — so the first run works on a clean machine. If the file exists but can't
be read or parsed, `toodoo` reports the problem and exits without writing,
rather than silently starting over and overwriting your data.

## Behaviour notes

- `flip` on an id that doesn't exist is an error.
- `remove` on an id that doesn't exist succeeds silently — removing something
  that isn't there already achieves the goal.
- `list` never writes to disk.
- `change` on an id that doesn't exist is an error.

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
