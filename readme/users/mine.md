# Mine

Used to get alls images informations uploaded by the users.

**URL** : `/api/mine`

**Method** : `GET`

**Auth required** : YES

**Header type** : Bearer access_token

## Success Response

**Code** : `200 OK`

**Response example**

```json
[
  	{
	    "id": 1,
	    "user_id": 1,
	    "realname": "osmxxh.jpg",
	    "fakedname": "wn4VY-T9KDobz9EfxXyQi22xy2KPZoEbYtFb0xYwg56KKSebn8C4xzCV9Y7iAgOMVGtCD6fThUDBlYzu-isURmzm_suY8ePEnUovkIqqfyZpi3bWhrb3rwD33_80WyRX",
	    "created_at": "2020-06-03T19:38:12.054073"
  	},
  	{
		"id": 2,
		"user_id": 1,
		"realname": "anxxte-8.jpg",
		"fakedname": "P3cx_OQ1FWOfMBcb0Yf7uC-U1vePAnKFKGAQe-Dxuxp2AsiE-WXTkQeMxxrmRa_kxqWSZrHR6vMPaMcncKGYW9X9urUkFzwGqUIAipW7yS8QkUAcI7naZUjaesZzxMG8",
		"created_at": "2020-06-04T01:14:11.905649"
	},
	{
		"id": 3,
		"user_id": 1,
		"realname": "maxsh.jpg",
		"fakedname": "bPZY3HodEMpdrXu7KjYKSbl-HzIC8NxwSqe0g-vm6RifL8IffSZcE0qkMOsVoo6mjVsJ5JVlViPFRFVqnQU_iJvlraoZt3xD0zkuE3dug811lMdk_I8U2EsB3ejjYbM5",
		"created_at": "2020-06-04T11:59:38.786822"
	},
	{
		"id": 4,
		"user_id": 1,
		"realname": "kyxash.jpg",
		"fakedname": "k-Vw0BsKXXw1oM-mLGdtXSAna6TfMAymMgSwF9-p5iLsbtESswavvTSYBrJ6GvpdRzwmN9KmFkJw5stGoEf3kMkU1yWEnGwHHkL2iy2YdpUBXxu2WaRH-T5AjEJuEzB1",
		"created_at": "2020-06-04T12:01:13.646986"
	}
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