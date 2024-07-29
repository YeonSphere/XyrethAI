#!/bin/bash

# Update package list and upgrade all packages
sudo pacman -Syu --noconfirm

# Install required libraries
sudo pacman -S --noconfirm python-pip python-virtualenv
sudo pacman -S --noconfirm alsa-lib portaudio ffmpeg
sudo pacman -S --noconfirm openblas lapack gfortran
sudo pacman -S --noconfirm hdf5
sudo pacman -S --noconfirm libjpeg-turbo zlib freetype2 lcms2 openjpeg2 libtiff

# Install Python packages
pip install pygame speechrecognition pyttsx3 tensorflow openai-whisper unittest wave

echo "All dependencies installed successfully."
