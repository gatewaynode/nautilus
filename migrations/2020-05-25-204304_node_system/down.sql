-- Remove node table
DROP TABLE nodes;

-- Undo alter statements
ALTER TABLE posts DROP COLUMN parent;
ALTER TABLE links DROP COLUMN parent;
ALTER TABLE posts DROP COLUMN updated;
ALTER TABLE links DROP COLUMN updated;
ALTER TABLE system DROP COLUMN updated;
ALTER TABLE posts DROP COLUMN version;
ALTER TABLE links DROP COLUMN version;
ALTER TABLE system DROP COLUMN version;

-- Restore publish fields
ALTER TABLE posts ADD COLUMN published BOOLEAN NOT NULL DEFAULT 'f';
ALTER TABLE links ADD COLUMN published BOOLEAN NOT NULL DEFAULT 'f';

-- Drop revision tables
DROP TABLE node_revisions;
DROP TABLE post_revisions;
DROP TABLE link_revisions;
DROP TABLE system_revisions;
