## 0.1.4 - 2021-05-24

### Bug fixes

* Include all breaking change footers in changelog (#39)
* Order the changelog entries after the git commit time



## 0.1.3 - 2021-05-21

### Bug fixes

* Error when a command line contains the `'` character
* Incorrect version displayed in CLI --help



## 0.1.2 - 2021-05-17

### Features

* Don't fail if git user is not configured


### Bug fixes

* Use a single shell instance per release phase (#34)
* Publishing phase should run after git push (#33)



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
