FROM eclipse-mosquitto:latest
WORKDIR /mosquitto/config
COPY --chown=mosquitto:mosquitto ./mosquitto/config .
CMD ["/usr/sbin/mosquitto", "-c" "/mosquitto/config/mosquitto.conf"]