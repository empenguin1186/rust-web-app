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

INSERT INTO Comments (comment_id, author, comment) VALUES (1, 1, "hoge");
INSERT INTO Comments (comment_id, author, comment) VALUES (2, 2, "fuga");
INSERT INTO Comments (comment_id, author, comment) VALUES (3, 3, "piyo");
INSERT INTO Comments (comment_id, author, comment) VALUES (4, 4, "hogehoge");
INSERT INTO Comments (comment_id, author, comment) VALUES (5, 5, "fugafuga");
INSERT INTO Comments (comment_id, author, comment) VALUES (6, 6, "piyopiyo");
INSERT INTO Comments (comment_id, author, comment) VALUES (7, 7, "hogehogehoge");
INSERT INTO TreePaths (ancestor, descendant) VALUES (1, 1);
INSERT INTO TreePaths (ancestor, descendant) VALUES (1, 2);
INSERT INTO TreePaths (ancestor, descendant) VALUES (1, 3);
INSERT INTO TreePaths (ancestor, descendant) VALUES (1, 4);
INSERT INTO TreePaths (ancestor, descendant) VALUES (1, 5);
INSERT INTO TreePaths (ancestor, descendant) VALUES (1, 6);
INSERT INTO TreePaths (ancestor, descendant) VALUES (1, 7);
INSERT INTO TreePaths (ancestor, descendant) VALUES (2, 2);
INSERT INTO TreePaths (ancestor, descendant) VALUES (2, 3);
INSERT INTO TreePaths (ancestor, descendant) VALUES (3, 3);
INSERT INTO TreePaths (ancestor, descendant) VALUES (4, 4);
INSERT INTO TreePaths (ancestor, descendant) VALUES (4, 5);
INSERT INTO TreePaths (ancestor, descendant) VALUES (4, 6);
INSERT INTO TreePaths (ancestor, descendant) VALUES (4, 7);
INSERT INTO TreePaths (ancestor, descendant) VALUES (5, 5);
INSERT INTO TreePaths (ancestor, descendant) VALUES (6, 6);
INSERT INTO TreePaths (ancestor, descendant) VALUES (6, 7);
INSERT INTO TreePaths (ancestor, descendant) VALUES (7, 7);

CREATE TABLE CommentsPE (
 comment_id SERIAL PRIMARY KEY,
 path VARCHAR(1000),
 author BIGINT UNSIGNED NOT NULL,
 comment TEXT NOT NULL
);

INSERT INTO CommentsPE (path, author, comment) VALUES ('1/', 1, "hoge");
INSERT INTO CommentsPE (author, comment) VALUES (2, 'fuga');
UPDATE CommentsPE
 SET path =
 (SELECT x.path FROM (
   SELECT path FROM CommentsPE WHERE comment_id = 1
 ) AS x) || LAST_INSERT_ID() || '/'
WHERE comment_id = LAST_INSERT_ID();

INSERT INTO CommentsPE (author, comment) VALUES (3, 'piyo');
UPDATE CommentsPE
 SET path =
 (SELECT x.path FROM (
   SELECT path FROM CommentsPE WHERE comment_id = 2
 ) AS x) || LAST_INSERT_ID() || '/'
WHERE comment_id = LAST_INSERT_ID();
