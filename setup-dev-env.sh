#!/bin/bash

# Sets up the needed development environment

echo "Setting up Sphinx-needs"

python3 -m venv venv
source venv/bin/activate

pip install sphinx
pip install sphinx sphinx-needs
pip inject sphinx sphinx-simplepdf
pip install weasyprint

echo "Done ✅"
