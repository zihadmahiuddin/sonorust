use sonorust_ir::nodes::*;
use sonorust_runtime::{context::MemoryAccess, testing::TestingRuntimeContext};
use sonorust_tests::get_available_executors;

#[test]
fn test_execute() {
    let nodes = vec![
        IRNode::Value(1.0),
        IRNode::Value(2.0),
        IRNode::OpCode(OpCode::Add(Add { inputs: vec![0, 1] })), // = 3.0
        IRNode::Value(100.0),
        IRNode::OpCode(OpCode::Execute(Execute { nodes: vec![2, 3] })), // should return 100.0
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 4, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 100.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_execute_0() {
    let nodes = vec![
        IRNode::Value(1.0),
        IRNode::Value(2.0),
        IRNode::OpCode(OpCode::Add(Add { inputs: vec![0, 1] })), // = 3.0
        IRNode::Value(100.0),
        IRNode::OpCode(OpCode::Execute0(Execute0 { nodes: vec![2, 3] })), // should return 100.0
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 4, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_execute_chained() {
    let nodes = vec![
        IRNode::Value(1.0),                                                // 0
        IRNode::Value(2.0),                                                // 1
        IRNode::OpCode(OpCode::Add(Add { inputs: vec![0, 1] })),           // 2 = 3.0
        IRNode::Value(5.0),                                                // 3
        IRNode::OpCode(OpCode::Multiply(Multiply { inputs: vec![2, 3] })), // 4 = 15.0
        IRNode::OpCode(OpCode::Execute(Execute { nodes: vec![2, 4] })),    // returns 15.0
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 5, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 15.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_if_true() {
    let nodes = vec![
        IRNode::Value(1.0),  // test (true)
        IRNode::Value(42.0), // consequent
        IRNode::Value(99.0), // alternate
        IRNode::OpCode(OpCode::If(If {
            test: 0,
            consequent: 1,
            alternate: 2,
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 42.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_if_false() {
    let nodes = vec![
        IRNode::Value(0.0),  // test (false)
        IRNode::Value(42.0), // consequent
        IRNode::Value(99.0), // alternate
        IRNode::OpCode(OpCode::If(If {
            test: 0,
            consequent: 1,
            alternate: 2,
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 99.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_if_with_expression_consequent() {
    let nodes = vec![
        IRNode::Value(1.0), // test = true
        IRNode::Value(2.0),
        IRNode::Value(3.0),
        IRNode::OpCode(OpCode::Add(Add { inputs: vec![1, 2] })), // 3 = 5.0
        IRNode::Value(100.0),                                    // alternate
        IRNode::OpCode(OpCode::If(If {
            test: 0,
            consequent: 3,
            alternate: 4,
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 5, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 5.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_if_with_expression_alternate() {
    let nodes = vec![
        IRNode::Value(0.0), // test = false
        IRNode::Value(2.0),
        IRNode::Value(3.0),
        IRNode::OpCode(OpCode::Multiply(Multiply { inputs: vec![1, 2] })), // 3 = 6.0
        IRNode::Value(42.0),                                               // consequent
        IRNode::OpCode(OpCode::If(If {
            test: 0,
            consequent: 4,
            alternate: 3,
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 5, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 6.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_nested_if() {
    let nodes = vec![
        IRNode::Value(1.0),  // test 0 (true)
        IRNode::Value(0.0),  // test 1 (false)
        IRNode::Value(11.0), // alternate inner
        IRNode::Value(22.0), // consequent inner
        IRNode::OpCode(OpCode::If(If {
            test: 1,
            consequent: 3,
            alternate: 2,
        })), // inner If = 11.0
        IRNode::OpCode(OpCode::If(If {
            test: 0,
            consequent: 4,
            alternate: 2,
        })), // outer If = 11.0
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 5, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 11.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_block_returns_value() {
    let nodes = vec![
        IRNode::Value(42.0),                              // 0: body value
        IRNode::OpCode(OpCode::Block(Block { body: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 42.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_block_returns_from_break() {
    let nodes = vec![
        IRNode::Value(99.0),                                         // 0: break value
        IRNode::Value(1.0),                                          // 1: break count
        IRNode::OpCode(OpCode::Break(Break { value: 0, count: 1 })), // 2
        IRNode::OpCode(OpCode::Block(Block { body: 2 })),            // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 99.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_block_with_if_breaks() {
    let nodes = vec![
        IRNode::Value(55.5),                                         // 0: break value
        IRNode::Value(1.0),                                          // 1: break count
        IRNode::OpCode(OpCode::Break(Break { value: 0, count: 1 })), // 2
        IRNode::Value(1.0),                                          // 3: true condition
        IRNode::Value(0.0),                                          // 4: else branch
        IRNode::OpCode(OpCode::If(If {
            test: 3,
            consequent: 2,
            alternate: 4,
        })), // 5
        IRNode::OpCode(OpCode::Block(Block { body: 5 })),            // 6
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 6, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 55.5,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_nested_block_breaks_outer() {
    let nodes = vec![
        IRNode::Value(123.0),                                        // 0: break value
        IRNode::Value(2.0),                                          // 1: break count
        IRNode::OpCode(OpCode::Break(Break { value: 0, count: 1 })), // 2
        IRNode::OpCode(OpCode::Block(Block { body: 2 })),            // 3
        IRNode::OpCode(OpCode::Block(Block { body: 3 })),            // 4
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 4, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 123.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_while_immediate_break() {
    // simulate: while(true) { break(1, 42.0) }
    let nodes = vec![
        IRNode::Value(42.0), // 0: break value
        IRNode::Value(1.0),  // 1: break count (pop 1 block)
        IRNode::OpCode(OpCode::Break(Break {
            // 2: break node
            count: 1,
            value: 0,
        })),
        IRNode::Value(1.0), // 3: test (true)
        IRNode::OpCode(OpCode::Block(Block {
            // 4: block node, body of while
            body: 2,
        })),
        IRNode::OpCode(OpCode::While(While {
            // 5: while node
            test: 3,
            body: 4,
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 5, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 42.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_while_without_break() {
    // simulate: while(get(0) != 42) { set(0, add(get(0), 1)) }
    let nodes = vec![
        IRNode::Value(0.0), // 0: memory index 0
        IRNode::OpCode(OpCode::Get(Get {
            block_id: 0,
            index: 0,
        })), // 1: get(0)
        IRNode::Value(42.0), // 2: const 42
        IRNode::OpCode(OpCode::NotEqual(NotEqual { lhs: 1, rhs: 2 })), // 3: get(0) != 42
        IRNode::Value(0.0), // 4: memory index 0
        IRNode::OpCode(OpCode::Get(Get {
            block_id: 0,
            index: 0,
        })), // 5: get(0)
        IRNode::Value(1.0), // 6: const 1
        IRNode::OpCode(OpCode::Add(Add { inputs: vec![5, 6] })), // 7: get(0) + 1
        IRNode::OpCode(OpCode::Set(Set {
            block_id: 0,
            index: 4,
            value: 7,
        })), // 8: set(0, ...)
        IRNode::OpCode(OpCode::Block(Block { body: 8 })), // 9: block with store
        IRNode::OpCode(OpCode::While(While { test: 3, body: 9 })), // 10: while
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 10, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
        assert_eq!(
            runtime_context.memory.read(&runtime_context.as_ctx(), 0, 0),
            Some(42.0),
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_switch() {
    let nodes = vec![
        IRNode::Value(0.0), // 0 - literal 0
        IRNode::OpCode(OpCode::Get(Get {
            block_id: 0,
            index: 0,
        })), // 1
        IRNode::Value(10.0), // 2 - condition of case 0
        IRNode::Value(20.0), // 3 - body of case 0
        IRNode::OpCode(OpCode::Switch(Switch {
            discriminant: 1,
            tests_and_consequents: vec![2, 3],
        })), // 4
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();

        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 10.0);

        // matches case 0
        let result = executor.execute(&nodes, 4, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 20.0,
            "Assertion failed for executor: {}",
            executor_name
        );

        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 5.0);

        // no match → default
        let result = executor.execute(&nodes, 4, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_switch_with_default() {
    let nodes = vec![
        IRNode::Value(0.0), // 0 - literal 0
        IRNode::OpCode(OpCode::Get(Get {
            block_id: 0,
            index: 0,
        })), // 1
        IRNode::Value(10.0), // 2 - condition of case 0
        IRNode::Value(20.0), // 3 - body of case 0
        IRNode::Value(99.0), // 4 - default
        IRNode::OpCode(OpCode::SwitchWithDefault(SwitchWithDefault {
            discriminant: 1,
            tests_and_consequents: vec![2, 3],
            default_consequent: 4,
        })), // 5
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();

        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 10.0);

        // matches case 0
        let result = executor.execute(&nodes, 5, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 20.0,
            "Assertion failed for executor: {}",
            executor_name
        );

        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 5.0);

        // no match → default
        let result = executor.execute(&nodes, 5, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 99.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_switch_integer() {
    let nodes = vec![
        IRNode::Value(0.0), // 0 - literal 0
        IRNode::OpCode(OpCode::Get(Get {
            block_id: 0,
            index: 0,
        })), // 1
        IRNode::Value(10.0), // 2 - body of case 0
        IRNode::Value(20.0), // 3 - body of case 1
        IRNode::OpCode(OpCode::SwitchInteger(SwitchInteger {
            discriminant: 1,
            consequents: vec![2, 3],
        })), // 4
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();

        // matches case 0
        let result = executor.execute(&nodes, 4, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 10.0,
            "Assertion failed for executor: {}",
            executor_name
        );

        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 1.0);

        // matches case 1
        let result = executor.execute(&nodes, 4, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 20.0,
            "Assertion failed for executor: {}",
            executor_name
        );

        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 5.0);

        // no match → default
        let result = executor.execute(&nodes, 4, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_switch_integer_with_default() {
    let nodes = vec![
        IRNode::Value(0.0), // 0 - literal 0
        IRNode::OpCode(OpCode::Get(Get {
            block_id: 0,
            index: 0,
        })), // 1
        IRNode::Value(10.0), // 2 - body of case 0
        IRNode::Value(20.0), // 3 - body of case 1
        IRNode::Value(99.0), // 4 - default
        IRNode::OpCode(OpCode::SwitchIntegerWithDefault(SwitchIntegerWithDefault {
            discriminant: 1,
            consequents: vec![2, 3],
            default_consequent: 4,
        })), // 5
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();

        // matches case 0
        let result = executor.execute(&nodes, 5, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 10.0,
            "Assertion failed for executor: {}",
            executor_name
        );

        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 1.0);

        // matches case 1
        let result = executor.execute(&nodes, 5, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 20.0,
            "Assertion failed for executor: {}",
            executor_name
        );

        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 5.0);

        // no match → default
        let result = executor.execute(&nodes, 5, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 99.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}
