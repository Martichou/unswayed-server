# Refresh

Used to get a new Token for a registered User from a refresh_token.

**URL** : `/refresh`

**Method** : `POST`

**Auth required** : NO

**Data constraints**

```json
{
    "refresh_token": "[valid refresh_token]"
}
```

**Data example**

```json
{
    "refresh_token": "aSxGF8aX25EifSaGuwFO2xeqJiPEyJx55oqxa-7EznQ-HjVASlADwaYd7VMd4s7i"
}
```

## Success Response

**Code** : `201 CREATED`

**Response example**

```json
{
	"id": 8,
	"user_id": 1,
	"access_token": "UMrkriQk89KdAZ4F9XlEtTjcal6WUMI0ovk182jNCPzboZpu7bS94NCqU2XwoFHF",
	"refresh_token": "FzhG-VfcKAt_BoljSPPCCzHZxG-adLDD60BKFhDUgVuC00yqK7cksmNNoJF976h4",
	"created_at": "2020-06-04T18:11:04.187270"
}
```

## Error Response

**Condition** : If 'refresh_token' is incorrect

**Code** : `401 UNAUTHORIZED`

**Content** :

```json
{
  "error": "The token is invalid or has been expired"
}
```