use actix::prelude::*;
use actix_web_actors::ws;

#[derive(Message)]
#[rtype(result="()")]
pub struct ReloadMessage;

pub struct ReloadActor;
impl Actor for ReloadActor {
    type Context = ws::WebsocketContext<Self>;
    
    fn started(&mut self, ctx: &mut Self::Context) {
        let mut addrs = ADDRS.lock().unwrap();
        addrs.push(ctx.address());
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        let mut addrs = ADDRS.lock().unwrap();
        let idx = addrs.iter().position(|a| *a == ctx.address());
        if let Some(i) = idx {
            addrs.remove(i);
        }
    }
}
impl Handler<ReloadMessage> for ReloadActor {
    type Result = ();

    fn handle(&mut self, _msg: ReloadMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text("reload");
    }
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ReloadActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}


use actix_web::{ web, get, Error, HttpResponse, HttpRequest };

use crate::state::ADDRS;

#[get("/reload")]
async fn reload(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(ReloadActor, &req, stream)
}
