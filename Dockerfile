FROM rust

WORKDIR /usr/jpp

RUN apt-get update
RUN apt-get -y upgrade
RUN apt-get install gnat -y
RUN apt-get install -y gprbuild
RUN apt-get install golang -y
RUN apt-get install ghc ghc-prof ghc-doc -y
RUN apt-get install swi-prolog -y
RUN apt-get install default-jre -y

COPY . .

# FROM archlinux

# WORKDIR /usr/jpp

# RUN pacman -Sy archlinux-keyring --noconfirm
# RUN pacman -Syyu --noconfirm

# RUN pacman -S sudo debugedit fakeroot git make --noconfirm

# RUN pacman -S gcc --noconfirm
# RUN pacman -S gcc-ada --noconfirm

# RUN useradd -m builduser
# RUN chown -R builduser:builduser /usr/jpp
# USER builduser

# RUN git clone https://aur.archlinux.org/gprbuild.git
# RUN cd gprbuild && makepkg -si --noconfirm --log && cd .. && rm -r gprbuild

# USER root

# RUN pacman -S rust --noconfirm
# RUN pacman -S jdk-openjdk --noconfirm

# COPY . .
