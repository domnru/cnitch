#!/bin/sh

set -e

# Container-IDs extrahieren
container_names=$(docker ps --format "{{.Names}}")
target_dir='/tmp/bin_switcher.sh'

# Über jede ID iterieren und docker exec ausführen
for container_name in $container_names; do
    read -p "Do you want to monitor $container_name (y/N)" confirm
    if [ "$confirm" != "y" ]; then
        echo "Skipped $container_name."
    else
        docker cp ./client/target/release/cnitch-client $container_name:/tmp/cnitch-client
        docker cp ./bin_switcher.sh $container_name:$target_dir
        echo "test"
        docker exec -it $container_name sh -c "chmod +x $target_dir && sh $target_dir"
    fi
done