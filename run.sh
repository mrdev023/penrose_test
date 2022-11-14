#!/usr/bin/env bash
SCREEN_SIZE=1366x960
XDISPLAY=:2

touch xephyr.log

Xephyr +extension RANDR -screen ${SCREEN_SIZE} ${XDISPLAY} -ac &
XEPHYR_PID=$!

sleep 1
env DISPLAY=${XDISPLAY} cargo run --release 2>&1 xephyr.log &
WM_PID=$!

trap "kill $XEPHYR_PID && kill $WM_PID && rm xephyr.log" SIGINT SIGTERM exit

wait $WM_PID
kill $XEPHYR_PID