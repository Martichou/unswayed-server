# Get/{filename}

Used to get an image previously uploaded.

**URL** : `/api/get/{filename}`

**Method** : `GET`

**Auth required** : YES

**Header type** : Bearer access_token

## Success Response

**Code** : `200 OK`

**Response example**

```raw
the raw data of the filename
```

## Error Response

**Condition** : If the filename doesn't belong to the user

**Code** : `400 BAD REQUEST`

**Content** :

```json
{
  "error": "Make sure you're the owner of the file you're requesting"
}
```