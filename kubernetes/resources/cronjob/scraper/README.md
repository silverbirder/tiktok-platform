```
$ python3 -m venv .venv
(.venv) $ pip install -r requirements.txt 
(.venv) $ python app.py
```

```
$ docker build -t scraper .
$ docker run --rm scraper
$ docker run --rm --entrypoint "bash" scraper
```