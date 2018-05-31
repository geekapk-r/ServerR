-- Your SQL goes here
CREATE TABLE updates(
    ignored Serial PRIMARY KEY,
    app SmallInt NOT Null,
    version_name VarChar NOT Null,
    reversion SmallInt NOT Null,
    install_url VarChar NOT Null,
    updates VarChar NOT Null,
    api_min SmallInt,
    api_target SmallInt
)