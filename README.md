# Stem Cell

[![Build status](https://github.com/stepchowfun/stem-cell/workflows/Continuous%20integration/badge.svg?branch=main)](https://github.com/stepchowfun/stem-cell/actions?query=branch%3Amain)

This is a simple project to demonstrate the cross-platform release management process I use for my open source work. The program itself just prints `Hello, World!`.

This repository can be used as a starting point for a new project. Be sure to rename all references to `Stem Cell` and `stem-cell` accordingly. Then follow the instructions in [`MAINTAINERS.md`](https://github.com/stepchowfun/stem-cell/blob/main/MAINTAINERS.md) to configure the repository on GitHub.

## Installation instructions

### Easy installation on macOS or Linux

If you are running macOS or Linux on an x86-64 CPU, you can install Stem Cell with this command:

```sh
curl https://raw.githubusercontent.com/stepchowfun/stem-cell/main/install.sh -LSfs | sh
```

The same command can be used again to update Stem Cell to the latest version.

**NOTE:** Piping `curl` to `sh` is considered dangerous by some since the server might be compromised. If you're concerned about this, you can download and inspect the installation script or choose one of the other installation methods.

#### Customizing the installation

The installation script supports the following environment variables:

- `VERSION=x.y.z` (defaults to the latest version)
- `PREFIX=/path/to/install` (defaults to `/usr/local/bin`)

For example, the following will install Stem Cell into the working directory:

```sh
curl https://raw.githubusercontent.com/stepchowfun/stem-cell/main/install.sh -LSfs | PREFIX=. sh
```

### Manual installation for macOS, Linux, or Windows

The [releases page](https://github.com/stepchowfun/stem-cell/releases) has precompiled binaries for macOS, Linux, and Windows systems running on an x86-64 CPU. You can download one of them and place it in a directory listed in your [`PATH`](https://en.wikipedia.org/wiki/PATH_\(variable\)).

### Installation with Cargo

If you have [Cargo](https://doc.rust-lang.org/cargo/), you can install Stem Cell as follows:

```sh
cargo install stem-cell
```

You can run that command with `--force` to update an existing installation.
