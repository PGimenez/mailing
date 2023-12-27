FROM rust:latest
WORKDIR /app
COPY mailing/target/release/mailing /app/
EXPOSE 8080
CMD ["./mailing"]
