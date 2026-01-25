# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.0.7](https://github.com/zheylmun/compass_data/compare/v0.0.6...v0.0.7) - 2026-01-25

### Other

- Add release-plz configuration to repository
- Fix clippy lint for a value that it thought looked like Pi
- Enhance the readme
- Add a ton of tests for the new parsing
- Add cli binary for inspecting compass projects and surveys
- Fix warnings resulting from miete macros
- Major refactor to separate lexing and parsing for projects, adding in propper error reporting when encountering issues in the makefiles
- Ensure that dat files write out the parameters correctly
- Fix warnings
- Add support for project parameters
- Finish filling out project format doc
- Add survey format to survey parser
- Update to nom 8
- Add document describing project file format
- Add FileFormat struct to the Survey module
- Add test that runs additional projects from an environment variable driven directory
- Add caching to CI builds
