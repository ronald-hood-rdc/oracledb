# Build stage for compiling the Rust application
FROM rust:1.67 as builder
WORKDIR /usr/src/oracledb

# Copy the Cargo.toml and Cargo.lock files to leverage Docker cache
COPY Cargo.toml Cargo.lock ./
# Create a dummy source file to compile dependencies
RUN mkdir src && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
# Build only the dependencies to cache them
RUN cargo build --release
# Remove the dummy source file
RUN rm src/*.rs

# Now copy your actual source code
COPY . .
# Compile the actual application
RUN touch src/main.rs && \
    cargo install --path .

# Final stage that includes only the runtime dependencies
FROM oraclelinux:8
ARG release=19
ARG update=21

# Install Oracle Instant Client
RUN dnf -y install oracle-release-el8 && \
    dnf -y install oracle-instantclient${release}.${update}-basic \
                   oracle-instantclient${release}.${update}-devel \
                   oracle-instantclient${release}.${update}-sqlplus && \
    rm -rf /var/cache/dnf

# Copy the compiled Rust application from the builder stage
COPY --from=builder /usr/local/cargo/bin/oracledb /usr/local/bin/oracledb

# Set the default command to run your application
CMD ["/usr/local/bin/oracledb"]
