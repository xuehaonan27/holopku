-- Your SQL goes here
-- CreateEnum
CREATE TYPE "LoginProvider" AS ENUM ('IAAA', 'PASSWORD');

-- CreateTable
CREATE TABLE "Users" (
    id SERIAL NOT NULL PRIMARY KEY,
    username VARCHAR(255) NOT NULL, -- E.g. 2200088888
    email VARCHAR(100),
    login_provider "LoginProvider" NOT NULL,
    nickname VARCHAR NOT NULL,
    password VARCHAR,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
);

CREATE TABLE "Posts" (
    id SERIAL NOT NULL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    user_id INT NOT NULL,
    content TEXT NOT NULL,
    likes INT NOT NULL DEFAULT 0,
    favorates INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES "Users"(id) ON DELETE CASCADE
);

CREATE TABLE "Comments" (
    id SERIAL NOT NULL PRIMARY KEY,
    post_id INT NOT NULL,
    user_id INT NOT NULL,
    content TEXT NOT NULL,
    likes INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES "Users"(id) ON DELETE CASCADE,
    FOREIGN KEY (post_id) REFERENCES "Posts"(id) ON DELETE CASCADE
);
