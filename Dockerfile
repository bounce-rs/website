FROM debian:bookworm-slim

COPY build/backend/bounce-website-server /opt/

CMD ["/opt/bounce-website-server", "--listen-addr", "0.0.0.0:5000"]
