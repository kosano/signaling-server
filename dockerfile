FROM kosano/ubuntu-cargo:0.1.1

WORKDIR /cache 
COPY . .
RUN cargo build --release
WORKDIR /work
RUN cp target/release/signaling_server /work
RUN rm -rf /cache
EXPOSE 8080
CMD ["/work/signaling_server"]