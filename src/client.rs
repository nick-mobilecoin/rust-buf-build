// Taken from the example at https://github.com/tikv/grpc-rs/tree/master/tests-and-examples/examples/hello_world
// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};
// use grpcio_proto::example::helloworld::HelloRequest;
// use grpcio_proto::example::helloworld_grpc::GreeterClient;
use helloworld::HelloRequest;
use helloworld_grpc::GreeterClient;
include!(concat!(env!("OUT_DIR"), concat!("/gen/mod.rs")));


fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:50051");
    let client = GreeterClient::new(ch);

    let mut req = HelloRequest::default();
    req.set_name("world".to_owned());
    let reply = client.say_hello(&req).expect("rpc");
    println!("Greeter received: {}", reply.get_message());
}