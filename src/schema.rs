table! {
    comments (id) {
        id -> Int4,
        content -> Text,
        user_id -> Nullable<Int4>,
        project_id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    migrations (id) {
        id -> Int4,
        migration -> Varchar,
        batch -> Int4,
    }
}

table! {
    profile_cover_images (id) {
        id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        user_id -> Int4,
        w3200_keyname -> Varchar,
        w3200_object_url -> Varchar,
        w300_keyname -> Varchar,
        w300_object_url -> Varchar,
    }
}

table! {
    profile_user_images (id) {
        id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        user_id -> Int4,
        w1500_keyname -> Varchar,
        w200_keyname -> Varchar,
        w40_keyname -> Varchar,
        w1500_object_url -> Varchar,
        w200_object_url -> Varchar,
        w40_object_url -> Varchar,
        deleted_at -> Nullable<Timestamp>,
    }
}

table! {
    project_categories (id) {
        id -> Int4,
        name -> Varchar,
        picture_url -> Varchar,
        slug -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
        color_hex -> Varchar,
        parent_category_id -> Nullable<Int4>,
    }
}

table! {
    project_image_categories (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        color_hex -> Varchar,
    }
}

table! {
    project_images (id) {
        id -> Int4,
        w1500_keyname -> Varchar,
        w350_keyname -> Varchar,
        project_image_category_id -> Int4,
        w1500_object_url -> Varchar,
        w350_object_url -> Varchar,
        primary -> Bool,
        project_id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        original_object_url -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
        views_count -> Int4,
    }
}

table! {
    projects (id) {
        id -> Int4,
        category_id -> Int4,
        user_id -> Int4,
        title -> Varchar,
        slug -> Varchar,
        content -> Text,
        published -> Bool,
        views_count -> Int4,
        likes_count -> Int4,
        deleted_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        sketchfab_model_number -> Nullable<Varchar>,
        is_pro -> Bool,
        bitbucket_project_key -> Nullable<Varchar>,
        tags -> Varchar,
    }
}

table! {
    subscriptions (id) {
        id -> Int4,
        subscriber_id -> Int4,
        user_id -> Int4,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        slug -> Varchar,
        email -> Varchar,
        password -> Varchar,
        punchline -> Nullable<Varchar>,
        website_url -> Nullable<Text>,
        admin -> Bool,
        active -> Bool,
        isNewProjectEmailSubscriber -> Bool,
        deleted_at -> Nullable<Timestamp>,
        remember_token -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        api_token -> Nullable<Varchar>,
    }
}

joinable!(comments -> projects (project_id));
joinable!(profile_user_images -> users (user_id));
joinable!(project_images -> project_image_categories (project_image_category_id));
joinable!(project_images -> projects (project_id));
joinable!(projects -> project_categories (category_id));

allow_tables_to_appear_in_same_query!(
    comments,
    migrations,
    profile_cover_images,
    profile_user_images,
    project_categories,
    project_image_categories,
    project_images,
    projects,
    subscriptions,
    users,
);
