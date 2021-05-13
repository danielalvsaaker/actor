use actor_static::*;

struct Payload(usize);

impl Message for Payload {
    type Result = usize;
}

struct Node {
    id: usize,
    next: Addr<Payload>,
    chan: tokio::sync::mpsc::Sender<()>,
}

impl Actor<Payload> for Node {
    type Context = Context<Self, Payload>;
}

const NODES: usize = 3000;
const ROUNDS: usize = 4000;
const LIMIT: usize = NODES * ROUNDS;

fn handler(act: &mut Node, msg: Payload) -> usize {
    if msg.0 >= LIMIT {
        println!(
            "Actor {} reached limit of {} (payload was {})",
            act.id, LIMIT, msg.0
        );
        let _ = act.chan.try_send(());
        return msg.0;
    }

    if msg.0 % 498989 == 1 {
        println!(
            "Actor {} received message {} of {} ({:.2}%)",
            act.id,
            msg.0,
            LIMIT,
            100.0 * msg.0 as f32 / LIMIT as f32,
        );
    }

    act.next.do_send(Payload(msg.0 + 1));

    msg.0 + 1
}

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(14);

    let now = tokio::time::Instant::now();

    let node = Node::create(
        move |ctx: &mut Context<Node, Payload>| {
            let first_addr = ctx.address();

            let mut prev_addr = Node {
                id: 1,
                next: first_addr,
                chan: tx.clone(),
            }
            .start(handler);

            for id in 2..NODES {
                prev_addr = Node {
                    id,
                    next: prev_addr,
                    chan: tx.clone(),
                }
                .start(handler);
            }

            Node {
                id: NODES,
                next: prev_addr,
                chan: tx,
            }
        },
        handler,
    );

    println!(
        "Sending start message and waiting for termination after {} messages...",
        LIMIT
    );

    node.send(Payload(1)).await.unwrap();
    rx.recv().await;

    let elapsed = now.elapsed();
    println!(
        "Time taken: {}.{:06} seconds ({} msg/second)",
        elapsed.as_secs(),
        elapsed.subsec_micros(),
        (NODES * ROUNDS * 1000000) as u128 / elapsed.as_micros()
    );
}
