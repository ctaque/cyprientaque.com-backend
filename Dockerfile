FROM node:11.15.0 AS frontend
COPY . .
ENV REACT_APP_JWT_SECRET=$REACT_APP_JWT_SECRET
#build frontend
RUN REACT_APP_JWT_SECRET=$REACT_APP_JWT_SECRET npm --prefix ./static/react/ run build

FROM rust@sha256:80f2e747f4d6b572e79b845df87b47dd9102f236113efd8921a115f4515b7df1
COPY . .
COPY --from=frontend --chown=root:root ./static ./static
RUN cargo build --release

EXPOSE 8088
CMD ["./target/release/ctprods", "listen"]
