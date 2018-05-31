table! {
    apps (id) {
        id -> Int2,
        author_user -> Int2,
        category -> Int2,
        package_name -> Nullable<Varchar>,
        app_name -> Varchar,
        alias -> Nullable<Varchar>,
        icon_url -> Nullable<Varchar>,
        app_description -> Varchar,
        visualizer -> Nullable<Varchar>,
        button_text -> Nullable<Varchar>,
        special -> Nullable<Varchar>,
        previews -> Nullable<Varchar>,
        app_permissions -> Nullable<Varchar>,
        size -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        stars_num -> Int2,
        comments_num -> Int4,
    }
}

table! {
    categories (id) {
        id -> Int2,
        category_name -> Varchar,
        parent_category -> Nullable<Int2>,
    }
}

table! {
    comments (id) {
        id -> Int4,
        author_user -> Int2,
        app -> Int2,
        reply_comment -> Nullable<Int4>,
        content -> Varchar,
        stars_num -> Int2,
        replies_num -> Int4,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    comment_stars (igored) {
        igored -> Int4,
        uid -> Int2,
        comment -> Int4,
    }
}

table! {
    follow (ignored) {
        ignored -> Int4,
        uid -> Int2,
        followed_user -> Int2,
    }
}

table! {
    notifications (igored) {
        igored -> Int4,
        uid -> Int2,
        created_at -> Timestamp,
        notification_type -> Int2,
        notification_data -> Int4,
        enabled -> Bool,
    }
}

table! {
    _security (uid) {
        uid -> Int2,
        metapass -> Varchar,
        passhash -> Nullable<Varchar>,
    }
}

table! {
    stars (ignored) {
        ignored -> Int4,
        uid -> Int2,
        app -> Int2,
    }
}

table! {
    timelines (igored) {
        igored -> Int4,
        uid -> Int2,
        created_at -> Timestamp,
        line_type -> Int2,
        line_data -> Int4,
    }
}

table! {
    updates (ignored) {
        ignored -> Int4,
        app -> Int2,
        version_name -> Varchar,
        reversion -> Int2,
        install_url -> Varchar,
        updates -> Varchar,
        api_min -> Nullable<Int2>,
        api_target -> Nullable<Int2>,
    }
}

table! {
    users (id) {
        id -> Int2,
        simple_name -> Varchar,
        avatar_url -> Nullable<Varchar>,
        user_name -> Varchar,
        alias -> Nullable<Varchar>,
        github -> Nullable<Varchar>,
        bio -> Varchar,
        dev_bio -> Nullable<Varchar>,
        created_at -> Timestamp,
        online_at -> Timestamp,
        followers_num -> Int2,
        enabled -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    apps,
    categories,
    comments,
    comment_stars,
    follow,
    notifications,
    _security,
    stars,
    timelines,
    updates,
    users,
);
