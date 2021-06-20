# Stem Cell

[![Build status](https://github.com/stepchowfun/stem-cell/workflows/Continuous%20integration/badge.svg?branch=main)](https://github.com/stepchowfun/stem-cell/actions?query=branch%3Amain)

This is a simple project to demonstrate the cross-platform release management process for my open source work.

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

## For maintainers

### Release instructions

1. Bump the version in `Cargo.toml`, run `cargo build` to update `Cargo.lock`, and update `CHANGELOG.md` with information about the new version. Ship those changes as a single commit.
2. Once the GitHub workflow has finished on the `main` branch, update the version in `install.sh` to point to the new release.

### GitHub instructions

When setting up the repository on GitHub, change the following settings:

- Under `Secrets`, add the following repository secrets with appropriate values:
  - `CRATES_IO_TOKEN`
  - `DOCKER_PASSWORD`
- Under `Branches`, add a branch protection rule for the `main` branch.
  - Enable `Require status checks to pass before merging`.
    - Enable `Require branches to be up to date before merging`.
    - Add the following status checks:
      - `Build and test on Linux`
      - `Build and test on Windows`
      - `Build and test on macOS`
      - `Create a release on GitHub if applicable`
      - `Run the installer script to validate it`
  - Enable `Include administrators`.
- Under `Options`, enable `Automatically delete head branches`.

The GitHub workflow will fail initially because the job to install the latest release will not find any release to download. You will need to bootstrap the v0.0.0 release by temporarily removing or commenting out the line in the workflow with the `[ref:remove_to_bootstrap]` tag.

This repository can be used as a starting point for a new project. Be sure to rename all references to `Stem Cell` and `stem-cell` accordingly.
