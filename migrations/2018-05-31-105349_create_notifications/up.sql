-- Your SQL goes here
CREATE TABLE notifications(
    igored Serial PRIMARY KEY,
    uid SmallInt NOT Null,
    created_at TimeStamp NOT Null DEFAULT now(),
    notification_type SmallInt NOT Null,
    notification_data Integer NOT Null,
    enabled Boolean NOT Null DEFAULT false
)