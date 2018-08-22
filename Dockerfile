FROM debian:9.5-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY target/release/github-label-adder /usr/local/bin/github-label-adder

CMD /usr/local/bin/github-label-adder