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