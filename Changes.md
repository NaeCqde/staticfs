# Changes

## Unreleased

- Security
  - A symbolic link that leads out of the served directory is no longer followed.
  - The page of a directory is our own: a quote or `#` in a file name no longer breaks the link or adds attributes to it.
  - The page has a Content-Security-Policy, and every response has `nosniff`.
- A wrong command line exits with a code other than 0 (2); `--help` added; a failure to start exits with 1.
- Code is split into modules and has unit tests.
- The macOS x86_64 build is gone.
