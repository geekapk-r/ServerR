-- Your SQL goes here
CREATE TABLE _security(
    uid SmallInt PRIMARY KEY,
    metapass VarChar NOT Null,
    passhash VarChar
)