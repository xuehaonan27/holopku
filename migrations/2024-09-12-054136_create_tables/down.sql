-- This file should undo anything in `up.sql`
-- 回滚触发器
DROP TRIGGER IF EXISTS trg_check_comments_ids ON "Posts";
-- 回滚触发器函数
DROP FUNCTION IF EXISTS check_comments_ids();

DROP TABLE IF EXISTS "Comments";

DROP TABLE IF EXISTS "Posts";

DROP TABLE IF EXISTS "Users";

DROP TYPE IF EXISTS "GoodsType";
DROP TYPE IF EXISTS "GameType";
DROP TYPE IF EXISTS "Place";
DROP TYPE IF EXISTS "PostType";
DROP TYPE IF EXISTS "LoginProvider";