```
$ python3 -m venv .venv
$ source ./bin/activate
$ pip3 install -r requirements.txt
$ FLASK_RUN_PORT=8000 python3 -m flask run --without-threads
```

```
$ docker build . -t tiktok-scraper-api:latest
$ docker run --rm -it -e FLASK_RUN_PORT=5000 -e FLASK_DEBUG=1 -p 5000:5000 tiktok-scraper-api:latest
```