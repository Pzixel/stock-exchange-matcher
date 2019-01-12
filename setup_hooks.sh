#!/bin/sh
echo "#!/bin/bash
cargo fmt"  > .git/hooks/pre-commit

chmod +x .git/hooks/pre-commit

echo "Hooks updated"
