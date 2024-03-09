# git-message-ai: Generate commit messages automatically

`git-message-ai` cli to generate your commit messages based on code changes and a ticket / task.

[TODO] Adicionar gif

## Installation

[TODO] For each language

### Start the configuration

1. It will create a local configuration file

```bash
git-message-ai initiate
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
git-message-ai generate [ticket-id]
```

or if you prefer using interactive mode:

```bash
git-message-ai generate -i
```

## Available Options

| Command            | Description                                     | Arguments                                         |
| ------------------ | ----------------------------------------------- | ------------------------------------------------- |
| `generate` or `-g` | Generates a message with predefined arguments   | (`ticket-id` or `-t`), (`interactive` or `-i`) |
| `initiate` or `-i` | Initializes the configuration                   | none                                              |

```bash
git-message-ai --help  

A CLI to generate your git messages!

Usage: git-message-ai [COMMAND]

Commands:
  generate, -g, --generate  Generates a message with predefined arguments
  initiate, -i, --initiate  Initializes the configuration
  help                      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```
