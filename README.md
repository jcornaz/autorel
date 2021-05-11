# autorel

[![License](https://img.shields.io/github/license/jcornaz/autorel)](https://github.com/jcornaz/autorel/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/jcornaz/autorel/status.svg)](https://deps.rs/repo/github/jcornaz/autorel)
[![Build](https://github.com/jcornaz/autorel/actions/workflows/build.yml/badge.svg)](https://github.com/jcornaz/autorel/actions/workflows/build.yml)
[![Zenhub](https://img.shields.io/badge/workspace-zenhub-%236061be)](https://app.zenhub.com/workspaces/autorel-60980eaac1cd55000f3de46b/board)

A release automation CLI

## Usage

`autorel` parses tag and commit messages of the commits since the last release to decide if there is something to
release.

If there is indeed something to release, it infers the next version number (according to the semantic versioning rules)
and invoke the hooks defined in the configuration file (`release.yml` by default)

For the reference of the configuration file see:
https://github.com/jcornaz/autorel/blob/main/docs/config-ref.md

By default, it'll also generate a changelog using [clog](https://github.com/clog-cli). To customize the changelog
generation see:
https://github.com/clog-tool/clog-lib/tree/0.9.0#default-options

This utility must run from the root of a git repository that follows the conventional-commits convention.
See: https://www.conventionalcommits.org

```
USAGE:
    autorel [FLAGS] [OPTIONS]

FLAGS:
        --dry-run
            Only prints what would be done if the this flag wasn't specified. Without actually doing
            anything

    -h, --help
            Prints help information

    -V, --version
            Prints version information


OPTIONS:
        --config <config>
            Path of the configuration file [default: release.yml]
```