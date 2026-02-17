# Zed Extension for Gno

## Prerequisities

1. Install `gnopls`

```sh
# go install github.com/gnoverse/gnopls@latest (until https://github.com/gnoverse/gnopls/issues/46)
git clone https://github.com/gnoverse/gnopls
GOROOT=$(go1.24.13 env GOROOT) PATH=$GOROOT/bin:$PATH make install
```

## Installation

To install as a dev extension in Zed:

1. Clone this repo as `zed-gno`
1. Open zed
1. Run zed: install dev extension from the command palette
1. Select the `zed-gno` directory
