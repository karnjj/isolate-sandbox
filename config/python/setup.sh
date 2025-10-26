#!/bin/bash

SCRIPT_DIR=$(dirname "$0")

echo "Setting up python environment..."

# ensure python3 is installed
if ! command -v python3 &> /dev/null; then
    echo "python3 could not be found"
    exit 1
fi

# create environment
python3 -m venv ~/.isolate-sandbox/environment/python

# install packages
~/.isolate-sandbox/environment/python/bin/pip install -r $SCRIPT_DIR/requirements.txt

echo "Done"