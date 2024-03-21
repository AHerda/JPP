# Use the official Ada base image
FROM archlinux:latest

# Set the working directory
WORKDIR /jpp

# Compile the Ada program
RUN pacman -Syu --noconfirm
RUN pacman -S make --noconfirm
RUN pacman -S gcc --noconfirm
RUN pacman -S gcc-ada --noconfirm
RUN pacman -S rust --noconfirm

# Copy the Ada source code to the container
COPY . .
