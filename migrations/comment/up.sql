CREATE TABLE Comments (
 comment_id SERIAL PRIMARY KEY,
 author BIGINT UNSIGNED NOT NULL,
 comment TEXT NOT NULL
);

CREATE TABLE TreePaths (
 ancestor BIGINT UNSIGNED NOT NULL,
 descendant BIGINT UNSIGNED NOT NULL,
 PRIMARY KEY (ancestor, descendant),
 FOREIGN KEY (ancestor) REFERENCES Comments(comment_id),
 FOREIGN KEY (descendant) REFERENCES Comments(comment_id)
);
