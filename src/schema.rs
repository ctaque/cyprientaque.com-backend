table! {
    project_categories (id) {
        id -> Int4,
        name -> Varchar,
        picture_url -> Nullable<Varchar>,
        slug -> Varchar,
    }
}

table! {
    projects (id) {
        id -> Int4,
        category_id -> Int4,
        title -> Varchar,
        slug -> Varchar,
        content -> Text,
        views_count -> Int4,
        likes_count -> Int4,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

joinable!(projects -> project_categories (category_id));

allow_tables_to_appear_in_same_query!(
    project_categories,
    projects,
);
