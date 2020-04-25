table! {
    posts (id) {
        id -> Int4,
        published -> Bool,
        title -> Varchar,
        body -> Text,
        time -> Timestamptz,
    }
}
