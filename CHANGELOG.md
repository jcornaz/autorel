## 0.1.0 - 2021-05-15

### Features

* Upload files to github release
* Tag release
* Fail after preparation step if repository is dirty
* Commit and push changes made during the preparation phase
* Improve trim changelog output in terminal
* Create github release
* Append changelog to CHANGELOG.md if clog isn't configured by the user
* --dry-run flag
* Allow to confiigure tag prefix (#14)
* Changelog generation (powered by clog)
* Configuration file (#4)
* Infer version to release
* Fail if any release script fails
* Execute release scripts only if there are public changes
* Run release scripts if they exist
* Command line interface


### Bug fixes

* Dry-run failing if git user isn't defined
* Dry-run failures due to git manipulations
* Return non-zero status code in case of failure


