use sonorust_ir::nodes::*;
use sonorust_runtime::testing::TestingRuntimeContext;
use sonorust_tests::get_available_executors;

#[test]
fn test_not_equal_true() {
    let nodes = vec![
        IRNode::Value(2.0),                                            // 0
        IRNode::Value(3.5),                                            // 1
        IRNode::OpCode(OpCode::NotEqual(NotEqual { lhs: 0.into(), rhs: 1.into() })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 1.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_not_equal_false() {
    let nodes = vec![
        IRNode::Value(4.5),                                            // 0
        IRNode::Value(4.5),                                            // 1
        IRNode::OpCode(OpCode::NotEqual(NotEqual { lhs: 0.into(), rhs: 1.into() })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_equal_true() {
    let nodes = vec![
        IRNode::Value(2.0),                                      // 0
        IRNode::Value(2.0),                                      // 1
        IRNode::OpCode(OpCode::Equal(Equal { lhs: 0.into(), rhs: 1.into() })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 1.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_equal_false() {
    let nodes = vec![
        IRNode::Value(2.5),                                      // 0
        IRNode::Value(4.5),                                      // 1
        IRNode::OpCode(OpCode::Equal(Equal { lhs: 0.into(), rhs: 1.into() })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_greater_true() {
    let nodes = vec![
        IRNode::Value(5.0),                                          // 0
        IRNode::Value(2.0),                                          // 1
        IRNode::OpCode(OpCode::Greater(Greater { lhs: 0.into(), rhs: 1.into() })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 1.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_greater_false() {
    let nodes = vec![
        IRNode::Value(1.0),
        IRNode::Value(10.0),
        IRNode::OpCode(OpCode::Greater(Greater { lhs: 0.into(), rhs: 1.into() })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_greater_or_true_equal() {
    let nodes = vec![
        IRNode::Value(3.3),
        IRNode::Value(3.3),
        IRNode::OpCode(OpCode::GreaterOr(GreaterOr { lhs: 0.into(), rhs: 1.into() })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 1.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_greater_or_true_gt() {
    let nodes = vec![
        IRNode::Value(9.0),
        IRNode::Value(7.5),
        IRNode::OpCode(OpCode::GreaterOr(GreaterOr { lhs: 0.into(), rhs: 1.into() })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 1.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_greater_or_false() {
    let nodes = vec![
        IRNode::Value(1.0),
        IRNode::Value(2.0),
        IRNode::OpCode(OpCode::GreaterOr(GreaterOr { lhs: 0.into(), rhs: 1.into() })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_less_true() {
    let nodes = vec![
        IRNode::Value(-5.0),
        IRNode::Value(3.0),
        IRNode::OpCode(OpCode::Less(Less { lhs: 0.into(), rhs: 1.into() })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 1.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_less_false() {
    let nodes = vec![
        IRNode::Value(10.0),
        IRNode::Value(1.0),
        IRNode::OpCode(OpCode::Less(Less { lhs: 0.into(), rhs: 1.into() })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_less_or_true_eq() {
    let nodes = vec![
        IRNode::Value(4.4),
        IRNode::Value(4.4),
        IRNode::OpCode(OpCode::LessOr(LessOr { lhs: 0.into(), rhs: 1.into() })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 1.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_less_or_true_lt() {
    let nodes = vec![
        IRNode::Value(1.5),
        IRNode::Value(2.0),
        IRNode::OpCode(OpCode::LessOr(LessOr { lhs: 0.into(), rhs: 1.into() })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 1.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_less_or_false() {
    let nodes = vec![
        IRNode::Value(5.0),
        IRNode::Value(3.0),
        IRNode::OpCode(OpCode::LessOr(LessOr { lhs: 0.into(), rhs: 1.into() })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_not_zero() {
    let nodes = vec![
        IRNode::Value(0.0),                            // 0
        IRNode::OpCode(OpCode::Not(Not { value: 0.into() })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 1.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_not_nonzero() {
    let nodes = vec![
        IRNode::Value(std::f32::consts::PI),           // 0
        IRNode::OpCode(OpCode::Not(Not { value: 0.into() })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_and_all_nonzero() {
    let nodes = vec![
        IRNode::Value(1.0),                                    // 0
        IRNode::Value(2.0),                                    // 1
        IRNode::OpCode(OpCode::And(And { args: vec![0.into(), 1.into()] })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);

        // Last input returned
        assert_eq!(
            result, 2.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_and_with_zero_first() {
    let nodes = vec![
        IRNode::Value(0.0),                                    // 0
        IRNode::Value(2.0),                                    // 1
        IRNode::OpCode(OpCode::And(And { args: vec![0.into(), 1.into()] })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_and_with_zero_last() {
    let nodes = vec![
        IRNode::Value(2.0),                                    // 0
        IRNode::Value(0.0),                                    // 1
        IRNode::OpCode(OpCode::And(And { args: vec![0.into(), 1.into()] })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_or_all_zero() {
    let nodes = vec![
        IRNode::Value(0.0),                                  // 0
        IRNode::Value(0.0),                                  // 1
        IRNode::OpCode(OpCode::Or(Or { args: vec![0.into(), 1.into()] })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_or_first_nonzero() {
    let nodes = vec![
        IRNode::Value(5.0),                                  // 0
        IRNode::Value(0.0),                                  // 1
        IRNode::OpCode(OpCode::Or(Or { args: vec![0.into(), 1.into()] })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 5.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_or_second_nonzero() {
    let nodes = vec![
        IRNode::Value(0.0),                                  // 0
        IRNode::Value(8.0),                                  // 1
        IRNode::OpCode(OpCode::Or(Or { args: vec![0.into(), 1.into()] })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2.into(), &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 8.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}
