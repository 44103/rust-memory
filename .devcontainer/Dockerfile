FROM rust:latest

RUN export DEBIAN_FRONTEND="noninteractive" && \
  apt-get update && apt-get -y install --no-install-recommends \
  git \
  zsh \
  vim \
  clang curl cmake \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*

run rustup component add rustfmt

ENV TZ=Asia/Tokyo
RUN zsh -c "`curl -L raw.github.com/44103/dotfiles/main/install.sh`"
