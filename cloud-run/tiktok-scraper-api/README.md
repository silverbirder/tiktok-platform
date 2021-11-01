```
$ python3 -m venv .venv
$ source ./bin/activate
$ pip3 install -r requirements.txt
$ python3 -m gunicorn --bind :5000 --workers 1 --threads 8 app:app
```

```
$ docker build . -t tiktok-scraper-api:latest
$ docker run --rm -it -e PORT=5000 -p 5000:5000 tiktok-scraper-api:latest
```