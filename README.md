# Unswayed Server

Unswayed is an Android Application providing encrypted photos backup and gallery app functionnalities.
This repo is the backend server of the application, intended for the Auth system + upload/download.

The project is still (as you can see) under development, there are no plans to release it to a large audience (maybe just for a few friends and if all goes well, a closed beta could be considered).

## Open Endpoints

Open endpoints require no Authentication.

* [Account Creation](readme/create.md) : `POST /create`
* [Access Token](readme/auth.md) : `POST /auth`
* [Refresh Token](readme/refresh.md) : `POST /refresh`

## Endpoints that require Authentication

Closed endpoints require a valid Token to be included in the header of the
request. A Token can be acquired from the Access Token above.

### Current User related

Each endpoint manipulates or displays information related to the User whose
Token is provided with the request:

#### Users informations
* [Me](readme/users/me.md) : `GET /api/users/me`
* [List Images](readme/users/mine.md) : `GET /api/users/mine`
* [List Images Paged](readme/users/mine_paged.md) : `GET /api/users/mine_paged`

#### Files related
* [Upload Images](readme/files/upload.md) : `POST /api/files/upload`
* [Get Image](readme/files/get.md) : `GET /api/files/get/{filename}`


## Server setup

You first need to install Rust (ofc)

``sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
``


Then you have to setup a postgresql server.
So add the repo to the apt list

``sh
$ sudo sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'
``

``sh
$ wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | sudo apt-key add -
``

``sh
$ sudo apt-get update
``

And finally install postgresql-12 (tested)

``sh
$ sudo apt-get install postgresql-12
``

In addition you have to install the diesel_cli using the following command

``sh
$ cargo install diesel_cli --no-default-features --features postgres
``

## Windows (WSL2) port forwarding
If you're working on Windows using WSL2 you might need to forward the port from the host to wsl.

``sh
$ netsh interface portproxy add v4tov4 listenport=8080 listenaddress=192.168.1.19 connectport=8080 connectaddress=172.20.14.205
``

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
[GNU AGPLv3](https://choosealicense.com/licenses/agpl-3.0/)