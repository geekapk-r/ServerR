-- Your SQL goes here
CREATE TABLE categories(
    id SmallSerial PRIMARY KEY,
    category_name VarChar NOT Null,
    parent_category SmallInt
)