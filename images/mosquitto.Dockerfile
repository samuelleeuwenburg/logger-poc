FROM eclipse-mosquitto:latest
WORKDIR /mosquitto/config
COPY --chown=mosquitto:mosquitto ./mosquitto/config .