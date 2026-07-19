#!/usr/bin/env bash
# Workspace aliases for rust-aoc
# Usage: source .ws_aliases.sh

# Run cargo clippy with pedantic warnings on a workspace member.
#   Usage: cLint <member>
#   Example: cLint aoc-core
cLint() {
  if [ -z "$1" ]; then
    echo "Usage: cLint <member>" >&2
    return 1
  fi
  cargo clippy -p "$1" -- -W clippy::pedantic
}

# Fetch an Advent of Code puzzle input.
#   Usage: aocGI <year> <day>
#   Example: aocGI 2023 5
aocGI() {
  if [ -z "$1" ] || [ -z "$2" ]; then
    echo "Usage: aocGI <year> <day>" >&2
    return 1
  fi
  cargo run -p aoc-fetch -- --year "$1" --day "$2"
}

# Create and checkout a new branch then push it to the remote.
#   Usage: gitB <branch>
#   Example: gitB my_branch
gitB() {
  if [ -z "$1" ]; then
    echo "Usage: gitB <branch>" >&2
    return 1
  fi
  git checkout -b "$1"
  git push --set-upstream origin "$1"
}
