#!/bin/sh

set -e 

image_name='cnitch_client_binary:do_not_use'
name='cnitch-client'


docker build -t $image_name -f Dockerfile . 
docker run --name $name -d $image_name sleep infinity
docker cp $name:/app/$name $name
docker stop $name && docker rm $name