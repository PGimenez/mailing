FROM rust:latest
WORKDIR /app
COPY target/release/mailing /app/
EXPOSE 8080
ENV RUST_LOG=info
CMD ["./mailing"]
