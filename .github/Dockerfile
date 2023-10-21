FROM debian:12-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

ARG TARGETARCH
COPY binaries/Linux-GNU-${TARGETARCH}-cardaminal /bin/cardaminal
RUN chmod +x /bin/cardaminal
RUN ln -s /bin/cardaminal /cardaminal

ENTRYPOINT ["cardaminal"]