FROM debian:12.5-slim

RUN apt-get update
RUN apt-get install ca-certificates -y
RUN apt-get upgrade -y

RUN useradd -m app

USER app
WORKDIR /home/app
COPY kube_cache/kube_cache .
COPY kube_cache/default.png .

EXPOSE 8080

CMD ["./kube_cache"]
