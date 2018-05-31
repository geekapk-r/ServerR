#[derive(Debug)]
pub enum WebHookListenType {
    ReplyToMessage,
    CommentApp,
    NewUser,
}

impl WebHookListenType {
    pub fn from_str(s: String) -> WebHookListenType {
        if s == "replyToMessage" {
            WebHookListenType::ReplyToMessage
        } else if s == "commentApp" {
            WebHookListenType::CommentApp
        } else {
            WebHookListenType::NewUser
        }
    }
}

#[derive(Debug)]
pub struct WebHook {
    pub hook_type: WebHookListenType,
    pub url: String,
    pub data: i32,
}
