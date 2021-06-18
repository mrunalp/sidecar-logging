#!/bin/bash

exec 1> /var/log/stdout.log
exec 2> /var/log/stderr.log

exec ./target/debug/sidecar-logging
