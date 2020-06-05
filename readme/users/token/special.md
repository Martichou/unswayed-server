# Special

The Special token is a token with a longer validity and which allow you to refresh the validity lifetime.

**URL** : `/api/token/special`

**Method** : `GET`

**Auth required** : YES

**Header type** : Bearer access_token

## Success Response

**Code** : `201 CREATED`

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