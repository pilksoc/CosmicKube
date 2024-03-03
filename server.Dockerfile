FROM alpine:3.19.1

RUN apk update && apk add ca-certificates && apk cache clean
WORKDIR /usr/local/cosmic_kube
COPY backend/target/release/cosmic_kube_amd64 /usr/local/bin/cosmic_kube

EXPOSE 8000

CMD ["cosmic_kube"]