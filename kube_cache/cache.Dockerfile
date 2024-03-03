FROM debian:12.5-slim

RUN apt update && apt install ca-certificates -y && apt upgrade -y
WORKDIR /usr/local/bin/kube_cache
COPY kube_cache/kube_cache /usr/local/bin/kube_cache
COPY kube_cache/default.png /usr/local/bin/kube_cache
RUN chmod u+x /usr/local/bin/kube_cache

EXPOSE 8080

CMD ["kube_cache"]
