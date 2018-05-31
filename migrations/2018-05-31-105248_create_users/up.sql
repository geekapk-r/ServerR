-- Your SQL goes here
CREATE TABLE users(
    id SmallSerial PRIMARY KEY,
    simple_name VarChar NOT Null,
    avatar_url VarChar,
    user_name VarChar NOT Null,
    alias VarChar,
    github VarChar,
    bio VarChar NOT Null DEFAULT '',
    dev_bio VarChar,
    created_at TimeStamp NOT Null DEFAULT now(),
    online_at TimeStamp NOT Null DEFAULT now(),
    followers_num SmallInt NOT Null DEFAULT 0,
    enabled Boolean NOT Null DEFAULT true
)