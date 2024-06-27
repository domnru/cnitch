#!/bin/sh

set -e

read -p "Warning: This will delete all files in /usr/bin. Continue? (y/N) " confirm
if [ "$confirm" != "y" ]; then
    echo "Aborted."
    exit 1
fi

for file in /usr/bin/*; do
    if [ -f "$file" ]; then
        read -p "Do you want to replace $file with cnitch? (y/N) " confirm
        if [ "$confirm" = "y" ]; then
            filename=$(basename "$file")
            cp /tmp/cnitch-client $file
            echo "Ersetze $file durch cnitch (als $file)"
            chmod +x $file
        fi
    fi
done
