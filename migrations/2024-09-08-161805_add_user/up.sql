-- Your SQL goes here
CREATE TABLE "users"(
	"id" SERIAL PRIMARY KEY,
	"username" TEXT NOT NULL,
	"password" TEXT NOT NULL,
	"mail" TEXT NOT NULL,
	"api_token" TEXT
);

