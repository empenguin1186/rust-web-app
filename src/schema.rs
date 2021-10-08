table! {
    Comments (comment_id) {
        comment_id -> Unsigned<Bigint>,
        author -> Unsigned<Bigint>,
        comment -> Text,
    }
}

table! {
    CommentsPE (comment_id) {
        comment_id -> Unsigned<Bigint>,
        path -> Nullable<Varchar>,
        author -> Unsigned<Bigint>,
        comment -> Text,
    }
}

table! {
    TreePaths (ancestor, descendant) {
        ancestor -> Unsigned<Bigint>,
        descendant -> Unsigned<Bigint>,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    Comments,
    CommentsPE,
    TreePaths,
    posts,
);
