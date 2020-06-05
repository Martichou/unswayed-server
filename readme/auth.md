# Auth

Used to collect a Token for a registered User.

**URL** : `/auth`

**Method** : `POST`

**Auth required** : NO

**Data constraints**

```json
{
    "email": "[valid email address]",
    "passwd": "[password in plain text (encrypted)]"
}
```

**Data example**

```json
{
    "email": "iloveunswayed@example.com",
    "passwd": "encrypt(abcd1234)"
}
```

## Success Response

**Code** : `201 CREATED`

**Response example**

```json
{
	"id": 5,
	"user_id": 1,
	"token_type": 1,
	"access_token": "25y4wHPlq0MCUxvoW5U36O3ZoFe73n9F_dTJ9Ei_b80el2G0NNosN7R7S3z2sR9f",
	"refresh_token": "aSxGF8aX25EifSaGuwFO2xeqJiPEyJx55oqxa-7EznQ-HjVASlADwaYd7VMd4s7i",
	"created_at": "2020-06-04T11:49:51.926297",
	"expire_at": "2020-06-04T13:49:51.926297"
}
```

## Error Response

**Condition** : If 'email' or 'passwd' are incorrect

**Code** : `401 UNAUTHORIZED`

**Content** :

```json
{
  "error": "Your email was entered incorrectly, or your password was incorrect"
}
```