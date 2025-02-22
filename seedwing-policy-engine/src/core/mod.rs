use crate::lang::hir::Type;
use crate::lang::lir::Bindings;
use crate::value::{RationaleResult, Value};
use async_mutex::Mutex;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::sync::Arc;

pub mod base64;
pub mod json;
pub mod list;
pub mod sigstore;
pub mod x509;

#[derive(Debug)]
pub enum FunctionError {
    InvalidInput,
    Other(String),
}

pub trait Function: Sync + Send + Debug {
    fn documentation(&self) -> Option<String> {
        None
    }

    fn parameters(&self) -> Vec<String> {
        Default::default()
    }

    fn call<'v>(
        &'v self,
        input: Arc<Mutex<Value>>,
        bindings: &'v Bindings,
    ) -> Pin<Box<dyn Future<Output = Result<RationaleResult, FunctionError>> + 'v>>;
}
