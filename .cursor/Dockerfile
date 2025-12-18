# Use official Rust 1.92.0 (latest stable) image as base
FROM rust:1.92.0-bookworm

# Set environment variables
ENV CARGO_HOME=/usr/local/cargo
ENV RUSTUP_HOME=/usr/local/rustup
ENV PATH=/usr/local/cargo/bin:$PATH
ENV RUST_BACKTRACE=1
ENV DEBIAN_FRONTEND=noninteractive

# Install system dependencies in a single layer
RUN apt-get update -qq && \
    apt-get install -yqq --no-install-recommends \
    # Build essentials
    build-essential \
    pkg-config \
    # SSL/TLS
    libssl-dev \
    ca-certificates \
    # Graphics/Windowing dependencies for Bevy
    libudev-dev \
    libasound2-dev \
    libx11-dev \
    libxi-dev \
    libgl1-mesa-dev \
    libglu1-mesa-dev \
    libxcursor-dev \
    libxrandr-dev \
    libxinerama-dev \
    libwayland-dev \
    libxkbcommon-dev \
    libvulkan-dev \
    libwayland-cursor0 \
    libwayland-egl1 \
    mesa-vulkan-drivers \
    # Development tools
    curl \
    git \
    sudo \
    && rm -rf /var/lib/apt/lists/*

# Add rustfmt and clippy components
RUN rustup component add rustfmt clippy

# Install cargo-watch for development convenience
RUN cargo install cargo-watch

# Create a non-root user
RUN useradd -m -s /bin/bash ubuntu && \
    echo 'ubuntu ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers && \
    chown -R ubuntu:ubuntu /usr/local/cargo

# Switch to the ubuntu user
USER ubuntu
WORKDIR /home/ubuntu

# Set up PATH for the ubuntu user
ENV PATH="/home/ubuntu/.cargo/bin:/usr/local/cargo/bin:${PATH}"
RUN echo 'export PATH="/home/ubuntu/.cargo/bin:/usr/local/cargo/bin:$PATH"' >> /home/ubuntu/.bashrc && \
    echo 'export CARGO_HOME=/usr/local/cargo' >> /home/ubuntu/.bashrc && \
    echo 'export RUSTUP_HOME=/usr/local/rustup' >> /home/ubuntu/.bashrc

# Default command
CMD ["/bin/bash", "-l"]
