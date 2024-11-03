-- Your SQL goes here
-- CreateEnum
CREATE TYPE "LoginProvider" AS ENUM ('IAAA', 'PASSWORD');
CREATE TYPE "PostType" AS ENUM ('FOODPOST', 'SELLPOST', 'AMUSEMENTPOST');
CREATE TYPE "Place" AS ENUM (
    'JiaYuan',
    'YiYuan',
    'ShaoYuan',
    'YanNan',
    'NongYuan',
    'XueYi',
    'XueWu',
    'Other'
);
CREATE TYPE "GameType" AS ENUM (
    'WolfKill',
    'JvBen',
    'BloodTower',
    'Karaok',
    'BoardGame',
    'Sports',
    'Riding',
    'Other'
);
CREATE TYPE "GoodsType" AS ENUM (
    'Ticket',
    'Book',
    'Display',
    'Computer',
    'Other'
);

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
    comments_id INT[] NOT NULL,
    images INT[] NOT NULL,
    post_type "PostType" NOT NULL,
    contact VARCHAR(255),
    -- FoodPost
    food_place "Place",
    score INT DEFAULT 0,
    -- AmusementPost
    people_all INT DEFAULT 0,
    people_already INT DEFAULT 0,
    game_type "GameType",
    start_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    amuse_place VARCHAR(255),
    -- SellPost
    price INT DEFAULT 0,
    goods_type "GoodsType",
    sold BOOLEAN DEFAULT false,
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

CREATE OR REPLACE FUNCTION check_comments_ids() RETURNS TRIGGER AS $$
BEGIN
    -- 检查 comments_id 数组中的每个值是否存在于 Comments 表中
    IF NEW.comments_id IS NOT NULL THEN
        FOR i IN 1..array_length(NEW.comments_id, 1) LOOP
            IF NOT EXISTS (SELECT 1 FROM "Comments" WHERE id = NEW.comments_id[i]) THEN
                RAISE EXCEPTION 'Invalid comment id: %', NEW.comments_id[i];
            END IF;
        END LOOP;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_check_comments_ids
BEFORE INSERT OR UPDATE ON "Posts"
FOR EACH ROW
EXECUTE FUNCTION check_comments_ids();
