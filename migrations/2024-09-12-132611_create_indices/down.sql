-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS "idx_comment_post_id";

DROP INDEX IF EXISTS "idx_comment_user_id";

DROP INDEX IF EXISTS "idx_post_user_id";