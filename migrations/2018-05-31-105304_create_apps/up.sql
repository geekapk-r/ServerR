-- Your SQL goes here
CREATE TABLE apps(
    id SmallSerial PRIMARY KEY,
    author_user SmallInt NOT Null,
    category SmallInt NOT Null,
    package_name VarChar,
    app_name VarChar NOT Null,
    alias VarChar,
    icon_url VarChar,
    app_description VarChar NOT Null DEFAULT '',
    visualizer VarChar,
    button_text VarChar,
    special VarChar,
    previews VarChar,
    app_permissions VarChar,
    size Integer NOT Null DEFAULT 0,
    created_at TimeStamp NOT Null DEFAULT now(),
    updated_at TimeStamp NOT Null DEFAULT now(),
    stars_num SmallInt NOT Null DEFAULT 0,
    comments_num Integer NOT Null DEFAULT 0
)