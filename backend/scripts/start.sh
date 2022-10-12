#!/usr/bin/env sh

screen -S Surreal -dm bash -c "surreal start --bind 127.0.0.1:8004 --user root --pass root file:surreal_data"
