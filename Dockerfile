FROM rust:latest
WORKDIR /app
COPY mailing/target/release/mailing /app/
EXPOSE 8080
ENV RUST_LOG=info
CMD ["./mailing"]
