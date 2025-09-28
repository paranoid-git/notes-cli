## Note Manager

notes-cli is a simple command-line tool for managing notes in Rust.

## Installation

To install notes-cli, clone the repository and build the project:

```bash
git clone https://github.com/paranoid-git/notes-cli.git
cd notes-cli
cargo build --release
```

## Usage

To use notes-cli, run the binary and follow the prompts:

```bash
./target/release/notes-cli
```

Here are the available commands:

- `create`: Create a new note
- `list`: List all notes
- `delete`: Delete a note
- `import`: Import notes from a JSON file
- `export`: Export notes to a JSON file
- `help`: Helpful message
- `search`: Search for notes via tag

For example, to create a new note with the content "Hello, World!", run:

```bash
create important "Hello, World!"
```

To list all notes, run:

```bash
list
```

To delete a note with the ID 2, run:

```bash
delete 2
```

To import notes from a JSON file, run:

```bash
import notes.json
```

To export notes to a JSON file, run:

```bash
export notes.json
```

To search for notes with the tag "important", run:

```bash
search important
```

