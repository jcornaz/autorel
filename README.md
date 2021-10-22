# autorel

[![License](https://img.shields.io/badge/license-Unlicense%20OR%20MIT-green)](#License)
[![Release](https://img.shields.io/github/v/release/jcornaz/autorel?sort=semver)](https://github.com/jcornaz/autorel/releases)
[![Dependencies](https://deps.rs/repo/github/jcornaz/autorel/status.svg)](https://deps.rs/repo/github/jcornaz/autorel)
[![Build](https://img.shields.io/github/workflow/status/jcornaz/autorel/Build/main)](https://github.com/jcornaz/autorel/actions/workflows/build.yml)
[![Zenhub](https://img.shields.io/badge/workspace-zenhub-%236061be)](https://app.zenhub.com/workspaces/autorel-60980eaac1cd55000f3de46b/board)

A release automation CLI

## Status

**This project is discontinued.**

I recommend to manually [keep a changelog](https://keepachangelog.com), manually *trigger* the releases and automate the publishing process using tools like [cargo-release](github.com/crate-ci/cargo-release), [action-gh-release](https://github.com/softprops/action-gh-release), [extract-release-notes](https://github.com/ffurrer2/extract-release-notes) or [jreleaser](https://jreleaser.org).



## Usage

Given a git repository that follows [conventional-commits convention](https://www.conventionalcommits.org),
`autorel` parses the commit messages since the last version tag to decide if there is something to release.

If there is indeed something to release, it performs the following steps:

1. Compute next version number (according to the semantic versioning rules)
2. Run user-defined verification commands (see configuration file)
3. Update changelog file (can be disabled)
4. Run user-defined preparation commands (see configuration file)
5. Update git repository (commit user-defined files, tag and push)
6. Run user-defined publication commands (see configuration file)
7. Create a github release (only if configured)

Any failure in one of these steps will abort the release process.

This tools also expects to find a non-empty configuration file ('release.yml' by default) that defines command-lines
that should run as part of the release process. See: https://github.com/jcornaz/autorel#Configuration

```
USAGE:
    autorel [FLAGS] [OPTIONS]

FLAGS:
        --dry-run
            Only prints what would be done if the this flag wasn't specified. Without actually doing
            anything

        --force
            Force to proceed with the release, even if no previous version was found in the tags

    -h, --help
            Prints help information

        --stable
            Ensure to release a stable version number (>= 1.0.0)

    -V, --version
            Prints version information


OPTIONS:
        --config <config>
            Path of the configuration file [default: release.yml]
```

## Installation

Binaries for linux (x64) are downloadable from the [release page](https://github.com/jcornaz/autorel/releases).

Other platforms aren't supported yet.

## Run from CI environment

Some CI environments (like github-actions) have defaults that can prevent `autorel` to work correctly . So when running from a CI, make sure to:

* Make a deep clone of the repository (fetching all history)
* Fetch the tags

By precaution, `autorel` will fail if it cannot finds the previous version in tags. Because that
would likely be due to a misconfiguration of the CI job.

If there isn't any previous version because you are actually using `autorel` to release the first version of your software/library,
you can use flag `--force` to proceed with the release, even if the previous version is not found.

## Configuration

By default, `autorel` expects to find a non-empty configuration file at `./release.yml`. The location of the
configuration file can be overridden via the command line option: `--config`.

Here are all options of the configuration file:

```yaml
# If a changelog file (CHANGELOG.md) should be generated/updated. True by default
changelog: true

# Tag prefix. 'v' by default.
tag_prefix: v

commit:

  # Commit message to use, in case there is something to commit (see bellow).
  # All occurrences of "{{version}}" will be replaced by the version being released.
  # The following message is the default.
  message: "chore: release {{version}}"

  # List of files to commit after the `prepare` hook has run.
  # If after committing these files the directory is still dirty, the release process will fail.
  # By default it commits the 'CHANGELOG.md' file if the changelog generation is enabled.
  # If the changelog generation is disabled, it doesn't commit anything by default.
  files:
    - CHANGELOG.md

# Github release configuration. Empty by default (not creating github releases)
# Note that publishing a github release requires a valid github token to be available in the`GITHUB_TOKEN` environment variable.
github:

  # Github repository
  repo: jcornaz/autorel

  # Files to upload to the github release. Empty by default
  files:
    - LICENSE
    - target/release/autorel


# The list of hooks `autorel` will invoke in case of a new release.
# They must all be valid `sh` command lines. (more shells may eventually be supported in the future)
# All occurrences of "{{version}}" will be replaced by the version being released.
# 
# No hook is registered by default
hooks:
  verify: # Last chance to verify everything is ready to be published 
    - echo Verify {{version}}

  prepare: # Prepare the release.
    - echo Prepare {{version}}

  publsh: # Actually publish the release.
    - echo Publish {{version}}
```

## License

Licensed under either of

* Unlicense, ([UNLICENSE](UNLICENSE) or http://unlicense.org)
* MIT ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
