use std::collections::{VecDeque};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Message {
    pub kind: MessageKind,
    pub text: String,
    pub no_repeat: bool,
}

impl Message {
    pub fn new<S>(kind: MessageKind, text: S, no_repeat: bool) -> Self
    where S: Into<String>
    {
        Message {
            text: text.into(),
            kind,
            no_repeat
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MessageKind {
    Info,
    Inventory,
    Debug,
    Skill
}


impl From<String> for Message {
    fn from(text: String) -> Self {
        Message {
            text,
            kind: MessageKind::Info,
            no_repeat: false,
        }
    }
}

impl From<&str> for Message {
    fn from(text: &str) -> Self {
        Message {
            text: text.into(),
            kind: MessageKind::Info,
            no_repeat: false,
        }
    }
}

impl From<(MessageKind, String)> for Message {
    fn from(kind_text: (MessageKind, String)) -> Self {
        Message {
            kind: kind_text.0,
            text: kind_text.1,
            no_repeat: false,
        }
    }
}

impl From<(MessageKind, &str)> for Message {
    fn from(kind_text: (MessageKind, &str)) -> Self {
        Message {
            kind: kind_text.0,
            text: kind_text.1.into(),
            no_repeat: false,
        }
    }
}

#[derive(Debug, Default)]
pub struct MessageQueue {
    messages: VecDeque<Message>
}

impl MessageQueue {
    pub fn push<M>(&mut self, msg: M)
    where M: Into<Message> {
        let msg = msg.into();
        // do not add message if it is identical to the last one
        // and no_repeat is true
        if let Some(current) = self.messages.front() {
            if current.no_repeat == true && *current == msg {
                return;
            }
        }
        self.messages.push_front(msg);
    }

    pub fn flush(&mut self) {
        // TODO: this could be a little more elaborate
        self.messages.truncate(50);
    }

    pub fn iter(&self) -> impl Iterator<Item=&Message> {
        self.messages.iter()
    }
}
