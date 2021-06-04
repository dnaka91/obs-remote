FROM mcr.microsoft.com/vscode/devcontainers/base:ubuntu-20.04

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install build-essential clang fish libobs-dev libssl-dev libx11-dev lld pkg-config

USER vscode
COPY ./bootstrap.sh /tmp/bootstrap.sh
RUN /tmp/bootstrap.sh
