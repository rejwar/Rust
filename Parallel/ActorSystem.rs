use actix::prelude::*;

struct Worker;

impl Actor for Worker {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "String")]
struct ComputeTask;

impl Handler<ComputeTask> for Worker {
    fn handle(&mut self, _: ComputeTask, _: &mut Context<Self>) -> String {
        "Task Completed!".to_string()
    }
}

#[actix::main]
async fn main() {
    let Worker1 = Worker.start();
    let Result = Worker1.send(ComputeTask).await.unwrap();
    println!("Actor System Response: {}", Result);
}
