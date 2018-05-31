use schema::*;
use std::time::SystemTime;

#[derive(Queryable)]
pub struct User {
    id: i16,
    simple_name: String,
    avatar_url: Option<String>,
    user_name: String,
    alias: Option<String>,
    github: Option<String>,
    bio: String,
    dev_bio: Option<String>,
    created_at: SystemTime,
    online_at: SystemTime,
    followers_num: i16,
    enabled: bool,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    simple_name: String,
    avatar_url: Option<String>,
    user_name: String,
    alias: Option<String>,
    github: Option<String>,
    bio: String,
    dev_bio: Option<String>,
}

#[derive(Queryable)]
pub struct Category {
    id: i16,
    category_name: String,
    parent_category: Option<i16>,
}

#[derive(Insertable)]
#[table_name = "categories"]
pub struct NewCategory {
    category_name: String,
    parent_category: Option<i16>,
}

#[derive(Queryable)]
pub struct App {
    id: i16,
    author_user: i16,
    category: i16,
    package_name: Option<String>,
    app_name: String,
    alias: Option<String>,
    icon_url: Option<String>,
    app_description: String,
    visualizer: Option<String>,
    button_text: Option<String>,
    special: Option<String>,
    previews: Option<String>,
    app_permissions: Option<String>,
    size: i32,
    created_at: SystemTime,
    updated_at: SystemTime,
    stars_num: i16,
    comments_num: i32,
}

#[derive(Insertable)]
#[table_name = "apps"]
pub struct NewApp {
    author_user: i16,
    category: i16,
    package_name: Option<String>,
    app_name: String,
    alias: Option<String>,
    icon_url: Option<String>,
    app_description: String,
    visualizer: Option<String>,
    button_text: Option<String>,
    special: Option<String>,
    previews: Option<String>,
    app_permissions: Option<String>,
    size: i32,
}

#[derive(Queryable)]
pub struct Comment {
    id: i32,
    author_user: i16,
    app: i16,
    reply_comment: Option<i32>,
    content: String,
    stars_num: i16,
    replies_num: i32,
    created_at: SystemTime,
    updated_at: Option<SystemTime>,
}

#[derive(Insertable)]
#[table_name = "comments"]
pub struct NewComment {
    author_user: i16,
    app: i16,
    reply_comment: Option<i32>,
    content: String,
}

#[derive(Queryable)]
pub struct Follow {
    uid: i16,
    followed_user: i16,
}

#[derive(Insertable)]
#[table_name = "follow"]
pub struct NewFollow {
    uid: i16,
    followed_user: i16,
}

#[derive(Queryable)]
pub struct Security {
    uid: i16,
    metapass: String,
    passhash: Option<String>,
}

#[derive(Insertable)]
#[table_name = "_security"]
pub struct NewSecurity {
    uid: i16,
    metapass: String,
    passhash: Option<String>,
}

#[derive(Queryable)]
pub struct Update {
    app: i16,
    version: String,
    reversion: i16,
    install_url: String,
    updates_text: String,
    api_min: Option<i16>,
    api_target: Option<i16>,
}

#[derive(Insertable)]
#[table_name = "updates"]
pub struct NewUpdate {
    app: i16,
    version_name: String,
    reversion: i16,
    install_url: String,
    updates_text: String,
    api_min: Option<i16>,
    api_target: Option<i16>,
}

#[derive(Queryable)]
pub struct Star {
    uid: i16,
    app: i16,
}

#[derive(Insertable)]
#[table_name = "stars"]
pub struct NewStar {
    uid: i16,
    app: i16,
}

#[derive(Queryable)]
pub struct CommentStar {
    uid: i16,
    comment: i32,
}

#[derive(Insertable)]
#[table_name = "comment_stars"]
pub struct NewCommentStar {
    uid: i16,
    comment: i32,
}

#[derive(Queryable)]
pub struct Timeline {
    uid: i16,
    created_at: SystemTime,
    line_type: i16,
    line_data: i32,
}

#[derive(Insertable)]
#[table_name = "timelines"]
pub struct NewTimeline {
    uid: i16,
    line_type: i16,
    line_data: i32,
}

#[derive(Queryable)]
pub struct Notification {
    uid: i16,
    created_at: SystemTime,
    notification_type: i16,
    notification_data: i32,
    enabled: bool,
}

#[derive(Insertable)]
#[table_name = "notifications"]
pub struct NewNotification {
    uid: i16,
    notification_type: i16,
    notification_data: i32,
}
