#[derive(Debug, Serialize, Deserialize)]
pub enum WebHookListenType {
    CommentApp,
    ReplyToMessage,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct WebHook {
    pub hook_type: WebHookListenType,
    pub data: u32,
    pub url: String,
}

impl WebHook {
    pub fn parse(s: String) -> Vec<WebHook> {
        let mut tmp = Vec::<WebHook>::new();
        for i in s.split(";") {
            if *(::VERBOSE) {
                println!("Parsing item: {}", i);
            }
            if i == "" {
                continue;
            }
            let mut splited_entry = i.split(":");
            let a = splited_entry.next();
            let b = splited_entry.next();
            let c = splited_entry.next();
            if *(::VERBOSE) {
                println!("Construct item: {:?} : {:?} : {:?}", a, b, c);
            }
            tmp.push(WebHook {
                hook_type: WebHookListenType::from_str(a.unwrap().to_string()),
                data: b.unwrap().to_string().parse::<u32>().unwrap(),
                url: c.unwrap()
                    .to_string()
                    .replace("HTTPS", "https:")
                    .replace("HTTP", "http:"),
            });
        }
        return tmp;
    }
}
