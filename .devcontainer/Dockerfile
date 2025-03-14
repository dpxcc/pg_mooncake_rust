FROM mcr.microsoft.com/devcontainers/rust:latest

RUN apt update \
 && apt install -y \
    bison \
    cmake \
    flex \
    libclang-dev \
    liblz4-dev \
 && rm -rf /var/lib/apt/lists/*

USER vscode

RUN cargo install --locked cargo-pgrx \
 && cargo pgrx init
