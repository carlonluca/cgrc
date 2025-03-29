FROM carlonluca/docker-multiarch:noble
RUN apt-get update \
 && apt-get install -y build-essential curl \
 && curl https://sh.rustup.rs -sSf | bash -s -- -y \
 && . $HOME/.cargo/env \
 && rustup install 1.85.1 \
 && rustup target add x86_64-unknown-linux-musl \
 && rustup target add aarch64-unknown-linux-musl \
 && rustup target add armv7-unknown-linux-musleabihf \
 && apt-get clean
ENV PATH="/root/.cargo/bin:${PATH}"
