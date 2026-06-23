use sonorust_ir::nodes::*;
use sonorust_runtime::basic::BasicRuntimeContext;
use sonorust_tests::get_available_executors;

#[test]
fn test_not_equal_true() {
    let nodes = vec![
        ResolvedNode::Value(2.0),                                            // 0
        ResolvedNode::Value(3.5),                                            // 1
        ResolvedNode::OpCode(OpCode::NotEqual(NotEqual { lhs: 0, rhs: 1 })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
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
        ResolvedNode::Value(4.5),                                            // 0
        ResolvedNode::Value(4.5),                                            // 1
        ResolvedNode::OpCode(OpCode::NotEqual(NotEqual { lhs: 0, rhs: 1 })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
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
        ResolvedNode::Value(2.0),                                      // 0
        ResolvedNode::Value(2.0),                                      // 1
        ResolvedNode::OpCode(OpCode::Equal(Equal { lhs: 0, rhs: 1 })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
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
        ResolvedNode::Value(2.5),                                      // 0
        ResolvedNode::Value(4.5),                                      // 1
        ResolvedNode::OpCode(OpCode::Equal(Equal { lhs: 0, rhs: 1 })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
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
        ResolvedNode::Value(5.0),                                          // 0
        ResolvedNode::Value(2.0),                                          // 1
        ResolvedNode::OpCode(OpCode::Greater(Greater { lhs: 0, rhs: 1 })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
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
        ResolvedNode::Value(1.0),
        ResolvedNode::Value(10.0),
        ResolvedNode::OpCode(OpCode::Greater(Greater { lhs: 0, rhs: 1 })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
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
        ResolvedNode::Value(3.3),
        ResolvedNode::Value(3.3),
        ResolvedNode::OpCode(OpCode::GreaterOr(GreaterOr { lhs: 0, rhs: 1 })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
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
        ResolvedNode::Value(9.0),
        ResolvedNode::Value(7.5),
        ResolvedNode::OpCode(OpCode::GreaterOr(GreaterOr { lhs: 0, rhs: 1 })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
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
        ResolvedNode::Value(1.0),
        ResolvedNode::Value(2.0),
        ResolvedNode::OpCode(OpCode::GreaterOr(GreaterOr { lhs: 0, rhs: 1 })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
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
        ResolvedNode::Value(-5.0),
        ResolvedNode::Value(3.0),
        ResolvedNode::OpCode(OpCode::Less(Less { lhs: 0, rhs: 1 })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
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
        ResolvedNode::Value(10.0),
        ResolvedNode::Value(1.0),
        ResolvedNode::OpCode(OpCode::Less(Less { lhs: 0, rhs: 1 })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
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
        ResolvedNode::Value(4.4),
        ResolvedNode::Value(4.4),
        ResolvedNode::OpCode(OpCode::LessOr(LessOr { lhs: 0, rhs: 1 })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
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
        ResolvedNode::Value(1.5),
        ResolvedNode::Value(2.0),
        ResolvedNode::OpCode(OpCode::LessOr(LessOr { lhs: 0, rhs: 1 })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
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
        ResolvedNode::Value(5.0),
        ResolvedNode::Value(3.0),
        ResolvedNode::OpCode(OpCode::LessOr(LessOr { lhs: 0, rhs: 1 })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
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
        ResolvedNode::Value(0.0),                            // 0
        ResolvedNode::OpCode(OpCode::Not(Not { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
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
        ResolvedNode::Value(std::f32::consts::PI),           // 0
        ResolvedNode::OpCode(OpCode::Not(Not { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
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
        ResolvedNode::Value(1.0),                                      // 0
        ResolvedNode::Value(2.0),                                      // 1
        ResolvedNode::OpCode(OpCode::And(And { inputs: vec![0, 1] })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);

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
        ResolvedNode::Value(0.0),                                      // 0
        ResolvedNode::Value(2.0),                                      // 1
        ResolvedNode::OpCode(OpCode::And(And { inputs: vec![0, 1] })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
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
        ResolvedNode::Value(2.0),                                      // 0
        ResolvedNode::Value(0.0),                                      // 1
        ResolvedNode::OpCode(OpCode::And(And { inputs: vec![0, 1] })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
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
        ResolvedNode::Value(0.0),                                    // 0
        ResolvedNode::Value(0.0),                                    // 1
        ResolvedNode::OpCode(OpCode::Or(Or { inputs: vec![0, 1] })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
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
        ResolvedNode::Value(5.0),                                    // 0
        ResolvedNode::Value(0.0),                                    // 1
        ResolvedNode::OpCode(OpCode::Or(Or { inputs: vec![0, 1] })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
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
        ResolvedNode::Value(0.0),                                    // 0
        ResolvedNode::Value(8.0),                                    // 1
        ResolvedNode::OpCode(OpCode::Or(Or { inputs: vec![0, 1] })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let mut runtime_context = BasicRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 8.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}
