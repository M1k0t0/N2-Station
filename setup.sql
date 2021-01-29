CREATE TABLE IF NOT EXISTS `users`(
            `uuid` CHAR(32) NOT NULL,
            `username` VARCHAR(16) NOT NULL,
            `email` VARCHAR(30) NOT NULL,
            `passwd` BINARY(60) NOT NULL,
            `reg_date` TIMESTAMP NOT NULL
        );
CREATE TABLE IF NOT EXISTS `rooms`(
            `owner_uuid` CHAR(32) NOT NULL,
            `stream_id` VARCHAR(16) NOT NULL,
            `title` VARCHAR(16) NOT NULL,
            `desc` VARCHAR(20) NOT NULL,
            `tag` VARCHAR(1024) NULL,
            `open` BOOL NOT NULL,
            `stream_token` CHAR(32) NULL
        );