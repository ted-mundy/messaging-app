-- Your SQL goes here
CREATE TABLE "messages"(
	"uuid" UUID NOT NULL PRIMARY KEY,
	"content" TEXT NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL,
	"conversation_id" INT4 NOT NULL,
	"user_id" INT4 NOT NULL,
	"read_at" TIMESTAMP
);

