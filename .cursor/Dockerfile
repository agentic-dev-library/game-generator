# Use official Rust 1.92.0 (latest stable) image as base
FROM rust:1.92.0-trixie

WORKDIR /workspace

# Install system dependencies for Bevy in a single layer
RUN apt-get update -qq && \
    apt-get install -yqq --no-install-recommends \
    # Build essentials
    build-essential \
    pkg-config \
    libssl-dev \
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
    && rm -rf /var/lib/apt/lists/*
