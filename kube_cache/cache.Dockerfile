FROM debian:12.5-slim

RUN apt update && apt install ca-certificates -y && apt upgrade -y
WORKDIR /usr/local/kube_cache
COPY kube_cache/kube_cache /usr/local/bin/kube_cache
COPY default.png /usr/local/bin/kube_cache

EXPOSE 8000

CMD ["kube_cache"]
