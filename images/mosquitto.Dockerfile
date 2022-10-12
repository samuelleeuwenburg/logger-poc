FROM eclipse-mosquitto:latest
WORKDIR /mosquitto/config
COPY ./mosquitto/config .