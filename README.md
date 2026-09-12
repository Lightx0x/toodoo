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
toodoo flip 1                        # toggle task 1 between done and undone
toodoo flip 1 2 3                    # toggle multiple tasks
toodoo remove 1                      # delete task 1
toodoo remove 1 2 3                  # delete multiple tasks
toodoo change 1 "Buy eggs"           # change task 1
toodoo clear                         # delete all tasks
```

Every command that modifies the list prints a confirmation once the change has
been written to disk.

Example session:

```
$ toodoo add "Finish Rust project" "Test the remove feature" "Try deleting from list"
Added Finish Rust project
Added Test the remove feature
Added Try deleting from list

$ toodoo list
[ ] 1: Finish Rust project
[ ] 2: Test the remove feature
[ ] 3: Try deleting from list

$ toodoo flip 1 2
Flipped task 1: Finish Rust project -> done
Flipped task 2: Test the remove feature -> done

$ toodoo remove 2
Removed Test the remove feature

$ toodoo list
[✓] 1: Finish Rust project
[ ] 2: Try deleting from list

$ toodoo flip 1
Flipped task 1: Finish Rust project -> undone

$ toodoo change 1 "Finish Other Project"
Changed task 1: Finish Rust project -> Finish Other Project

$ toodoo list
[ ] 1: Finish Other Project
[ ] 2: Try deleting from list

$ toodoo clear
Cleared 2 task(s)

$ toodoo list
No Tasks found
```

`toodoo --help` lists every command; `toodoo <command> --help` explains one.

## Task State

Tasks are stored as JSON in `todo.json` in the current directory:

```json
[
  {
    "id": 1,
    "text": "Finish Other Project",
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
- `change` on an id that doesn't exist is an error.
- `clear` on an empty list succeeds and reports `Cleared 0 task(s)`.
- `list` never writes to disk.
- Confirmations are printed only after a successful save; if the write fails,
  the error is reported instead and nothing is confirmed.

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
