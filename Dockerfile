FROM ubuntu:latest
LABEL authors="andrewozniack"

ENTRYPOINT ["top", "-b"]