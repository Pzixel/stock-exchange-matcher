#!/bin/sh
echo "cargo fmt"  > .git/hooks/pre-commit

chmod +x .git/hooks/pre-commit

echo "Hooks updated"
