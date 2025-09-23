# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.187.0/containers/rust/.devcontainer/base.Dockerfile

FROM mcr.microsoft.com/vscode/devcontainers/rust:2-1

WORKDIR /tmp

RUN wget https://github.com/exercism/cli/releases/download/v3.0.13/exercism-linux-64bit.tgz && \
    tar xzf exercism-linux-64bit.tgz && \
    mv exercism /usr/local/bin/