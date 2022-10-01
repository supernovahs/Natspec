# Natspec 

Generate perfect code Natspec every time.

## Build

You need Rust and Cargo installed on your machine. See the installation guide
[here](https://doc.rust-lang.org/cargo/getting-started/installation.html).

Then clone the repo and install the CLI globally like this:

```sh
cargo install --path .
```

## Usage

```sh
author "<Title of Contract> " "<Author Name>" "<Author contact>" "<Description of contract>"
```

```sh
/// @title: Natspec Generator
/// @author: supernovahs.eth <supernovahs@proton.me>
/// @notice: Generates perfect every time
```

It will also copy the Natspec to your clipboard automatically.

### With VSCode

Set your global [`tasks.json`](https://stackoverflow.com/questions/41046494/making-global-tasks-in-vs-code) like so to add the command as task:

```json
{
    "version": "2.0.0",
    "tasks": [
      {
        "label": "Generate title,author",
        "type": "shell",
        "command": "author ${input:title} ${input: authorname} ${input:authorcontact} ${input:description}",
        "presentation": {
          "reveal": "never"
        }
      }
    ],
    "inputs": [
      {
        "id": "header",
        "description": "Header",
        "type": "promptString"
      },{
        "id": "title",
        "description": "title",
        "type": "promptString" 
      },{
        "id": "authorname",
        "description": "authorname",
        "type": "promptString"
      },{
        "id": "authorcontact",
        "description": "authorcontact",
        "type": "promptString"
      },{
        "id": "description",
        "description": "description",
        "type": "promptString"
      }
    ]
  }
```

This will copy the generated Natspec to your clipboard.

## Credits

[@transmissions11](https://github.com/transmissions11)
