ALTER TABLE posts ADD COLUMN summary VARCHAR DEFAULT 'none' NOT NULL;
ALTER TABLE posts ADD COLUMN comment_url VARCHAR DEFAULT 'https://www.reddit.com/r/gatewaynode/' NOT NULL;
