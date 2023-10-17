FROM rust:1.73

RUN apt update && apt install -y --no-install-recommends python3 python3-pip python3-venv

WORKDIR /app

# Context is still the package 
COPY . .

RUN ls -lah
# Create a new user and set permissions
RUN useradd -m cftime_rs && chown -R cftime_rs:cftime_rs /app
USER cftime_rs

# Create and activate a virtual environment
RUN python3 -m venv .venv

# Install Maturin and build the Rust Python project
RUN .venv/bin/pip install --no-cache-dir maturin && \
    .venv/bin/maturin develop && \
    .venv/bin/pip install --no-cache-dir '.[dev]'

# Set the entry point (modify it as needed)
CMD [".venv/bin/pytest", "python/cftime_rs/tests"]
