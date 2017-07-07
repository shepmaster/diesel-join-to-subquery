CREATE TABLE users (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL
);

CREATE TABLE posts (
  id INTEGER PRIMARY KEY NOT NULL,
  name TEXT NOT NULL
);

CREATE TABLE votes (
  user_id INT NOT NULL,
  post_id INT NOT NULL,
  count INT NOT NULL,
  PRIMARY KEY (user_id, post_id)
);

CREATE VIEW total_votes AS SELECT
  votes.user_id AS user_id,
  SUM(votes.count) AS total
FROM votes
GROUP BY votes.user_id;
