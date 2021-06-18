#!/bin/sh

while [ ! -e /var/log/shared/stdout.pipe ]; do
    sleep 1
done

cat /var/log/shared/stdout.pipe | vector



