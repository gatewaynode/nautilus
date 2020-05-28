-- Add node references to existing content tables
ALTER TABLE posts ADD COLUMN parent VARCHAR (255) NOT NULL DEFAULT '{"content_type": "None", "id": 0}';
ALTER TABLE links ADD COLUMN parent VARCHAR (255) NOT NULL DEFAULT '{"content_type": "None", "id": 0}';

-- Update all tables with version field for revisions
ALTER TABLE posts ADD COLUMN version INTEGER NOT NULL DEFAULT 1;
ALTER TABLE links ADD COLUMN version INTEGER NOT NULL DEFAULT 1;
ALTER TABLE system ADD COLUMN version INTEGER NOT NULL DEFAULT 1;

-- Add the updated timestamp field to content and system tables
ALTER TABLE posts ADD COLUMN updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now();
ALTER TABLE links ADD COLUMN updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now();
ALTER TABLE system ADD COLUMN updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now();

-- Remove the unused "published" and comment_url column from content tables
ALTER TABLE posts DROP COLUMN published;
ALTER TABLE posts DROP COLUMN comment_url;
ALTER TABLE links DROP COLUMN published;

-- Create the primary node table
CREATE TABLE nodes (
  id SERIAL PRIMARY KEY,
  version INTEGER NOT NULL DEFAULT 1,
  child VARCHAR (255) NOT NULL DEFAULT '{"content_type": "None", "id": 0}',
  _child_hash VARCHAR (140) NOT NULL DEFAULT '',
  _self_hash VARCHAR (140) NOT NULL DEFAULT '',
  _hash_chain VARCHAR (140) NOT NULL DEFAULT '',
  labels TEXT NOT NULL DEFAULT '',
  workflow VARCHAR (255) NOT NULL DEFAULT 'DRAFT',
  permissions TEXT NOT NULL DEFAULT '',
  paths_to TEXT NOT NULL DEFAULT '',
  paths_from TEXT NOT NULL DEFAULT '',
  node_next TEXT NOT NULL DEFAULT '',
  node_last TEXT NOT NULL DEFAULT '',
  time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

-- Create the revision tables
CREATE TABLE node_revisions (
  id INTEGER NOT NULL DEFAULT 0,
  version INTEGER NOT NULL DEFAULT 1,
  child VARCHAR (255) NOT NULL DEFAULT '{"content_type": "None", "id": 0}',
  _child_hash VARCHAR (140) NOT NULL DEFAULT '',
  _self_hash VARCHAR (140) NOT NULL DEFAULT '',
  _hash_chain VARCHAR (140) NOT NULL DEFAULT '',
  labels TEXT NOT NULL DEFAULT '',
  workflow VARCHAR (255) NOT NULL DEFAULT 'DRAFT',
  permissions TEXT NOT NULL DEFAULT '',
  paths_to TEXT NOT NULL DEFAULT '',
  paths_from TEXT NOT NULL DEFAULT '',
  node_next TEXT NOT NULL DEFAULT '',
  node_last TEXT NOT NULL DEFAULT '',
  time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  PRIMARY KEY(id, version)
);

CREATE TABLE post_revisions (
  id INTEGER NOT NULL DEFAULT 0,
  version INTEGER NOT NULL DEFAULT 1,
  parent VARCHAR (255) NOT NULL DEFAULT '{"content_type": "None", "id": 0}',
  title VARCHAR NOT NULL DEFAULT '',
  body TEXT NOT NULL DEFAULT '',
  summary VARCHAR NOT NULL DEFAULT '',
  tags VARCHAR NOT NULL DEFAULT '',
  time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  PRIMARY KEY (id, version)
);

CREATE TABLE link_revisions (
  id INTEGER NOT NULL DEFAULT 0,
  version INTEGER NOT NULL DEFAULT 1,
  parent VARCHAR (255) NOT NULL DEFAULT '{"content_type": "None", "id": 0}',
  text VARCHAR NOT NULL,
  title VARCHAR NOT NULL,
  url VARCHAR NOT NULL,
  tags VARCHAR NOT NULL,
  time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  PRIMARY KEY (id, version)
);

CREATE TABLE system_revisions (
  key VARCHAR NOT NULL DEFAULT 'INVALID',
  version INTEGER NOT NULL DEFAULT 1,
  data VARCHAR NOT NULL,
  time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
  PRIMARY KEY (key, version)
);
