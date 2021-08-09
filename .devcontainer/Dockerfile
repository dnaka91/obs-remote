FROM mcr.microsoft.com/vscode/devcontainers/base:ubuntu-20.04

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && apt-get -y install \
    build-essential \
    clang \
    clang-format \
    fish \
    libssl-dev \
    libx11-dev \
    lld \
    pkg-config \
    protobuf-compiler \
    protobuf-compiler-grpc \
    software-properties-common

RUN export DEBIAN_FRONTEND=noninteractive &&  \
    add-apt-repository ppa:obsproject/obs-studio && \
    apt-get -y install obs-studio

ENV PROTOLINT_VERSION=0.32.0
RUN curl -L -o /tmp/protolint.tar.gz https://github.com/yoheimuta/protolint/releases/download/v${PROTOLINT_VERSION}/protolint_${PROTOLINT_VERSION}_Linux_x86_64.tar.gz && \
    tar -C /usr/local/bin -xvf /tmp/protolint.tar.gz protolint && \
    rm /tmp/protolint.tar.gz

USER vscode
COPY ./bootstrap.sh /tmp/bootstrap.sh
RUN /tmp/bootstrap.sh
