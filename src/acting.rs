use std::collections::HashMap;
use std::collections::VecDeque;

use rand::random;

pub struct MessageQueue {
    inner: VecDeque<Message>,
}

// TODO: Use the delegate crate
impl MessageQueue {
    pub fn new() -> MessageQueue {
        MessageQueue {
            inner: VecDeque::new(),
        }
    }

    pub fn push_msg(&mut self, msg: Message) {
        self.inner.push_back(msg);
    }

    pub fn pop_msg(&mut self) -> Option<Message> {
        self.inner.pop_back()
    }

    pub fn make_ctx<'a>(&'a mut self) -> Context<'a> {
        Context {
            msg_queue: self,
        }
    }
}

pub struct Worker {
    actors: HashMap<ActorAddr, Box<dyn Actor>>,

    pub msg_queue: MessageQueue,
}

impl Worker {
    pub fn new() -> Worker {
        Worker {
            actors: HashMap::new(),
            msg_queue: MessageQueue::new(),
        }
    }

    pub fn add_actor(&mut self, actor: Box<dyn Actor>) -> ActorAddr {
        let addr = ActorAddr::random();
        self.actors.insert(addr, actor);
        addr
    }

    pub fn step_once(&mut self) -> bool {
        match self.msg_queue.pop_msg() {
            Some(Message { to, cont }) => {
                eprintln!("Sending {:?} to {:?}", cont, to);
                if let Some(act) = self.actors.get_mut(&to) {
                    let ctx = self.msg_queue.make_ctx();
                    act.handle_message(cont, ctx);
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct ActorAddr {
    pub addr: u32,
}

impl ActorAddr {
    fn random() -> ActorAddr {
        ActorAddr { addr: random() }
    }
}
#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Message {
    pub to: ActorAddr,
    pub cont: MessageContent,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct MessageContent {
    pub atom: Atom,
    pub data: Vec<Argument>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub enum Argument {
    ActorAddr(ActorAddr),
    Number(i32),
    String(String),
    Atom(Atom),
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub struct Atom(pub u32);

// TODO: Properly abstract this to handle multiple workers
pub struct Context<'a> {
    msg_queue: &'a mut MessageQueue,
}

impl<'a> Context<'a> {
    pub fn push_msg(&mut self, msg: Message) {
        self.msg_queue.push_msg(msg);
    }
}

pub trait Actor {
    fn handle_message<'a>(&'a mut self, message: MessageContent, ctx: Context<'a>);
}
