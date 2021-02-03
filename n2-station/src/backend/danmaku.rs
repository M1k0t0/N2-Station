use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};

use actix::prelude::*;
use actix_web_actors::ws;
use rand::{prelude::ThreadRng, Rng};

//highly inspired by https://github.com/actix/examples/blob/master/websocket-chat/

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const TIMEOUT_DURATION: Duration = Duration::from_secs(10);

#[derive(Message)]
#[rtype(result = "()")]
enum Message {
    Message { name: String, msg: String },
    KickOut { reason: String },
}

#[derive(Message)]
#[rtype(usize)]
struct Connect {
    room: String,
    name: String,
    addr: Recipient<Message>,
}

#[derive(Message)]
#[rtype(result = "()")]
struct Disconnect {
    id: usize,
}

#[derive(Message)]
#[rtype(result = "()")]
struct ClientMessage {
    id: usize,
    msg: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct OpenRoom {
    pub(crate) room: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CloseRoom {
    pub(crate) room: String,
}

pub struct ChatServer {
    peers: HashSet<String>,
    sessions: HashMap<usize, (String, String, Recipient<Message>)>,
    rooms: HashMap<String, HashSet<usize>>,
    rng: ThreadRng,
}

impl ChatServer {
    pub fn new() -> Self {
        Self {
            peers: HashSet::new(),
            sessions: HashMap::new(),
            rooms: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }

    fn send_message(&self, session: usize, message: &str) {
        if let Some((room, name, recp)) = self.sessions.get(&session) {
            let _ = recp.do_send(Message::Message {
                name: name.clone(),
                msg: message.to_owned(),
            });
            if let Some(receivers) = self.rooms.get(room) {
                for receiver in receivers {
                    if *receiver != session {
                        if let Some((_, _, addr)) = self.sessions.get(receiver) {
                            let _ = addr.do_send(Message::Message {
                                name: name.clone(),
                                msg: message.to_owned(),
                            });
                        }
                    }
                }
            }
        }
    }

    fn broadcast_message(&self, room: &str, message: &str) {
        if let Some(receivers) = self.rooms.get(room) {
            for receiver in receivers {
                if let Some((_, _, addr)) = self.sessions.get(receiver) {
                    let _ = addr.do_send(Message::Message {
                        name: "0".to_owned(),
                        msg: message.to_owned(),
                    });
                }
            }
        }
    }
}

impl Actor for ChatServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ChatServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        if self.peers.contains(&msg.name) {
            return 0;
        }
        if self.rooms.contains_key(&msg.room) {
            let id = self.rng.gen_range(1..std::usize::MAX);
            self.broadcast_message(&msg.room, &format!("{} joined the room!", msg.name));
            self.rooms
                .entry(msg.room.clone())
                .or_insert_with(HashSet::new)
                .insert(id);
            self.sessions.insert(id, (msg.room, msg.name, msg.addr));
            id
        } else {
            0
        }
    }
}

impl Handler<Disconnect> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Self::Context) -> Self::Result {
        if let Some((room, name, _)) = self.sessions.get(&msg.id) {
            self.peers.remove(name);
            if let Some(sessions) = self.rooms.get_mut(room) {
                sessions.remove(&msg.id);
                self.broadcast_message(room, &format!("{} left the room!", name));
            }
        }
    }
}

impl Handler<ClientMessage> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, _: &mut Self::Context) -> Self::Result {
        self.send_message(msg.id, &msg.msg);
    }
}

impl Handler<OpenRoom> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: OpenRoom, _: &mut Self::Context) -> Self::Result {
        let _ = self.rooms.entry(msg.room).or_insert_with(HashSet::new);
    }
}

impl Handler<CloseRoom> for ChatServer {
    type Result = ();

    fn handle(&mut self, msg: CloseRoom, _: &mut Self::Context) -> Self::Result {
        if let Some(sessions) = self.rooms.get(&msg.room) {
            for session in sessions {
                if let Some((_, _, addr)) = self.sessions.get(session) {
                    let _ = addr.do_send(Message::KickOut {
                        reason: "Room closed".to_owned(),
                    });
                }
            }
        }
        self.rooms.remove(&msg.room);
    }
}

pub struct DanmakuSession {
    id: usize,
    heartbeat: Instant,
    room: String,
    name: String,
    addr: Addr<ChatServer>,
}

impl Actor for DanmakuSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.schedule_heartbeat(ctx);
        let addr = ctx.address();
        self.addr
            .send(Connect {
                room: self.room.clone(),
                name: self.name.clone(),
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => {
                        if res != 0 {
                            act.id = res;
                        } else {
                            ctx.text("chat 0;Something went wrong...");
                            ctx.stop();
                        }
                    }
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(Disconnect { id: self.id });
        Running::Stop
    }
}

impl Handler<Message> for DanmakuSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Message::Message { name, msg } => ctx.text(format!("chat {};{}", name, msg)),
            Message::KickOut { reason } => {
                ctx.text(format!("chat 0;You are kicked out due to: {}", reason));
                ctx.stop()
            }
        }
    }
}

impl DanmakuSession {
    pub fn new(room: &str, name: &str, svr: Addr<ChatServer>) -> Self {
        Self {
            id: 0,
            heartbeat: Instant::now(),
            room: room.to_owned(),
            name: name.to_owned(),
            addr: svr,
        }
    }

    fn schedule_heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.heartbeat) > TIMEOUT_DURATION {
                act.addr.do_send(Disconnect { id: act.id });
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for DanmakuSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Text(text) => {
                let message = text.trim();
                if message.starts_with("message ") {
                    let body: Vec<&str> = message.splitn(2, ' ').collect();
                    let body = if body.len() > 1 { body[1] } else { "" };
                    self.addr.do_send(ClientMessage {
                        id: self.id,
                        msg: body.to_owned(),
                    });
                }
            }
            ws::Message::Binary(_) => (),
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Ping(msg) => {
                self.heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.heartbeat = Instant::now();
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}
