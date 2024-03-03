FROM debian:12.5-slim

RUN apt update && apt install ca-certificates && apt upgrade -y
WORKDIR /usr/local/cosmic_kube
COPY backend/target/release/cosmic_kube_amd64 /usr/local/bin/cosmic_kube

EXPOSE 8000

CMD ["cosmic_kube"]