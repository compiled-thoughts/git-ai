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

# If you are using JIRA
GA_JIRA_API_KEY=<you can get it here: https://id.atlassian.com/manage-profile/security/api-tokens>
GA_JIRA_USER=<your user to sign in into jira>

# If you are using OPENAI
GA_OPENAI_TOKEN=<you can get it here: https://platform.openai.com/api-keys>
```

> **Example unix:**  
> ```bash
> export GA_JIRA_USER="copiled@gmail.com"  
> export GA_JIRA_TOKEN="ITH0AOKENTIJAR"  
> export GA_OPENAI_TOKEN="sk-ODIAjsoid"
> ```

> **Example windows:**
> ```powershell
> $Env:GA_JIRA_USER="copiled@gmail.com"  
> $Env:GA_JIRA_TOKEN="ITH0AOKENTIJAR"  
> $Env:GA_OPENAI_TOKEN="sk-ODIAjsoid"
> ```

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
| `generate` or `-g` | Generates a message with predefined arguments   | (`ticket-id` or `-t`), (`interactive` or `-i`)    |
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
