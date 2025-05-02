FROM rust:1.76

# Install dependencies
RUN apt-get update && apt-get install -y cmake pkg-config libssl-dev git clang curl

# Create app directory
WORKDIR /app

# Copy project files
COPY . .

# Build project
RUN cargo build --release

# Run the node
CMD ["./target/release/my-substrate-chain", "--dev", "--ws-external"]
