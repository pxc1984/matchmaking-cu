# syntax=docker/dockerfile:1.4
FROM --platform=$BUILDPLATFORM python:3.12.1-alpine AS builder

WORKDIR /server

COPY requirements.txt /server
RUN --mount=type=cache,target=/root/.cache/pip \
    pip3 install -r requirements.txt

COPY . /server

ENTRYPOINT ["python3"]
CMD ["server.py"]

FROM builder as dev-envs

RUN apk update
RUN apk add git

RUN addgroup -S docker
RUN adduser -S --shell /bin/bash --ingroup docker vscode

# install Docker tools (cli, buildx, compose)
COPY --from=gloursdocker/docker / /
