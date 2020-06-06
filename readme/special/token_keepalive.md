# Special Token Keep Alive

Add 6 more hours to your special token.

**URL** : `/api/token/token_keepalive`

**Method** : `PATCH`

**Auth required** : YES

**Header type** : Bearer special_token

## Success Response

**Code** : `200 OK`

**Response example**

```json
{
	"id": 5,
	"user_id": 1,
	"token_type": 2,
	"access_token": "25y4wHPlq0MCUxvoW5U36O3ZoFe73n9F_dTJ9Ei_b80el2G0NNosN7R7S3z2sR9f",
	"refresh_token": "none",
	"created_at": "2020-06-04T11:49:51.926297",
	"expire_at": "2020-06-04T23:49:51.926297"
}
```

## Error Response

**Condition** : If the token is invalid or expired

**Code** : `401 UNAUTHORIZED`

**Content** :

```json
{
  "error": "The token is invalid or has been expired"
}
```

**Condition** : If the token is not a Special Token

**Code** : `400 BAD REQUEST`

**Content** :

```json
{
  "error": "The provided token is not a special token"
}
```