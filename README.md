# autorel

[![License](https://img.shields.io/github/license/jcornaz/autorel)](https://github.com/jcornaz/autorel/blob/main/LICENSE)
[![dependency status](https://deps.rs/repo/github/jcornaz/autorel/status.svg)](https://deps.rs/repo/github/jcornaz/autorel)
[![Build](https://github.com/jcornaz/autorel/actions/workflows/build.yml/badge.svg)](https://github.com/jcornaz/autorel/actions/workflows/build.yml)
[![Zenhub](https://img.shields.io/badge/workspace-zenhub-%236061be)](https://app.zenhub.com/workspaces/autorel-60980eaac1cd55000f3de46b/board)

A release automation CLI

## Usage

`autorel` parses the commit messages since the last version tag to decide if there is something to release.

It requires running from a git repository that follows the [conventional-commits convention](https://www.conventionalcommits.org).

This tools also expects to find a non-empty configuration file ('release.yml' by default) that defines
command-lines that should run as part of the release process.
See: https://github.com/jcornaz/autorel#Configuration

If there is something to release (according to the commits found since last release), it performs the following steps:

1. Compute next version number (according to the semantic versioning rules)
2. Runs user-defined verification command
3. Generate a changelog (can be disabled)
4. Run user-defined preparation command
5. Commit changes made during the preparation (and the changelog if generated)
6. Run user-defined publication command
7. Push git commits

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

## Installation

Binaries for linux (x64) will be downloadable from the [release page](https://github.com/jcornaz/autorel/releases).

Other platforms aren't supported yet.

## Configuration

By default, `autorel` expects to find a non-empty configuration file at `./release.yml`. The location of the
configuration file can be overridden via the command line option: `--config`.

Here are all options of the configuration file:

```yaml
# If a changelog should be generated. True by default
changelog: true

# Tag prefix. 'v' by default.
tag_prefix: v

commit:

  # Commit message to use, in case there is something to commit (see bellow).
  # All occurrences of "{version}" will be replaced by the version being released.
  # The following message is the default.
  message: "chore: release {version}"

  # List of files to commit after the `prepare` hook has run.
  # If after committing these files the directory is still dirty, the release process will fail.
  # By default it commits the 'CHANGELOG.md' file if the changelog generation is enabled.
  # If the changelog generation is disabled, it doesn't commit anything by default.
  files:
    - CHANGELOG.md

# Github repository on which releases should be created. Empty by default (not creating github releases)
github_repo: jcornaz/autorel

# The list of hooks `autorel` will invoke in case of a new release.
# They must all be `sh` command lines. (more interpreters may eventually be supported in the future)
# All occurrences of "{version}" will be replaced by the version being released.
# 
# No hook is registered by default
hooks:
  verify: # Last chance to verify everything is ready to be published 
    - echo Verify {version}

  prepare: # Prepare the release. Search-and-replace strings in README and docs for example.
    - echo Prepare {version}

  publsh: # Actually publish the release.
    - echo Publish {version}
```
