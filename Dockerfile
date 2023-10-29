FROM rust:1.67
COPY ./target/release/ctprods .

EXPOSE 8088
CMD ["./ctprods", "listen"]
