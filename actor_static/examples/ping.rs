use actor_static::*;

// Declare a message.
struct Ping(u32);

// Implement Message for our message.
impl Message for Ping {
    type Result = u32;
}

// Declare an actor.
struct PingActor {
    count: u32,
}

// Implement Actor for our actor.
impl Actor<Ping> for PingActor {
    type Context = Context<Self, Ping>;
}

// Declare a handler function for handling incoming messages.
fn handle(act: &mut PingActor, msg: Ping) -> u32 {
    act.count += msg.0;
    act.count
}

#[tokio::main]
async fn main() {
    // Start the actor with the given handler function.
    let addr = PingActor { count: 10 }.start(handle);

    // Send a message and wait for a response.
    let res = addr.send(Ping(10)).await;

    println!("RESULT: {}", res.unwrap() == 20);
}
