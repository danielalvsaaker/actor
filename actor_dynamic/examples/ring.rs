use actor_dynamic::*;

struct Payload(usize);

impl Message for Payload {
    type Result = ();
}

struct Node {
    id: usize,
    limit: usize,
    next: Addr<Node>,
    chan: tokio::sync::mpsc::Sender<()>,
}

impl Actor for Node {
    type Context = Context<Self>;
}

impl Handler<Payload> for Node {
    type Result = ();

    fn handle(&mut self, msg: Payload) {
        if msg.0 >= self.limit {
            println!(
                "Actor {} reached limit of {} (payload was {})",
                self.id, self.limit, msg.0
            );
            let _ = self.chan.try_send(());
            return;
        }

        if msg.0 % 498989 == 1 {
            println!(
                "Actor {} received message {} of {} ({:.2}%)",
                self.id,
                msg.0,
                self.limit,
                100.0 * msg.0 as f32 / self.limit as f32,
            );
        }

        self.next.do_send(Payload(msg.0 + 1));
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let n_nodes = 3000;
    let n_rounds = 4000;
    let limit = n_nodes * n_rounds;
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);

    let now = tokio::time::Instant::now();

    let node = Node::create(move |ctx| {
        let first_addr = ctx.address();

        let mut prev_addr = Node {
            id: 1,
            limit,
            next: first_addr,
            chan: tx.clone(),
        }
        .start();

        for id in 2..n_nodes {
            prev_addr = Node {
                id,
                limit,
                next: prev_addr,
                chan: tx.clone(),
            }
            .start();
        }

        Node {
            id: n_nodes,
            limit,
            next: prev_addr,
            chan: tx,
        }
    });

    println!(
        "Sending start message and waiting for termination after {} messages...",
        limit
    );

    node.send(Payload(1)).await.unwrap();
    rx.recv().await;

    let elapsed = now.elapsed();
    println!(
        "Time taken: {}.{:06} seconds ({} msg/second)",
        elapsed.as_secs(),
        elapsed.subsec_micros(),
        (n_nodes * n_rounds * 1000000) as u128 / elapsed.as_micros()
    );
}
