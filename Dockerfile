FROM rust:latest

WORKDIR /server
COPY . .

RUN cargo install --path .

EXPOSE 80

CMD ["http_server"]
# sudo docker run -p 80:80 -v /etc/httpd.conf:/etc/httpd.conf:ro -v /var/www/html:/var/www/html:ro --name http_server -t http_server
