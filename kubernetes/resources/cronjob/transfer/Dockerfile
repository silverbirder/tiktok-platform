FROM mcr.microsoft.com/playwright:focal

RUN apt-get update && apt-get install -y python3-pip
COPY requirements.txt requirements.txt
RUN pip3 install -r requirements.txt
RUN python -m playwright install

# Essentially, let's mount it...
# COPY ./secrets/key.json /secrets/key.json
# ENV GOOGLE_APPLICATION_CREDENTIALS /secrets/key.json

# Temporary support. Once this one is merged, delete RUN
# @https://github.com/davidteather/TikTok-Api/pull/753
RUN pip3 install "git+https://github.com/BeatRoy/TikTok-Api.git@fix/403-error-cloudfront"

COPY . .

ENTRYPOINT ["python"]
CMD ["app.py"]