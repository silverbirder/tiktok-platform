FROM mcr.microsoft.com/playwright:focal

RUN apt-get update && apt-get install -y python3-pip
COPY requirements.txt requirements.txt
RUN pip3 install -r requirements.txt
RUN python -m playwright install

COPY . .

# Essentially, let's mount it...
COPY ./secrets/key.json /secrets/key.json
ENV GOOGLE_APPLICATION_CREDENTIALS /secrets/key.json

ENTRYPOINT ["python"]
CMD ["app.py"]