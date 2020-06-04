# Upload

Used to get the user_information.

**URL** : `/api/upload`

**Method** : `POST`

**Auth required** : YES

**Header type** : Bearer access_token

**Data constraints** (Multipart)

```json
{
    "images": "the raw data of the image 1",
	"images": "the raw data of the image 2",
	...
}
```

## Success Response

**Code** : `200 OK`

**Response example**

```json
[
	{
		"realname": "kxxsh.jpg",
		"filename": "k-Vw0BsKXXw1oM-mLGdtXSAna6TfMAymMgSwF9-p5iLsbtESswavvTSYBrJ6GvpdRzwmN9KmFkJw5stGoEf3kMkU1yWEnGwHHkL2iy2YdpUBXxu2WaRH-T5AjEJuEzB1",
		"url": "https://s3.server.com/k-Vw0BsKXXw1oM-mLGdtXSAna6TfMAymMgSwF9-p5iLsbtESswavvTSYBrJ6GvpdRzwmN9KmFkJw5stGoEf3kMkU1yWEnGwHHkL2iy2YdpUBXxu2WaRH-T5AjEJuEzB1"
  	},
	...
]
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