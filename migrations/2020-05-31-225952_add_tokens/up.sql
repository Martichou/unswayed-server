CREATE TABLE access_tokens (
	id SERIAL NOT NULL PRIMARY KEY,
	user_id INT NOT NULL,
	access_token TEXT NOT NULL,
	refresh_token TEXT NOT NULL,
	created_at TIMESTAMP NOT NULL
);