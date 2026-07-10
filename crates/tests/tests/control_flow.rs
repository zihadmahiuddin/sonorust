use sonorust_ir::nodes::*;
use sonorust_runtime::{access::MemoryAccess, testing::TestingRuntimeContext};
use sonorust_tests::get_available_executors;

#[test]
fn test_execute() {
    let nodes = vec![
        IRNode::Value(1.0),
        IRNode::Value(2.0),
        IRNode::OpCode(OpCode::Add(Add { args: vec![0.into(), 1.into()] })), // = 3.0
        IRNode::Value(100.0),
        IRNode::OpCode(OpCode::Execute(Execute { args: vec![2.into(), 3.into()] })), // should return 100.0
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 4.into(), &mut runtime_context.as_ctx() as _);
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
        IRNode::OpCode(OpCode::Add(Add { args: vec![0.into(), 1.into()] })), // = 3.0
        IRNode::Value(100.0),
        IRNode::OpCode(OpCode::Execute0(Execute0 { args: vec![2.into(), 3.into()] })), // should return 100.0
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 4.into(), &mut runtime_context.as_ctx() as _);
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
        IRNode::Value(1.0),                                              // 0
        IRNode::Value(2.0),                                              // 1
        IRNode::OpCode(OpCode::Add(Add { args: vec![0.into(), 1.into()] })),           // 2 = 3.0
        IRNode::Value(5.0),                                              // 3
        IRNode::OpCode(OpCode::Multiply(Multiply { args: vec![2.into(), 3.into()] })), // 4 = 15.0
        IRNode::OpCode(OpCode::Execute(Execute { args: vec![2.into(), 4.into()] })),   // returns 15.0
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);
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
            test: 0.into(),
            consequent: 1.into(),
            alternate: 2.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3.into(), &mut runtime_context.as_ctx() as _);
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
            test: 0.into(),
            consequent: 1.into(),
            alternate: 2.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3.into(), &mut runtime_context.as_ctx() as _);
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
        IRNode::OpCode(OpCode::Add(Add { args: vec![1.into(), 2.into()] })), // 3 = 5.0
        IRNode::Value(100.0),                                  // alternate
        IRNode::OpCode(OpCode::If(If {
            test: 0.into(),
            consequent: 3.into(),
            alternate: 4.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);
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
        IRNode::OpCode(OpCode::Multiply(Multiply { args: vec![1.into(), 2.into()] })), // 3 = 6.0
        IRNode::Value(42.0),                                             // consequent
        IRNode::OpCode(OpCode::If(If {
            test: 0.into(),
            consequent: 4.into(),
            alternate: 3.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);
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
            test: 1.into(),
            consequent: 3.into(),
            alternate: 2.into(),
        })), // inner If = 11.0
        IRNode::OpCode(OpCode::If(If {
            test: 0.into(),
            consequent: 4.into(),
            alternate: 2.into(),
        })), // outer If = 11.0
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);
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
        IRNode::OpCode(OpCode::Block(Block { body: 0.into() })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1.into(), &mut runtime_context.as_ctx() as _);
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
        IRNode::OpCode(OpCode::Break(Break { value: 0.into(), count: 1.into() })), // 2
        IRNode::OpCode(OpCode::Block(Block { body: 2.into() })),            // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3.into(), &mut runtime_context.as_ctx() as _);
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
        IRNode::OpCode(OpCode::Break(Break { value: 0.into(), count: 1.into() })), // 2
        IRNode::Value(1.0),                                          // 3: true condition
        IRNode::Value(0.0),                                          // 4: else branch
        IRNode::OpCode(OpCode::If(If {
            test: 3.into(),
            consequent: 2.into(),
            alternate: 4.into(),
        })), // 5
        IRNode::OpCode(OpCode::Block(Block { body: 5.into() })),            // 6
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 6.into(), &mut runtime_context.as_ctx() as _);
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
        IRNode::OpCode(OpCode::Break(Break { value: 0.into(), count: 1.into() })), // 2
        IRNode::OpCode(OpCode::Block(Block { body: 2.into() })),            // 3
        IRNode::OpCode(OpCode::Block(Block { body: 3.into() })),            // 4
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 4.into(), &mut runtime_context.as_ctx() as _);
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
            count: 1.into(),
            value: 0.into(),
        })),
        IRNode::Value(1.0), // 3: test (true)
        IRNode::OpCode(OpCode::Block(Block {
            // 4: block node, body of while
            body: 2.into(),
        })),
        IRNode::OpCode(OpCode::While(While {
            // 5: while node
            test: 3.into(),
            body: 4.into(),
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);
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
            block_id: 0.into(),
            index: 0.into(),
        })), // 1: get(0)
        IRNode::Value(42.0), // 2: const 42
        IRNode::OpCode(OpCode::NotEqual(NotEqual { lhs: 1.into(), rhs: 2.into() })), // 3: get(0) != 42
        IRNode::Value(0.0), // 4: memory index 0
        IRNode::OpCode(OpCode::Get(Get {
            block_id: 0.into(),
            index: 0.into(),
        })), // 5: get(0)
        IRNode::Value(1.0), // 6: const 1
        IRNode::OpCode(OpCode::Add(Add { args: vec![5.into(), 6.into()] })), // 7: get(0) + 1
        IRNode::OpCode(OpCode::Set(Set {
            block_id: 0.into(),
            index: 4.into(),
            value: 7.into(),
        })), // 8: set(0, ...)
        IRNode::OpCode(OpCode::Block(Block { body: 8.into() })), // 9: block with store
        IRNode::OpCode(OpCode::While(While { test: 3.into(), body: 9.into() })), // 10: while
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 10.into(), &mut runtime_context.as_ctx() as _);
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
            block_id: 0.into(),
            index: 0.into(),
        })), // 1
        IRNode::Value(10.0), // 2 - condition of case 0
        IRNode::Value(20.0), // 3 - body of case 0
        IRNode::OpCode(OpCode::Switch(Switch {
            discriminant: 1.into(),
            tests_and_consequents: vec![2.into(), 3.into()],
        })), // 4
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();

        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 10.0);

        // matches case 0
        let result = executor.execute(&nodes, 4.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 20.0,
            "Assertion failed for executor: {}",
            executor_name
        );

        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 5.0);

        // no match → default
        let result = executor.execute(&nodes, 4.into(), &mut runtime_context.as_ctx() as _);
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
            block_id: 0.into(),
            index: 0.into(),
        })), // 1
        IRNode::Value(10.0), // 2 - condition of case 0
        IRNode::Value(20.0), // 3 - body of case 0
        IRNode::Value(99.0), // 4 - default
        IRNode::OpCode(OpCode::SwitchWithDefault(SwitchWithDefault {
            discriminant: 1.into(),
            tests_and_consequents: vec![2.into(), 3.into()],
            default_consequent: 4.into(),
        })), // 5
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();

        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 10.0);

        // matches case 0
        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 20.0,
            "Assertion failed for executor: {}",
            executor_name
        );

        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 5.0);

        // no match → default
        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);
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
            block_id: 0.into(),
            index: 0.into(),
        })), // 1
        IRNode::Value(10.0), // 2 - body of case 0
        IRNode::Value(20.0), // 3 - body of case 1
        IRNode::OpCode(OpCode::SwitchInteger(SwitchInteger {
            discriminant: 1.into(),
            consequents: vec![2.into(), 3.into()],
        })), // 4
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();

        // matches case 0
        let result = executor.execute(&nodes, 4.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 10.0,
            "Assertion failed for executor: {}",
            executor_name
        );

        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 1.0);

        // matches case 1
        let result = executor.execute(&nodes, 4.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 20.0,
            "Assertion failed for executor: {}",
            executor_name
        );

        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 5.0);

        // no match → default
        let result = executor.execute(&nodes, 4.into(), &mut runtime_context.as_ctx() as _);
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
            block_id: 0.into(),
            index: 0.into(),
        })), // 1
        IRNode::Value(10.0), // 2 - body of case 0
        IRNode::Value(20.0), // 3 - body of case 1
        IRNode::Value(99.0), // 4 - default
        IRNode::OpCode(OpCode::SwitchIntegerWithDefault(SwitchIntegerWithDefault {
            discriminant: 1.into(),
            consequents: vec![2.into(), 3.into()],
            default_consequent: 4.into(),
        })), // 5
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();

        // matches case 0
        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 10.0,
            "Assertion failed for executor: {}",
            executor_name
        );

        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 1.0);

        // matches case 1
        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 20.0,
            "Assertion failed for executor: {}",
            executor_name
        );

        runtime_context
            .memory
            .write(&runtime_context.as_ctx(), 0, 0, 5.0);

        // no match → default
        let result = executor.execute(&nodes, 5.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 99.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}
