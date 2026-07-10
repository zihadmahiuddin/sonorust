use std::cell::RefCell;

use sonorust_ir::nodes::*;
use sonorust_runtime::{access::MemoryAccess, testing::TestingRuntimeContext};
use sonorust_tests::get_available_executors;

#[test]
fn test_get() {
    let nodes = vec![
        IRNode::Value(0.0),
        IRNode::OpCode(OpCode::Get(Get {
            block_id: 0.into(),
            index: 0.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 7.0);
        let result = executor.execute(&nodes, 1.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 7.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_get_pointed() {
    let nodes = vec![
        IRNode::Value(0.0), // 0: block_id
        IRNode::Value(0.0), // 1: index
        IRNode::Value(2.0), // 2: offset
        IRNode::OpCode(OpCode::GetPointed(GetPointed {
            block_id: 0.into(),
            index: 1.into(),
            offset: 2.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(3, RefCell::new(vec![0.0; 4096]));
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 3.0);
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 1, 5.0);
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 3, 7, 123.45);

        let result = executor.execute(&nodes, 3.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 123.45,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_get_shifted() {
    let nodes = vec![
        IRNode::Value(0.0), // 0: block_id
        IRNode::Value(2.0), // 1: x
        IRNode::Value(3.0), // 2: y
        IRNode::Value(4.0), // 3: s
        IRNode::OpCode(OpCode::GetShifted(GetShifted {
            block_id: 0.into(),
            x: 1.into(),
            y: 2.into(),
            s: 3.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(0, RefCell::new(vec![0.0; 4096]));
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 2 + 3 * 4, 999.99); // index = 14

        let result = executor.execute(&nodes, 4.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 999.99,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set() {
    let nodes = vec![
        IRNode::Value(0.0),
        IRNode::Value(7.0),
        IRNode::OpCode(OpCode::Set(Set {
            block_id: 0.into(),
            index: 0.into(),
            value: 1.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 7.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_pointed() {
    let nodes = vec![
        IRNode::Value(0.0),    // 0: block_id
        IRNode::Value(0.0),    // 1: index
        IRNode::Value(2.0),    // 2: offset
        IRNode::Value(123.45), // 3: value
        IRNode::OpCode(OpCode::SetPointed(SetPointed {
            block_id: 0.into(),
            index: 1.into(),
            offset: 2.into(),
            value: 3.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(3, RefCell::new(vec![0.0; 4096]));
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 3.0);
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 1, 5.0);

        let result = executor.execute(&nodes, 4.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 123.45,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context.memory.read(&runtime_context.as_ctx(), 3, 7),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_shifted() {
    let nodes = vec![
        IRNode::Value(0.0),    // 0: block_id
        IRNode::Value(2.0),    // 1: x
        IRNode::Value(3.0),    // 2: y
        IRNode::Value(4.0),    // 3: s
        IRNode::Value(123.45), // 4: value
        IRNode::OpCode(OpCode::SetShifted(SetShifted {
            block_id: 0.into(),
            x: 1.into(),
            y: 2.into(),
            s: 3.into(),
            value: 4.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(0, RefCell::new(vec![0.0; 4096]));

        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);

        let index = 2 + 3 * 4; // 14
        assert_eq!(
            result, 123.45,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context
                .memory
                .read(&runtime_context.as_ctx(), 0, index),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_add() {
    let nodes = vec![
        IRNode::Value(0.0), // block_id
        IRNode::Value(5.0), // index
        IRNode::Value(3.0), // value
        IRNode::OpCode(OpCode::SetAdd(SetAdd {
            block_id: 0.into(),
            index: 1.into(),
            value: 2.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(0, RefCell::new(vec![0.0; 4096]));
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 5, 7.0);

        let result = executor.execute(&nodes, 3.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 10.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context.memory.read(&runtime_context.as_ctx(), 0, 5),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_add_pointed() {
    let nodes = vec![
        IRNode::Value(0.0), // block_id
        IRNode::Value(0.0), // index
        IRNode::Value(2.0), // offset
        IRNode::Value(3.0), // value
        IRNode::OpCode(OpCode::SetAddPointed(SetAddPointed {
            block_id: 0.into(),
            index: 1.into(),
            offset: 2.into(),
            value: 3.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();

        runtime_context
            .memory
            .writable
            .insert(3, RefCell::new(vec![0.0; 4096]));
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 3.0);
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 1, 5.0);
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 3, 7, 7.0);

        let result = executor.execute(&nodes, 4.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 10.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context.memory.read(&runtime_context.as_ctx(), 3, 7),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_add_shifted() {
    let nodes = vec![
        IRNode::Value(0.0), // block_id
        IRNode::Value(2.0), // x
        IRNode::Value(3.0), // y
        IRNode::Value(4.0), // s
        IRNode::Value(5.0), // value
        IRNode::OpCode(OpCode::SetAddShifted(SetAddShifted {
            block_id: 0.into(),
            x: 1.into(),
            y: 2.into(),
            s: 3.into(),
            value: 4.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(0, RefCell::new(vec![0.0; 4096]));
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 14, 10.0); // 2 + 3 * 4 = 14

        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 15.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context
                .memory
                .read(&runtime_context.as_ctx(), 0, 14),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_subtract() {
    let nodes = vec![
        IRNode::Value(0.0), // block_id
        IRNode::Value(5.0), // index
        IRNode::Value(3.0), // value (subtract)
        IRNode::OpCode(OpCode::SetSubtract(SetSubtract {
            block_id: 0.into(),
            index: 1.into(),
            value: 2.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(0, RefCell::new(vec![10.0; 4096]));

        let result = executor.execute(&nodes, 3.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 7.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context.memory.read(&runtime_context.as_ctx(), 0, 5),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_subtract_pointed() {
    let nodes = vec![
        IRNode::Value(0.0), // block_id
        IRNode::Value(0.0), // index
        IRNode::Value(2.0), // offset
        IRNode::Value(3.0), // value (subtract)
        IRNode::OpCode(OpCode::SetSubtractPointed(SetSubtractPointed {
            block_id: 0.into(),
            index: 1.into(),
            offset: 2.into(),
            value: 3.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(3, RefCell::new(vec![10.0; 4096]));
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 3.0);
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 1, 5.0);
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 3, 7, 10.0);

        let result = executor.execute(&nodes, 4.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 7.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context.memory.read(&runtime_context.as_ctx(), 3, 7),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_subtract_shifted() {
    let nodes = vec![
        IRNode::Value(0.0), // block_id
        IRNode::Value(2.0), // x
        IRNode::Value(3.0), // y
        IRNode::Value(4.0), // s
        IRNode::Value(3.0), // value (subtract)
        IRNode::OpCode(OpCode::SetSubtractShifted(SetSubtractShifted {
            block_id: 0.into(),
            x: 1.into(),
            y: 2.into(),
            s: 3.into(),
            value: 4.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(0, RefCell::new(vec![10.0; 4096]));
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 14, 10.0); // 2 + 3 * 4 = 14

        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 7.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context
                .memory
                .read(&runtime_context.as_ctx(), 0, 14),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_multiply() {
    let nodes = vec![
        IRNode::Value(0.0), // block_id
        IRNode::Value(5.0), // index
        IRNode::Value(2.0), // value (multiplier)
        IRNode::OpCode(OpCode::SetMultiply(SetMultiply {
            block_id: 0.into(),
            index: 1.into(),
            value: 2.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(0, RefCell::new(vec![10.0; 4096]));

        let result = executor.execute(&nodes, 3.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 20.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context.memory.read(&runtime_context.as_ctx(), 0, 5),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_multiply_pointed() {
    let nodes = vec![
        IRNode::Value(0.0), // block_id
        IRNode::Value(0.0), // index
        IRNode::Value(2.0), // offset
        IRNode::Value(2.0), // value (multiplier)
        IRNode::OpCode(OpCode::SetMultiplyPointed(SetMultiplyPointed {
            block_id: 0.into(),
            index: 1.into(),
            offset: 2.into(),
            value: 3.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(3, RefCell::new(vec![10.0; 4096]));
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 3.0);
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 1, 5.0);
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 3, 7, 10.0);

        let result = executor.execute(&nodes, 4.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 20.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context.memory.read(&runtime_context.as_ctx(), 3, 7),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_multiply_shifted() {
    let nodes = vec![
        IRNode::Value(0.0), // block_id
        IRNode::Value(2.0), // x
        IRNode::Value(3.0), // y
        IRNode::Value(4.0), // s
        IRNode::Value(2.0), // value (multiplier)
        IRNode::OpCode(OpCode::SetMultiplyShifted(SetMultiplyShifted {
            block_id: 0.into(),
            x: 1.into(),
            y: 2.into(),
            s: 3.into(),
            value: 4.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(0, RefCell::new(vec![10.0; 4096]));
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 14, 10.0); // 2 + 3 * 4 = 14

        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 20.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context
                .memory
                .read(&runtime_context.as_ctx(), 0, 14),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_divide() {
    let nodes = vec![
        IRNode::Value(0.0),  // block_id
        IRNode::Value(5.0),  // index
        IRNode::Value(10.0), // value (divisor)
        IRNode::OpCode(OpCode::SetDivide(SetDivide {
            block_id: 0.into(),
            index: 1.into(),
            value: 2.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(0, RefCell::new(vec![20.0; 4096]));

        let result = executor.execute(&nodes, 3.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 2.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context.memory.read(&runtime_context.as_ctx(), 0, 5),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_divide_pointed() {
    let nodes = vec![
        IRNode::Value(0.0), // block_id
        IRNode::Value(0.0), // index
        IRNode::Value(2.0), // offset
        IRNode::Value(2.0), // value (divisor)
        IRNode::OpCode(OpCode::SetDividePointed(SetDividePointed {
            block_id: 0.into(),
            index: 1.into(),
            offset: 2.into(),
            value: 3.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(3, RefCell::new(vec![10.0; 4096]));
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 3.0);
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 1, 5.0);
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 3, 7, 20.0);

        let result = executor.execute(&nodes, 4.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 10.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context.memory.read(&runtime_context.as_ctx(), 3, 7),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_divide_shifted() {
    let nodes = vec![
        IRNode::Value(0.0), // block_id
        IRNode::Value(2.0), // x
        IRNode::Value(3.0), // y
        IRNode::Value(4.0), // s
        IRNode::Value(2.0), // value (divisor)
        IRNode::OpCode(OpCode::SetDivideShifted(SetDivideShifted {
            block_id: 0.into(),
            x: 1.into(),
            y: 2.into(),
            s: 3.into(),
            value: 4.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(0, RefCell::new(vec![40.0; 4096]));
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 14, 40.0); // 2 + 3 * 4 = 14

        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 20.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context
                .memory
                .read(&runtime_context.as_ctx(), 0, 14),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_power() {
    let nodes = vec![
        IRNode::Value(0.0), // block_id
        IRNode::Value(5.0), // index
        IRNode::Value(3.0), // value (exponent)
        IRNode::OpCode(OpCode::SetPower(SetPower {
            block_id: 0.into(),
            index: 1.into(),
            value: 2.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(0, RefCell::new(vec![2.0; 4096]));

        let result = executor.execute(&nodes, 3.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 8.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context.memory.read(&runtime_context.as_ctx(), 0, 5),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_power_pointed() {
    let nodes = vec![
        IRNode::Value(0.0), // block_id
        IRNode::Value(0.0), // index
        IRNode::Value(2.0), // offset
        IRNode::Value(3.0), // value (exponent)
        IRNode::OpCode(OpCode::SetPowerPointed(SetPowerPointed {
            block_id: 0.into(),
            index: 1.into(),
            offset: 2.into(),
            value: 3.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(3, RefCell::new(vec![2.0; 4096]));
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 3.0);
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 1, 5.0);
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 3, 7, 2.0);

        let result = executor.execute(&nodes, 4.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 8.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context.memory.read(&runtime_context.as_ctx(), 3, 7),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_power_shifted() {
    let nodes = vec![
        IRNode::Value(0.0), // block_id
        IRNode::Value(2.0), // x
        IRNode::Value(3.0), // y
        IRNode::Value(4.0), // s
        IRNode::Value(3.0), // value (exponent)
        IRNode::OpCode(OpCode::SetPowerShifted(SetPowerShifted {
            block_id: 0.into(),
            x: 1.into(),
            y: 2.into(),
            s: 3.into(),
            value: 4.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(0, RefCell::new(vec![2.0; 4096]));
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 14, 2.0); // 2 + 3 * 4 = 14

        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 8.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context
                .memory
                .read(&runtime_context.as_ctx(), 0, 14),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_rem() {
    let nodes = vec![
        IRNode::Value(0.0), // block_id
        IRNode::Value(5.0), // index
        IRNode::Value(3.0), // value (divisor)
        IRNode::OpCode(OpCode::SetRem(SetRem {
            block_id: 0.into(),
            index: 1.into(),
            value: 2.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(0, RefCell::new(vec![10.0; 4096]));

        let result = executor.execute(&nodes, 3.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 1.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context.memory.read(&runtime_context.as_ctx(), 0, 5),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_rem_pointed() {
    let nodes = vec![
        IRNode::Value(0.0), // block_id
        IRNode::Value(0.0), // index
        IRNode::Value(2.0), // offset
        IRNode::Value(3.0), // value (divisor)
        IRNode::OpCode(OpCode::SetRemPointed(SetRemPointed {
            block_id: 0.into(),
            index: 1.into(),
            offset: 2.into(),
            value: 3.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(3, RefCell::new(vec![10.0; 4096]));
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 3.0);
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 1, 5.0);
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 3, 7, 10.0);

        let result = executor.execute(&nodes, 4.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 1.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context.memory.read(&runtime_context.as_ctx(), 3, 7),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_rem_shifted() {
    let nodes = vec![
        IRNode::Value(0.0), // block_id
        IRNode::Value(2.0), // x
        IRNode::Value(3.0), // y
        IRNode::Value(4.0), // s
        IRNode::Value(3.0), // value (divisor)
        IRNode::OpCode(OpCode::SetRemShifted(SetRemShifted {
            block_id: 0.into(),
            x: 1.into(),
            y: 2.into(),
            s: 3.into(),
            value: 4.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(0, RefCell::new(vec![10.0; 4096]));
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 14, 10.0); // 2 + 3 * 4 = 14

        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 1.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context
                .memory
                .read(&runtime_context.as_ctx(), 0, 14),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_mod() {
    let nodes = vec![
        IRNode::Value(0.0), // block_id
        IRNode::Value(5.0), // index
        IRNode::Value(3.0), // value (mod divisor)
        IRNode::OpCode(OpCode::SetMod(SetMod {
            block_id: 0.into(),
            index: 1.into(),
            value: 2.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(0, RefCell::new(vec![10.0; 4096]));

        let result = executor.execute(&nodes, 3.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 1.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context.memory.read(&runtime_context.as_ctx(), 0, 5),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_mod_pointed() {
    let nodes = vec![
        IRNode::Value(0.0), // block_id
        IRNode::Value(0.0), // index
        IRNode::Value(2.0), // offset
        IRNode::Value(3.0), // value (mod divisor)
        IRNode::OpCode(OpCode::SetModPointed(SetModPointed {
            block_id: 0.into(),
            index: 1.into(),
            offset: 2.into(),
            value: 3.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(3, RefCell::new(vec![10.0; 4096]));
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 3.0);
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 1, 5.0);
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 3, 7, 10.0);

        let result = executor.execute(&nodes, 4.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 1.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context.memory.read(&runtime_context.as_ctx(), 3, 7),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_set_mod_shifted() {
    let nodes = vec![
        IRNode::Value(0.0), // block_id
        IRNode::Value(2.0), // x
        IRNode::Value(3.0), // y
        IRNode::Value(4.0), // s
        IRNode::Value(3.0), // value (mod divisor)
        IRNode::OpCode(OpCode::SetModShifted(SetModShifted {
            block_id: 0.into(),
            x: 1.into(),
            y: 2.into(),
            s: 3.into(),
            value: 4.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(0, RefCell::new(vec![10.0; 4096]));
        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 14, 10.0); // 2 + 3 * 4 = 14

        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 1.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            Some(result),
            runtime_context
                .memory
                .read(&runtime_context.as_ctx(), 0, 14),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_copy_basic() {
    let nodes = vec![
        IRNode::Value(0.0), // src_block_id
        IRNode::Value(0.0), // src_index
        IRNode::Value(1.0), // dst_block_id
        IRNode::Value(1.0), // dst_index
        IRNode::Value(3.0), // count
        IRNode::OpCode(OpCode::Copy(Copy {
            src_block_id: 0.into(),
            src_index: 1.into(),
            dst_block_id: 2.into(),
            dst_index: 3.into(),
            count: 4.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(0, RefCell::new(vec![11.0, 22.0, 33.0, 0.0, 0.0]));
        runtime_context
            .memory
            .writable
            .insert(1, RefCell::new(vec![0.0; 5]));

        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            runtime_context.memory.read(&runtime_context.as_ctx(), 1, 1),
            Some(11.0),
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            runtime_context.memory.read(&runtime_context.as_ctx(), 1, 2),
            Some(22.0),
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            runtime_context.memory.read(&runtime_context.as_ctx(), 1, 3),
            Some(33.0),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_copy_overlap_forward() {
    let nodes = vec![
        IRNode::Value(0.0),
        IRNode::Value(0.0),
        IRNode::Value(0.0),
        IRNode::Value(1.0),
        IRNode::Value(3.0),
        IRNode::OpCode(OpCode::Copy(Copy {
            src_block_id: 0.into(),
            src_index: 1.into(),
            dst_block_id: 2.into(),
            dst_index: 3.into(),
            count: 4.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(0, RefCell::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]));

        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            runtime_context.memory.read(&runtime_context.as_ctx(), 0, 1),
            Some(1.0),
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            runtime_context.memory.read(&runtime_context.as_ctx(), 0, 2),
            Some(2.0),
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            runtime_context.memory.read(&runtime_context.as_ctx(), 0, 3),
            Some(3.0),
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            runtime_context.memory.read(&runtime_context.as_ctx(), 0, 4),
            Some(5.0),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_copy_overlap_backward() {
    let nodes = vec![
        IRNode::Value(0.0),
        IRNode::Value(2.0),
        IRNode::Value(0.0),
        IRNode::Value(0.0),
        IRNode::Value(3.0),
        IRNode::OpCode(OpCode::Copy(Copy {
            src_block_id: 0.into(),
            src_index: 1.into(),
            dst_block_id: 2.into(),
            dst_index: 3.into(),
            count: 4.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(0, RefCell::new(vec![10.0, 11.0, 12.0, 13.0, 14.0]));

        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            runtime_context.memory.read(&runtime_context.as_ctx(), 0, 0),
            Some(12.0),
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            runtime_context.memory.read(&runtime_context.as_ctx(), 0, 1),
            Some(13.0),
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            runtime_context.memory.read(&runtime_context.as_ctx(), 0, 2),
            Some(14.0),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}
#[test]
fn test_copy_zero_count() {
    let nodes = vec![
        IRNode::Value(0.0),
        IRNode::Value(0.0),
        IRNode::Value(1.0),
        IRNode::Value(0.0),
        IRNode::Value(0.0),
        IRNode::OpCode(OpCode::Copy(Copy {
            src_block_id: 0.into(),
            src_index: 1.into(),
            dst_block_id: 2.into(),
            dst_index: 3.into(),
            count: 4.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = TestingRuntimeContext::default();
        runtime_context
            .memory
            .writable
            .insert(0, RefCell::new(vec![5.0]));
        runtime_context
            .memory
            .writable
            .insert(1, RefCell::new(vec![9.0]));

        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            runtime_context.memory.read(&runtime_context.as_ctx(), 1, 0),
            Some(9.0),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}
