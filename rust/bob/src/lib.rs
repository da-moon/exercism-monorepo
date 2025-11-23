enum Message {
    Question,
    Yell,
    YellQuestion,
    Empty,
    Other,
}
pub fn reply(message: &str) -> &str {
    let mut MessageType: Message = Message::Empty;
    let trimmed_message: &str = message.trim();
    if trimmed_message.len() != 0 {
        if trimmed_message.find(char::is_lowercase) == None {
            let question: Option<usize> = trimmed_message.find('?');
            if question == None {
                if (trimmed_message.find(|c: char| ((c < '9') && (c > '0')) || c == ',') != None)
                    && (trimmed_message.find(|c: char| (c < 'Z') && (c > 'A')) != None)
                {
                    MessageType = Message::Yell;
                } else if trimmed_message.find(|c: char| ((c < '9') && (c > '0')) || c == ',')
                    != None
                {
                    MessageType = Message::Other;
                } else {
                    MessageType = Message::Yell;
                }
            } else {
                if trimmed_message.len() - 1 == question.unwrap() {
                    if trimmed_message.find(|c: char| (c < 'z') && (c > 'A')) == None {
                        MessageType = Message::Question;
                    } else {
                        MessageType = Message::YellQuestion;
                    }
                } else {
                    MessageType = Message::Yell;
                }
            }
        } else {
            let question: Option<usize> = trimmed_message.find('?');
            if question == None {
                MessageType = Message::Other;
            } else {
                if trimmed_message.len() - 1 == question.unwrap() {
                    MessageType = Message::Question;
                } else {
                    MessageType = Message::Other;
                }
            }
        }
    } else {
        MessageType = Message::Empty;
    }
    let result = match MessageType {
        Message::Question => "Sure.",
        Message::Yell => "Whoa, chill out!",
        Message::YellQuestion => "Calm down, I know what I'm doing!",
        Message::Empty => "Fine. Be that way!",
        Message::Other => "Whatever.",
    };
    result
}
