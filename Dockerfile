FROM archlinux:latest

WORKDIR /jpp

RUN pacman -Syyu --noconfirm
RUN pacman -S make --noconfirm
RUN pacman -S gcc --noconfirm
RUN pacman -S gcc-ada --noconfirm
RUN pacman -S rust --noconfirm
RUN pacman -S jdk-openjdk --noconfirm

COPY . .
