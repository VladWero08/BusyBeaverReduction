CREATE DATABASE IF NOT EXISTS `busy-beaver`;
USE `busy-beaver`;

CREATE TABLE IF NOT EXISTS `turing_machines` (
    `id` int NOT NULL AUTO_INCREMENT,
    `transition_function` text NOT NULL,
    `number_of_states` tinyint NOT NULL,
    `number_of_symbols` tinyint NOT NULL,
    `halted` tinyint NOT NULL,
    `steps` bigint NOT NULL,
    `score` bigint NOT NULL,
    `time_to_run` int NOT NULL,
    
    PRIMARY KEY (`id`)
);