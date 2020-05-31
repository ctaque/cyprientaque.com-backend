table! {
    project_categories (id) {
        id -> Int4,
        name -> Varchar,
        picture_url -> Nullable<Varchar>,
        slug -> Varchar,
    }
}

table! {
    project_images (id) {
        id -> Int4,
        w1500_keyname -> Varchar,
        w350_keyname -> Varchar,
        w1500_object_url -> Varchar,
        w350_object_url -> Varchar,
        primary -> Bool,
        project_id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
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
        sketchfab_model_number -> Nullable<Varchar>,
    }
}

joinable!(project_images -> projects (project_id));
joinable!(projects -> project_categories (category_id));

allow_tables_to_appear_in_same_query!(
    project_categories,
    project_images,
    projects,
);
