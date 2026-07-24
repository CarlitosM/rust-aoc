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

# Run tests for a aoc year and (optional) day and optional (part)
#   Usage: aocTest <year> [day] [part]
#   Example: aocTest 2023 05 2
aocTest() {
  if [ -z "$1" ]; then
    echo "Usage: aocTest <year> [day] [part]" >&2
    return 1
  fi

  local year="$1"
  local day="${2:-}"
  local part="${3:-}"

  if [ -n "$day" ] && [ -n "$part" ]; then
    cargo test -p "aoc-${year}" "day${day}::tests_part${part}"
  elif [ -n "$day" ]; then
    cargo test -p "aoc-${year}" "day${day}"
  else
    cargo test -p "aoc-${year}"
  fi
}

# Run a solution for a given year, day and part
# and optionally submit the result to AOC
#   Usage: aocRun <year> <day> <part> [--submit]
#   Example: aocRun 2023 05 2 --submit
aocRun() {
  if [ -z "$1" ] || [ -z "$2" ] || [ -z "$3" ]; then
    echo "Usage: aocRun <year> <day> <part> [--submit]" >&2
    return 1
  fi

  local year="$1"
  local day="$2"
  local part="$3"
  local submit="${4:-}"

  if [ -n "$submit" ] && [ "$submit" != "--submit" ]; then
    echo "Invalid submit value: $submit" >&2
    echo "Usage: aocRun <year> <day> <part> [--submit]" >&2
    return 1
  fi

  if [ "$submit" = "--submit" ]; then
    cargo run -p "aoc-${year}" --bin run -- "$day" "$part" --submit
  else
    cargo run -p "aoc-${year}" --bin run -- "$day" "$part"
  fi
}

# Scaffold a new AOC year or day for a given year
#   Usage: aocNew <year> [day]
#   Example: aocNew 2023 05
aocNew() {
  if [ -z "$1" ]; then
    echo "Usage: aocNew <year> [day] [-l]" >&2
    return 1
  fi

  local year="$1"
  local day="${2:-}"

  if [ -n "$day" ]; then
    cargo run -p "aoc-scaffold" -- --year "$year" --day "$day"
  else
    cargo run -p "aoc-scaffold" -- --year "$year"
  fi

  if [ "$3" = "-l" ]; then
    if [ -n "$day" ]; then
      aocGI "$year" "$day"
    else
      aocGI "$year" 1
    fi
  fi
}
