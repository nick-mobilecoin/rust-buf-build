// Taken from the example at https://github.com/tikv/grpc-rs/tree/master/tests-and-examples/examples/hello_world
// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.
use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures_channel::oneshot;
use futures_executor::block_on;
use futures_util::future::{FutureExt as _, TryFutureExt as _};
use grpcio::{
    ChannelBuilder, Environment, ResourceQuota, RpcContext, ServerBuilder, ServerCredentials,
    UnarySink,
};

// use grpcio_proto::example::helloworld::{HelloReply, HelloRequest};
// use grpcio_proto::example::helloworld_grpc::{create_greeter, Greeter};
use helloworld::{HelloReply, HelloRequest};
use helloworld_grpc::{create_greeter, Greeter};

pub mod helloworld {
    include!(concat!(env!("OUT_DIR"), concat!("/helloworld.rs")));
}
pub mod helloworld_grpc {
    include!(concat!(env!("OUT_DIR"), concat!("/helloworld_grpc.rs")));
}

#[derive(Clone)]
struct GreeterService;

impl Greeter for GreeterService {
    fn say_hello(&mut self, ctx: RpcContext<'_>, req: HelloRequest, sink: UnarySink<HelloReply>) {
        let msg = format!("Hello {}", req.get_name());
        let mut resp = HelloReply::default();
        resp.set_message(msg);
        let f = sink
            .success(resp)
            .map_err(move |e| format!("failed to reply {:?}: {:?}", req, e))
            .map(|_| ());
        ctx.spawn(f)
    }
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let service = create_greeter(GreeterService);
    let addr = "127.0.0.1:50051";

    let quota = ResourceQuota::new(Some("HelloServerQuota")).resize_memory(1024 * 1024);
    let ch_builder = ChannelBuilder::new(env.clone()).set_resource_quota(quota);

    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .channel_args(ch_builder.build_args())
        .build()
        .unwrap();
    server
        .add_listening_port(addr, ServerCredentials::insecure())
        .unwrap();
    server.start();
    println!("listening on {addr}");
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = block_on(rx);
    let _ = block_on(server.shutdown());
}