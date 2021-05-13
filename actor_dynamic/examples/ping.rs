use actor_dynamic::*;

struct Ping(u32);

impl Message for Ping {
    type Result = u32;
}

struct PingActor {
    count: u32,
}

impl Actor for PingActor {
    type Context = Context<Self>;
}

impl Handler<Ping> for PingActor {
    type Result = u32;

    fn handle(&mut self, msg: Ping) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}

#[tokio::main]
async fn main() {
    let addr = PingActor { count: 10 }.start();

    let res = addr.send(Ping(10)).await;

    println!("RESULT: {}", res.unwrap() == 20);
}
