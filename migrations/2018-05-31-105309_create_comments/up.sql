-- Your SQL goes here
CREATE TABLE comments(
    id Serial PRIMARY KEY,
    author_user SmallInt NOT Null,
    app SmallInt NOT Null,
    reply_comment Integer,
    content VarChar NOT Null,
    stars_num SmallInt NOT Null DEFAULT 0,
    replies_num Integer NOT Null DEFAULT 0,
    created_at TimeStamp NOT Null DEFAULT now(),
    updated_at TimeStamp
)