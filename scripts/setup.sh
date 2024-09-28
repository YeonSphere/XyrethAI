#!/bin/bash

# Seokjin AI Setup Script

# Exit immediately if a command exits with a non-zero status
set -e

echo "Starting Seokjin AI setup..."

# Create necessary directories
mkdir -p data/{train,val,test}
mkdir -p models
mkdir -p logs

# Install dependencies
echo "Installing dependencies..."
pip install -r requirements.txt

# Set up configuration
echo "Setting up configuration..."
cp config/config.yaml.example config/config.yaml
cp config/logging.yaml.example config/logging.yaml

# Initialize the database (if applicable)
# echo "Initializing database..."
# python scripts/init_db.py

# Download pre-trained models (if applicable)
# echo "Downloading pre-trained models..."
# python scripts/download_models.py

# Set up environment variables
echo "Setting up environment variables..."
cp .env.example .env
echo "Please edit .env file with your specific settings."

# Set up git hooks (optional)
# echo "Setting up git hooks..."
# cp scripts/pre-commit .git/hooks/pre-commit
# chmod +x .git/hooks/pre-commit

echo "Seokjin AI setup completed successfully!"
echo "You can now start using Seokjin AI. Refer to the documentation for next steps."
