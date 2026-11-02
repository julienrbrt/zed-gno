# Zed Extension for Gno

## Prerequisites

1. Install `gnopls`:

```sh
go install github.com/gnoverse/gnopls@latest
```

## Running tests

Zed play buttons (runnables) need per-language code built into Zed itself, so
extensions cannot provide them. Use tasks instead. Add these to your project
`.zed/tasks.json` or the global `tasks.json`:

```json
[
  {
    "label": "gno test package",
    "command": "gno test ./...",
    "tags": ["gno-test-package"]
  },
  {
    "label": "gno update golden files",
    "command": "gno test -update-golden-tests ./...",
    "tags": ["gno-test-update"]
  },
  {
    "label": "gno lint",
    "command": "gno lint ./...",
    "tags": ["gno-lint"]
  }
]
```

Run them with `task: spawn`. Use `$ZED_RELATIVE_FILE` if you want to scope a
task to the file you are editing.

## Installation

To install as a dev extension in Zed:

1. Clone this repo as `zed-gno`
1. Open zed
1. Run zed: install dev extension from the command palette
1. Select the `zed-gno` directory
