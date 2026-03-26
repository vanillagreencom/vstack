#!/bin/bash
# Script: name
# Usage: script-name <command> [options]
#
# Self-contained — no project-specific paths. Resolves context from git root
# or environment variables.

set -euo pipefail

case "${1:-}" in
  command-name)
    # Implementation
    ;;
  *)
    echo "Usage: $0 command-name [options]" >&2
    exit 1
    ;;
esac
