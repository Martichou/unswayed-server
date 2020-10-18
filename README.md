# Unswayed Server

Unswayed is an Android Application providing encrypted photos backup and gallery app functionnalities.
This repo is the backend server of the application, intended for the Auth system + upload/download.

The project is still (as you can see) under development, there are no plans to release it to a large audience (maybe just for a few friends and if all goes well, a closed beta could be considered).

## Endpoints

The docs for the endpoints are available here [goto docs](readme)

## Server setup / Dev setup

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
$ sudo apt-get install postgresql-12 libpq-dev
$ cargo install diesel_cli --no-default-features --features postgres
$ diesel setup
```

## Windows (WSL2) port forwarding
If you're working on Windows using WSL2 you might need to forward the port from the host to wsl.

```bash
netsh interface portproxy add v4tov4 listenport=8080 listenaddress=192.168.1.19 connectport=8080 connectaddress=172.20.14.205
```

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
[GNU AGPLv3](https://choosealicense.com/licenses/agpl-3.0/)