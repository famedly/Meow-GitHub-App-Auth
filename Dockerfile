FROM rust:1.71

RUN mkdir /meow-app-auth
COPY . /meow-app-auth
RUN cargo install --path /meow-app-auth

ADD entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
