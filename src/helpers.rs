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

impl WebHook {
    pub fn parse(s: String) -> Vec<WebHook> {
        let mut tmp = Vec::<WebHook>::new();
        for i in s.split(";") {
            if i == "" {
                continue;
            }
            let mut splited_entry = i.split(":");
            tmp.push(WebHook {
                hook_type: WebHookListenType::from_str(splited_entry.nth(0).unwrap().to_string()),
                url: splited_entry.nth(2).unwrap().to_string(),
                data: splited_entry
                    .nth(1)
                    .unwrap()
                    .to_string()
                    .parse::<i32>()
                    .unwrap(),
            });
        }
        return tmp;
    }
}
