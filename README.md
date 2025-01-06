# Stem Cell

[![Build status](https://github.com/stepchowfun/stem-cell/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/stepchowfun/stem-cell/actions?query=branch%3Amain)

This is a simple project to demonstrate the cross-platform release management process I use for my open source work. The program itself just prints `Hello, World!`.

This repository can be used as a starting point for a new project. Be sure to rename all references to `Stem Cell` and `stem-cell` accordingly. Then follow the instructions in [`MAINTAINERS.md`](https://github.com/stepchowfun/stem-cell/blob/main/MAINTAINERS.md) to configure the repository on GitHub.

## Installation instructions

### Installation on macOS or Linux (AArch64 or x86-64)

If you're running macOS or Linux (AArch64 or x86-64), you can install Stem Cell with this command:

```sh
curl https://raw.githubusercontent.com/stepchowfun/stem-cell/main/install.sh -LSfs | sh
```

The same command can be used again to update to the latest version.

The installation script supports the following optional environment variables:

- `VERSION=x.y.z` (defaults to the latest version)
- `PREFIX=/path/to/install` (defaults to `/usr/local/bin`)

For example, the following will install Stem Cell into the working directory:

```sh
curl https://raw.githubusercontent.com/stepchowfun/stem-cell/main/install.sh -LSfs | PREFIX=. sh
```

If you prefer not to use this installation method, you can download the binary from the [releases page](https://github.com/stepchowfun/stem-cell/releases), make it executable (e.g., with `chmod`), and place it in some directory in your [`PATH`](https://en.wikipedia.org/wiki/PATH_\(variable\)) (e.g., `/usr/local/bin`).

### Installation on Windows (AArch64 or x86-64)

If you're running Windows (AArch64 or x86-64), download the latest binary from the [releases page](https://github.com/stepchowfun/stem-cell/releases) and rename it to `stem-cell` (or `stem-cell.exe` if you have file extensions visible). Create a directory called `Stem Cell` in your `%PROGRAMFILES%` directory (e.g., `C:\Program Files\Stem Cell`), and place the renamed binary in there. Then, in the "Advanced" tab of the "System Properties" section of Control Panel, click on "Environment Variables..." and add the full path to the new `Stem Cell` directory to the `PATH` variable under "System variables". Note that the `Program Files` directory might have a different name if Windows is configured for a language other than English.

To update an existing installation, simply replace the existing binary.

### Installation with Cargo

If you have [Cargo](https://doc.rust-lang.org/cargo/), you can install Stem Cell as follows:

```sh
cargo install stem-cell
```

You can run that command with `--force` to update an existing installation.
