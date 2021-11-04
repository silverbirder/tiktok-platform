```
$ docker build -t storer .
$ docker run -it --env DB_HOST=host.docker.internal --rm --entrypoint "bash" storer
$ docker run -it --env DB_HOST=host.docker.internal --rm storer
```