-- Add migration script here
ALTER TABLE users ADD role TEXT DEFAULT "user";
