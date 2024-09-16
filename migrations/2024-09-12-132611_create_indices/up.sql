-- Your SQL goes here
-- Create index for user_id to speed up search
CREATE INDEX "idx_post_user_id" ON "Posts"(user_id);

-- Search comments according to user id
CREATE INDEX "idx_comment_user_id" ON "Comments"(user_id);

-- Create index for post_id to speed up search
CREATE INDEX "idx_comment_post_id" ON "Comments"(post_id);
