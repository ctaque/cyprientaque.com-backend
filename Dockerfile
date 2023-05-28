FROM rust@sha256:80f2e747f4d6b572e79b845df87b47dd9102f236113efd8921a115f4515b7df1
COPY ./target/release/ctprods .

EXPOSE 8088
CMD ["./ctprods", "listen"]
