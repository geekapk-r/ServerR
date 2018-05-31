-- Your SQL goes here
CREATE TABLE timelines(
    igored Serial PRIMARY KEY,
    uid SmallInt NOT Null,
    created_at TimeStamp NOT Null DEFAULT now(),
    line_type SmallInt NOT Null,
    line_data Integer NOT Null
)