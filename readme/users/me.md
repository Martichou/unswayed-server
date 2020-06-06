# Me

Used to get the user_information.

**URL** : `/api/users/me`

**Method** : `GET`

**Auth required** : YES

**Header type** : Bearer access_token

## Success Response

**Code** : `200 OK`

**Response example**

```json
{
	"id": 2,
	"email": "iloveunswayed@example.com",
	"passwd": "encryptedpassword",
	"created_at": "2020-06-04T18:09:08.899937"
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