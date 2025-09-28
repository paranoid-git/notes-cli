## Note Manager

Note Manager is a simple command-line tool for managing notes in Rust.

## Installation

To install Note Manager, clone the repository and build the project:

```bash
git clone https://github.com/yourusername/note-manager.git
cd note-manager
cargo build --release
```

## Usage

To use Note Manager, run the `note-manager` binary and follow the prompts:

```bash
./target/release/note-manager
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
note-manager create important "Hello, World!"
```

To list all notes, run:

```bash
note-manager list
```

To delete a note with the ID 2, run:

```bash
note-manager delete 2
```

To import notes from a JSON file, run:

```bash
note-manager import notes.json
```

To export notes to a JSON file, run:

```bash
note-manager export notes.json
```

To search for notes with the tag "important", run:

```bash
note-manager search important
```

