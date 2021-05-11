# Configuration file reference

By default, `autorel` expects to find a non-empty configuration file at `./release.yml`. The location of the
configuration file can be overridden via the command line option: `--config`.

Here are all options of the configuration file with their default:

```yaml
changelog: true # If a changelog should be generated. True by default
tag_prefix: v # Tag prefix. 'v' by default.


# The list of hooks `autorel` will invoke in case of a new release.
# They must all be `sh` compatible command lines.
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
