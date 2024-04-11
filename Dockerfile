FROM rust

WORKDIR /usr/jpp

RUN apt-get update
RUN apt-get -y upgrade
RUN apt-get install gnat -y
RUN apt-get install -y gprbuild
RUN apt-get install golang -y
RUN apt-get install ghc ghc-prof ghc-doc -y
RUN apt-get install swi-prolog -y

COPY . .

# FROM testcab/yay

# WORKDIR /jpp

# RUN sudo pacman -Sy archlinux-keyring --noconfirm
# RUN sudo pacman -Syyu --noconfirm

# # RUN pacman -S --needed base-devel git --noconfirm
# # RUN mkdir -p /tmp/yay-build
# # RUN useradd -m -G wheel builder && passwd -d builder
# # RUN chown -R builder:builder /tmp/yay-build
# # RUN echo 'builder ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers
# # RUN su - builder -c "git clone https://aur.archlinux.org/yay.git /tmp/yay-build/yay"
# # RUN su - builder -c "cd /tmp/yay-build/yay && makepkg -si --noconfirm"
# # RUN rm -rf /tmp/yay-build

# RUN sudo pacman -S make --noconfirm
# RUN sudo pacman -S gcc --noconfirm
# RUN sudo pacman -S gcc-ada --noconfirm
# RUN yay -S gprbuild --noconfirm
# RUN sudo pacman -S rust --noconfirm
# RUN sudo pacman -S jdk-openjdk --noconfirm

# COPY . .
