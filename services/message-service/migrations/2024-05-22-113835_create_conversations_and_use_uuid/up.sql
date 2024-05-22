-- Your SQL goes here
ALTER TABLE "messages" DROP COLUMN "conversation_id";
ALTER TABLE "messages" DROP COLUMN "user_id";
ALTER TABLE "messages" ADD COLUMN "user_uuid" UUID NOT NULL;
ALTER TABLE "messages" ADD COLUMN "conversation_uuid" UUID NOT NULL;

ALTER TABLE "messages" ALTER COLUMN "uuid" SET DEFAULT gen_random_uuid();
ALTER TABLE "messages" ALTER COLUMN "created_at" SET DEFAULT now();
ALTER TABLE "messages" ALTER COLUMN "updated_at" SET DEFAULT now();

CREATE TABLE "conversations"(
	"uuid" UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
	"created_at" TIMESTAMP NOT NULL DEFAULT now(),
	"updated_at" TIMESTAMP NOT NULL DEFAULT now(),
	"receiving_user_uuid" UUID NOT NULL,
	"sending_user_uuid" UUID NOT NULL
);

ALTER TABLE "messages" ADD CONSTRAINT "fk_messages_conversation_uuid" FOREIGN KEY ("conversation_uuid") REFERENCES "conversations"("uuid") ON DELETE CASCADE;
