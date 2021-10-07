FROM ubuntu:latest
MAINTAINER Mahdi Sheikh Hosseini <mah454>

RUN apt-get update ; apt-get -y install libssl1.1 

COPY rust-robot /

ENTRYPOINT ["/rust-robot"]
