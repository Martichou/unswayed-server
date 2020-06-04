# Unswayed Server

Unswayed is an Android Application providing encrypted photos backup and gallery app functionnalities.
This repo is the backend server of the application, intended for the Auth system + upload/download.

The project is still (as you can see) under development, there are no plans to release it to a large audience (maybe just for a few friends and if all goes well, a closed beta could be considered).

## Open Endpoints

Open endpoints require no Authentication.

* [Account Creation](readme/users.md) : `POST /users`
* [Access Token](readme/auth.md) : `POST /auth`
* [Refresh Token](readme/refresh.md) : `POST /refresh`

## Endpoints that require Authentication

Closed endpoints require a valid Token to be included in the header of the
request. A Token can be acquired from the Access Token above.

### Current User related

Each endpoint manipulates or displays information related to the User whose
Token is provided with the request:

* [Me](readme/users/me.md) : `GET /api/me`
* [List Images](readme/users/mine.md) : `GET /api/mine`
* [Upload Images](readme/users/upload.md) : `POST /api/upload`
* [Get Image](readme/users/get.md) : `GET /api/get/{filename}`

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
[GNU AGPLv3](https://choosealicense.com/licenses/agpl-3.0/)