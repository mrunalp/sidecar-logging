#!/bin/bash

mkfifo      /var/log/shared/stdout.pipe
chmod 777   /var/log/shared/stdout.pipe

exec 1> /var/log/shared/stdout.pipe

exec $@
