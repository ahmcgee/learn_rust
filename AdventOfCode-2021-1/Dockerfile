FROM rust:1.68 as builder

WORKDIR /usr/src/advent_of_code_2021_1
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/advent_of_code_2021_1/target/release/advent_of_code_2021_1 /usr/local/bin/advent_of_code_2021_1
COPY --from=builder /usr/src/advent_of_code_2021_1/data/depths.txt /root/depths.txt
CMD ["advent_of_code_2021_1", "/root/depths.txt"]