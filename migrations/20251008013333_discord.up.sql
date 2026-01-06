-- Add up migration script here
CREATE SCHEMA IF NOT EXISTS discord;


-- Create discord users table
CREATE TABLE IF NOT EXISTS discord.users (
    user_id NUMERIC(20) PRIMARY KEY,
    is_restricted BOOLEAN NOT NULL DEFAULT FALSE,
    puffs INT NOT NULL DEFAULT 0
);


-- Create discord server / guilds table
CREATE TABLE IF NOT EXISTS discord.guilds (
    guild_id NUMERIC(20) PRIMARY KEY,
    is_restricted BOOLEAN NOT NULL DEFAULT FALSE,
    puffs INT NOT NULL DEFAULT 0
);