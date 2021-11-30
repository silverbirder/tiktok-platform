```
$ python3 -m venv .venv
(.venv) $ pip install -r requirements.txt 
(.venv) $ python app.py
```

```
$ docker build -t transfer .
$ docker run --rm transfer
$ docker run -it --rm --entrypoint "bash" transfer
```