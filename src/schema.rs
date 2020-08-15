table! {
    link_revisions (id, version) {
        id -> Int4,
        version -> Int4,
        text -> Varchar,
        title -> Varchar,
        url -> Varchar,
        tags -> Varchar,
        time -> Timestamptz,
        updated -> Timestamptz,
        parent -> Int4,
    }
}

table! {
    links (id) {
        id -> Int4,
        text -> Varchar,
        title -> Varchar,
        url -> Varchar,
        tags -> Varchar,
        time -> Timestamptz,
        version -> Int4,
        updated -> Timestamptz,
        parent -> Int4,
    }
}

table! {
    node_revisions (id, version) {
        id -> Int4,
        version -> Int4,
        _child_hash -> Varchar,
        _self_hash -> Varchar,
        _hash_chain -> Varchar,
        labels -> Text,
        workflow -> Varchar,
        permissions -> Text,
        paths_to -> Text,
        paths_from -> Text,
        node_next -> Text,
        node_last -> Text,
        time -> Timestamptz,
        updated -> Timestamptz,
        child -> Int4,
        child_content_type -> Varchar,
    }
}

table! {
    nodes (id) {
        id -> Int4,
        version -> Int4,
        _child_hash -> Varchar,
        _self_hash -> Varchar,
        _hash_chain -> Varchar,
        labels -> Text,
        workflow -> Varchar,
        permissions -> Text,
        paths_to -> Text,
        paths_from -> Text,
        node_next -> Text,
        node_last -> Text,
        time -> Timestamptz,
        updated -> Timestamptz,
        child -> Int4,
        child_content_type -> Varchar,
    }
}

table! {
    post_revisions (id, version) {
        id -> Int4,
        version -> Int4,
        title -> Varchar,
        body -> Text,
        summary -> Varchar,
        tags -> Varchar,
        time -> Timestamptz,
        updated -> Timestamptz,
        parent -> Int4,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        time -> Timestamptz,
        tags -> Varchar,
        summary -> Varchar,
        version -> Int4,
        updated -> Timestamptz,
        parent -> Int4,
    }
}

table! {
    system (key) {
        key -> Varchar,
        data -> Varchar,
        time -> Timestamptz,
        version -> Int4,
        updated -> Timestamptz,
    }
}

table! {
    system_revisions (key, version) {
        key -> Varchar,
        version -> Int4,
        data -> Varchar,
        time -> Timestamptz,
        updated -> Timestamptz,
    }
}

allow_tables_to_appear_in_same_query!(
    link_revisions,
    links,
    node_revisions,
    nodes,
    post_revisions,
    posts,
    system,
    system_revisions,
);
