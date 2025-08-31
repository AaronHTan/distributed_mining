use std::{collections::HashMap, fmt::Display, marker::Tuple};

use bincode::{error, Decode, Encode};
use serde::{de::DeserializeOwned, Serialize};

pub type Registry =
    HashMap<String, Box<dyn RpcFunction<dyn Tuple, dyn Fn<dyn Tuple, Output = ()>>>>;

pub trait RpcFunction<X, O>
where
    X: Tuple + Encode + Decode<String> + Serialize + DeserializeOwned + Display,
    O: Decode<String> + Encode + DeserializeOwned + Serialize + Display,
{
    fn new() -> Self;

    fn func(&self) -> fn(X) -> O;

    fn call_package(args: X) -> Result<Vec<u8>, error::EncodeError> {
        bincode::encode_to_vec(args, bincode::config::standard())
    }

    fn call(&self, args: X) -> O {
        self.func()(args)
    }

    fn name(&self) -> String;

    fn return_package(output: O) -> Result<Vec<u8>, bincode::error::EncodeError> {
        bincode::encode_to_vec(output, bincode::config::standard())
    }
}

fn add((x, y): (i32, i32)) -> i32 {
    x + y
}

struct AddRpc;

impl RpcFunction<(i32, i32), i32> for AddRpc {
    fn new() -> Self {
        AddRpc
    }

    fn func(self) -> fn((i32, i32)) -> i32 {
        add
    }
}
