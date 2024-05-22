-- This file should undo anything in `up.sql`
ALTER TABLE "messages" DROP COLUMN "user_uuid";
ALTER TABLE "messages" DROP COLUMN "conversation_uuid";
ALTER TABLE "messages" ADD COLUMN "conversation_id" INT4 NOT NULL;
ALTER TABLE "messages" ADD COLUMN "user_id" INT4 NOT NULL;

DROP TABLE IF EXISTS "conversations";
