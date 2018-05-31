-- Your SQL goes here
CREATE TABLE comment_stars(
    igored Serial PRIMARY KEY,
    uid SmallInt NOT Null,
    comment Integer NOT Null
)