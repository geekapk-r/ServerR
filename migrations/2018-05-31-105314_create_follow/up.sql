-- Your SQL goes here
CREATE TABLE follow(
    ignored Serial PRIMARY KEY,
    uid SmallInt NOT Null,
    followed_user SmallInt NOT Null
)