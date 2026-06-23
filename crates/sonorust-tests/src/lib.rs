use std::collections::HashMap;

use sonorust_cranelift::CraneliftJitExecutor;
use sonorust_runtime::SonorustIRExecutor;

pub fn get_available_executors() -> HashMap<&'static str, Box<dyn SonorustIRExecutor>> {
    let mut executors: HashMap<&'static str, Box<dyn SonorustIRExecutor>> = HashMap::new();
    executors.insert("cranelift-jit", Box::new(CraneliftJitExecutor::default()));
    executors
}
