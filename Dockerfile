FROM rust:1.66 as builder
EXPOSE 3000
RUN apt-get update && apt-get install -y libssl-dev pkg-config && rm -rf /var/lib/apt/lists/*
RUN cargo install --force --version=0.6.2 --target-dir ~/.sqlx-cli sqlx-cli
ARG DATABASE_URL
COPY ./migrations/ /migrations
COPY ./scripts/init.sh ./init.sh
RUN chmod +x ./init.sh
COPY ./target/release/service-product /service-product
CMD ["bash", "./init.sh"]
