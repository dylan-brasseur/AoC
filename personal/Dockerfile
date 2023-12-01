FROM python:3.11-slim
RUN apt-get update && apt-get install -y curl && apt-get clean
RUN pip install --no-cache-dir advent-of-code-data
COPY --chmod=755 bootstrap.sh .

ENTRYPOINT ["./bootstrap.sh"]