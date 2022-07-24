# FROM rust:1.62-slim-buster

# WORKDIR /app
# COPY . .

# RUN cargo install --path .

# EXPOSE 8000

# ENTRYPOINT ["cam-server-syncer"]

FROM rust:1.62-slim-buster
WORKDIR /usr/src

# Create blank proj
RUN USER=root cargo new cam-server-snycer

# We want dependencies cached, so copy those first.
COPY Cargo.toml /usr/src/cam-server-snycer/
COPY Cargo.lock /usr/src/cam-server-snycer/

WORKDIR /usr/src/cam-server-snycer

# This is a dummy build to get the dependencies cached.
RUN cargo build --release

RUN rm -rf src

# Now copy in the rest of the sources
COPY src /usr/src/cam-server-snycer/src/
COPY Cargo.toml /usr/src/cam-server-snycer/
COPY Cargo.lock /usr/src/cam-server-snycer/
COPY Rocket.toml /usr/src/cam-server-snycer/

# This is the actual build.
RUN cargo build --release \
    && mv target/release/cam-server-syncer /bin

EXPOSE 8000

CMD ["/bin/cam-server-syncer"]