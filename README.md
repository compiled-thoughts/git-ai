# 🧠 git-ai

The **git-ai** generates commit messages and pull requests automatically based on your tasks and file changes.

This project is a CLI developed in Rust which can be called using `git-ai` cli to generate your commit messages based on code changes and a ticket / task.

[TODO] Adicionar gif

## Installation

[TODO] For each language

### Start the configuration

1. It will create a local configuration file

```bash
git-ai initiate
```

2. Declare these envinroment varibles:

```.env
GMA_JIRA_AUTHORIZATION=<jira authorization can be your user and password as a basic>
GMA_OPENAI_TOKEN=<you can get it here: https://platform.openai.com/api-keys>
```

> Example: GMA_JIRA_AUTHORIZATION="Basic dXNlcjpwYXNzd29yZA=="   
> Example: GMA_OPENAI_TOKEN="sk-ODIAjsoid"

## Usage

```bash
git-ai generate [ticket-id]
```

or if you prefer using interactive mode:

```bash
git-ai generate -i
```

## Available Options

| Command            | Description                                     | Arguments                                         |
| ------------------ | ----------------------------------------------- | ------------------------------------------------- |
| `generate` or `-g` | Generates a message with predefined arguments   | (`ticket-id` or `-t`), (`interactive` or `-i`) |
| `initiate` or `-i` | Initializes the configuration                   | none                                              |

```bash
git-ai --help  

A CLI to generate your git messages!

Usage: git-ai [COMMAND]

Commands:
  generate, -g, --generate  Generates a message with predefined arguments
  initiate, -i, --initiate  Initializes the configuration
  help                      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```
