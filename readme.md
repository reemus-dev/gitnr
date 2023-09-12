# gitnr

A cross-platform CLI utility to create `.gitignore` files using templates.

- Use templates from the [GitHub](https://github.com/github/gitignore) & [TopTal](https://github.com/toptal/gitignore) collections
- Use local files and remote URLs as templates
- Filter out duplicate ignore lines when using multiple templates
- Interactive mode to search and select templates
- Cross-platform support (Windows, macOS, Linux)

```sh
# Create an ignore file for Rust with CLion IDE
gitnr create gh:Rust tt:clion+all

# Interactively search and select templates
gitnr search 
```

![gitnr demo](/doc/demo.gif)

## Table of Contents

- [Installation](#install--update)
    - [Linux & Mac](#linux--mac)
    - [Windows](#windows)
    - [Binary Download](#binary-download)
    - [From Source](#from-source)
- [Usage](#usage)
    - [Create](#create)
    - [Search](#search)
- [Why This Exists](#why-this-exists)
- [Contributing](#contributing)

## Install & Update

### Linux & Mac

Run any of the commands below in your terminal to get the latest version of `gitnr`.

**Install system-wide**
```sh
curl -s https://raw.githubusercontent.com/reemus-dev/gitnr/main/scripts/install.sh | sudo bash -s
```

**Install for current user**
```sh
curl -s https://raw.githubusercontent.com/reemus-dev/gitnr/main/scripts/install.sh | bash -s -- -u
```

_On Linux this defaults to `$HOME/.local/bin` and on macOS to `$HOME/bin`. The script will fail if the directory doesn't exist or is not in your system path._

**Install in specific directory**
```sh
curl -s https://raw.githubusercontent.com/reemus-dev/gitnr/main/scripts/install.sh | bash -s -- -d <dir>
```

### Windows

Run the command below in a PowerShell terminal to install the latest version of `gitnr`.

```powershell
Set-ExecutionPolicy Unrestricted -Scope Process; iex (iwr "https://raw.githubusercontent.com/reemus-dev/gitnr/main/scripts/install.ps1").Content
```

### Binary Download

See the [releases page](https://github.com/reemus-dev/gitnr/releases) to download a binary and then add it to a directory in your system path.

### From Source

```sh
git clone --depth=1 github.com/reemus-dev/gitnr
cd gitnr
cargo install --path .
```

_Note: This requires that you have Rust and cargo installed on your system._

## Usage

There are 3 commands available

| Command  | Description                                                                                                                                                                                               |
|----------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `create` | Create a .gitignore file and print the content to `stdout` or save it to a file                                                                                                                           |
| `search` | Interactive mode to search and select templates from the GitHub and TopTal collections. You can then copy the result to your clipboard or copy the relevant `create` command to generate your ignore file |
| `help`   | Display the CLI help message with available flags & commands                                                                                                                                              |

## Create

The create command accepts a list of templates to generate a `.gitignore` file with.

```sh
gitnr create [FLAGS] [TEMPLATES]...
```

### Create Template Arguments

Templates can be provided to the CLI as:
- Space separated or comma separated values
- With or without their source prefix

The following template sources are available:

| Template Sources                                                                      | Prefix  |
|---------------------------------------------------------------------------------------|---------|
| URL                                                                                   | `url:`  |
| File                                                                                  | `file:` |
| GitHub (a file from any public repo)                                                  | `repo:` |
| [GitHub Templates](https://github.com/github/gitignore/tree/main)                     | `gh:`   |
| [GitHub Community Templates](https://github.com/github/gitignore/tree/main/community) | `ghc:`  |
| [GitHub Global Templates](https://github.com/github/gitignore/tree/main/Global)       | `ghg:`  |
| [TopTal Templates](https://github.com/toptal/gitignore/tree/master/templates)         | `tt:`   |

For example:

```sh
# With prefix
gitnr create gh:Node

# Without prefix
gitnr create Node

# Combining templates - a project using Node.js + Vue in WebStorm
gitnr create gh:Node ghc:JavaScript/Vue tt:webstorm+all

# Using a remote URL and local file
gitnr create url:https://domain.com/template.gitignore file:path/to/local.template.gitignore

# Using a file from a GitHub repo
gitnr create repo:github/gitignore/main/Rust.gitignore
```

If you do not prefix the template, the CLI will try to automatically detect the template source. If it can't match the template name to a source, it defaults to checking the GitHub template collection. It's advised to be explicit about the source prefix to avoid any ambiguity.

Templates from the GitHub and TopTal collections do not need to have the `.gitignore`, `.stack` or `.patch` suffixes. Meaning you can use `gh:Rust` instead of `gh:Rust.gitignore` or `tt:webstorm+all` instead of `tt:webstorm+all.patch`.

The generated template will be created in the order of the template arguments supplied.

> [!NOTE]
> The TopTal template collection includes `stacks` and `patches`. A stack specifies multiple ignore templates that are combined, e.g. `Angular.stack`. The patch extension add modifications to the original template from GitHub's collection.

### Create Flags

By default, the resulting .gitignore template is printed to `stdout`. You can customize this behaviour using the CLI flags available:

| Flag            | Short       | Description                                                                           |
|-----------------|-------------|---------------------------------------------------------------------------------------|
| `--save`        | `-s`        | Write template to .gitignore file in current directory (overwriting any exiting file) |
| `--file <path>` | `-f <path>` | Write template to the specified file path overwriting any exiting file                |
| --refresh       | -r          | Refresh the template cache (templates are cached for 1 hour by default)               |


## Search

The search command allows you to interactively browse, filter and select templates from the GitHub and Toptal collections.

```bash
gitnr search
```

This is useful when you want to see what's available and preview different template combinations. You will be able to preview an individual template as well as preview a template combination.

The search command only has one flag, which is `--refresh | -r`. This allows you to refresh the template cache which by default caches the template collections for 1 hour. This is to avoid hitting the API rate-limits.

## Why This Exists?

- I wanted a way to template my `.gitignore` files for all projects to keep things consistent.
- I wanted to build a `.gitignore` from multiple templates to keep things modular
- I wanted to build something with Rust and learn the language
- And apparently I wanted to waste more time than I'd like to on a simple side project 😅

## Contributing

Open a PR or create an issue with any suggestions. Given this is my first Rust application, veterans will probably spot a lot of things that can be improved, refactored or removed. So feel free to open a PR or issue with any suggestions.

---

Improve your software dev skills by learning from my programming struggles at https://reemus.dev
