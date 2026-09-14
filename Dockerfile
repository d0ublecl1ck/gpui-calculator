FROM rustlang/rust:nightly-bookworm
WORKDIR /app
COPY . .
RUN export http_proxy=http://127.0.0.1:7897 https_proxy=http://127.0.0.1:7897 && \
    apt-get update && apt-get install -y --no-install-recommends \
      libxkbcommon-dev libxkbcommon-x11-dev libfontconfig1-dev libfreetype6-dev libxcb1-dev \
    && rm -rf /var/lib/apt/lists/* && cargo test
