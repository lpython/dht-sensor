# Create a custom Dockerfile based on mcr.microsoft.com/devcontainers/rust:latest
# with additional targets for embedded Rust, 'rustup update', installation of binutils,
# and installation of llvm-tools

FROM mcr.microsoft.com/devcontainers/rust:latest

USER vscode

WORKDIR /tmp

# Install additional targets for embedded Rust
RUN rustup target add thumbv6m-none-eabi thumbv7m-none-eabi riscv32imc-unknown-none-elf

# Install cargo-binutils and llvm-tools
RUN cargo install cargo-binutils
RUN rustup component add llvm-tools

USER root
# Install tealdeer (tldr)
RUN wget https://github.com/dbrgn/tealdeer/releases/download/v1.6.1/tealdeer-linux-x86_64-musl -O /usr/bin/tldr && chmod +x /usr/bin/tldr

# Update tldr database
RUN tldr --update
RUN tldr --update


# Install Python packages
RUN apt-get update && apt-get install -y python3-pip python3.11-venv

# Run 'rustup update' to ensure the latest Rust toolchain
RUN rustup update

USER vscode

# Set the default command to start a shell
CMD ["bash"]