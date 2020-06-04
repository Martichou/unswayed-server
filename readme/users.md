# Users

Used to register a user in the database.

**URL** : `/users`

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
	"id": 2,
	"email": "iloveunswayed@example.com",
	"passwd": "encrypt(abcd1234)",
	"created_at": "2020-06-04T18:09:08.899937"
}
```

## Error Response

**Condition** : If 'email' is already used

**Code** : `409 CONFLICT`

**Content** :

```json
{
  "error": "The requested item is already present"
}
```