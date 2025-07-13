FROM debian:latest

RUN apt-get update && apt-get -y install \
	curl \
	\
	build-essential \
	grub-common \
	grub-pc-bin \
	nasm \
	qemu-system-x86 \
	xorriso

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly-2025-07-01 --component rust-src

# RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
#
# RUN . "$HOME/.cargo/env"
#
# RUN rustup toolchain install nightly
#
# RUN rustup component add rust-src --toolchain nightly

