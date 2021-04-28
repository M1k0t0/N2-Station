use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use actix::prelude::*;
use actix_web_actors::ws;
use rand::{prelude::ThreadRng, Rng};

use super::model::response::BakedRoom;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const TIMEOUT_DURATION: Duration = Duration::from_secs(10);

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct IncrementalData {
    pub(crate) data: BakedRoom,
}

#[derive(Message)]
#[rtype(usize)]
struct Connect {
    addr: Recipient<IncrementalData>,
}

#[derive(Message)]
#[rtype(result = "()")]
struct Disconnect {
    id: usize,
}

pub struct IncrementalServer {
    sessions: HashMap<usize, Recipient<IncrementalData>>,
    rng: ThreadRng,
}

impl IncrementalServer {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
            rng: rand::thread_rng(),
        }
    }

    fn broadcast_incremental(&self, incremental: IncrementalData) {
        for recp in self.sessions.values() {
            let _ = recp.do_send(incremental.clone());
        }
    }
}

impl Handler<Connect> for IncrementalServer {
    type Result = usize;

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        let id = self.rng.gen_range(1..std::usize::MAX);
        self.sessions.insert(id, msg.addr);
        id
    }
}

impl Handler<Disconnect> for IncrementalServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        self.sessions.remove(&msg.id);
    }
}

impl Handler<IncrementalData> for IncrementalServer {
    type Result = ();

    fn handle(&mut self, msg: IncrementalData, _ctx: &mut Self::Context) -> Self::Result {
        self.broadcast_incremental(msg);
    }
}

impl Actor for IncrementalServer {
    type Context = Context<Self>;
}

pub struct IncrementalSession {
    id: usize,
    heartbeat: Instant,
    addr: Addr<IncrementalServer>,
}

impl Actor for IncrementalSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.schedule_heartbeat(ctx);
        let addr = ctx.address();
        self.addr
            .send(Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
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

impl IncrementalSession {
    pub fn new(svr: Addr<IncrementalServer>) -> Self {
        Self {
            id: 0,
            heartbeat: Instant::now(),
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

impl Handler<IncrementalData> for IncrementalSession {
    type Result = ();

    fn handle(&mut self, msg: IncrementalData, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(&serde_json::to_string(&msg.data).unwrap());
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for IncrementalSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        match msg {
            ws::Message::Text(_) => (),
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
