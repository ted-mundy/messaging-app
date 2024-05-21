-- Your SQL goes here
CREATE TABLE `messages`(
	`uuid` UUID NOT NULL PRIMARY KEY,
	`content` TEXT NOT NULL,
	`created_at` TIMESTAMP NOT NULL,
	`updated_at` TIMESTAMP NOT NULL,
	`conversation_id` BIGINT NOT NULL UNSIGNED,
	`user_id` BIGINT NOT NULL UNSIGNED,
	`read_at` TIMESTAMP
);

