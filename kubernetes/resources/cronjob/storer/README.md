```
$ docker build -t scraper .
$ docker run -it --env DB_HOST=host.docker.internal --rm --entrypoint "bash" scraper
$ docker run -it --env DB_HOST=host.docker.internal --rm scraper
```