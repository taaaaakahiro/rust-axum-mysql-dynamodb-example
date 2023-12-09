CREATE TABLE IF NOT EXISTS `users`(
    `id` VARCHAR(20) NOT NULL,
    `name` VARCHAR(50) NOT NULL,
    `created_at` TIMESTAMP DEFAULT NOW() NOT NULL,
    PRIMARY KEY (`id`)
);

-- CREATE TABLE IF NOT EXISTS `todos`(
--     `id` VARCHAR(20) NOT NULL,
--     `user_id` VARCHAR(20) NOT NULL,
--     `name` VARCHAR(50) NOT NULL,
--     `created_at` TIMESTAMP DEFAULT NOW() NOT NULL,
--     PRIMARY KEY (`id`),
--     CONSTRAINT `fk_todos_user1`
--     FOREIGN KEY (`user_id`)
--     REFERENCES `users` (`id`)
-- );