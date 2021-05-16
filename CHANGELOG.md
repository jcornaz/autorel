## 0.1.1 - 2021-05-16

### Features

* Add `--stable` flag to force releasing a version number >= 1.0.0
* Improve messages printed in terminal
* Fail if previous version is not found in tags



## 0.1.0 - 2021-05-15

### Features

* Upload files to github release (if configured)
* Create release tag
* Fail after preparation step if repository is dirty
* Commit and push changes made during the preparation phase
* Create github release
* `--dry-run` flag
* Allow to confiigure tag prefix (#14)
* Generate a changelog
* Infer version to release
* Execute a release only if there are public changes
