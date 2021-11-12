```
cargo run
```

```
$ docker build -t storer .
$ docker run -it --rm --entrypoint "bash" storer
$ docker run -it --rm storer
```

```
docker run --name some-mysql -e MYSQL_ROOT_PASSWORD=XXXX -p 3306:3306 -d mysql:5.7
mysql -u root -h 127.0.0.1  -p
mysql> CREATE DATABASE tiktok;
```