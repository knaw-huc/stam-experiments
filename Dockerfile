FROM alpine:latest
#VERSION can be:
# - stable: builds latest stable versions from source (default)
# - devel: latest development version (git master/main branch) (not implemented yet!)
ARG VERSION="stable"
LABEL org.opencontainers.image.authors="Maarten van Gompel <proycon@anaproy.nl>"
LABEL description="STAM Experiments"

RUN apk add cargo rust jq make python3 && mkdir /work

COPY . /work/

RUN if [ "$VERSION" = "stable" ]; then \
        cargo install stam-tools && mv /root/.cargo/bin/* /usr/bin/; \
        make clean; \
    else \
        echo "not implemented yet"; \
    fi

WORKDIR /work

ENTRYPOINT [ "make" ]
