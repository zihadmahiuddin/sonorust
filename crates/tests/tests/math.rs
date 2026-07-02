use sonorust_ir::nodes::*;
use sonorust_runtime::testing::TestingRuntimeContext;
use sonorust_tests::get_available_executors;

#[test]
fn test_abs_negative() {
    let nodes = vec![
        IRNode::Value(-3.5),                           // 0
        IRNode::OpCode(OpCode::Abs(Abs { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 3.5,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_abs_positive() {
    let nodes = vec![
        IRNode::Value(3.5),                            // 0
        IRNode::OpCode(OpCode::Abs(Abs { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 3.5,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_frac_negative() {
    let nodes = vec![
        IRNode::Value(-5.75),                            // 0
        IRNode::OpCode(OpCode::Frac(Frac { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, -0.75,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_frac_positive() {
    let nodes = vec![
        IRNode::Value(5.75),                             // 0
        IRNode::OpCode(OpCode::Frac(Frac { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.75,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_trunc_negative() {
    let nodes = vec![
        IRNode::Value(-4.8),                               // 0
        IRNode::OpCode(OpCode::Trunc(Trunc { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, -4.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_trunc_positive() {
    let nodes = vec![
        IRNode::Value(4.8),                                // 0
        IRNode::OpCode(OpCode::Trunc(Trunc { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 4.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_negate_negative() {
    let nodes = vec![
        IRNode::Value(-6.25),                                // 0
        IRNode::OpCode(OpCode::Negate(Negate { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 6.25,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_negate_positive() {
    let nodes = vec![
        IRNode::Value(6.25),                                 // 0
        IRNode::OpCode(OpCode::Negate(Negate { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, -6.25,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_add() {
    let nodes = vec![
        IRNode::Value(2.0),                                    // 0
        IRNode::Value(3.5),                                    // 1
        IRNode::OpCode(OpCode::Add(Add { args: vec![0, 1] })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 5.5,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_add_chain() {
    let nodes = vec![
        IRNode::Value(1.0),                                    // 0
        IRNode::Value(2.0),                                    // 1
        IRNode::Value(3.0),                                    // 2
        IRNode::OpCode(OpCode::Add(Add { args: vec![0, 1] })), // 3 = 3.0
        IRNode::OpCode(OpCode::Add(Add { args: vec![3, 2] })), // 4 = 6.0
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 4, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 6.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_subtract() {
    let nodes = vec![
        IRNode::Value(10.0),
        IRNode::Value(4.0),
        IRNode::OpCode(OpCode::Subtract(Subtract { args: vec![0, 1] })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 6.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_subtract_negative() {
    let nodes = vec![
        IRNode::Value(5.0),                                              // 0
        IRNode::Value(10.0),                                             // 1
        IRNode::OpCode(OpCode::Subtract(Subtract { args: vec![0, 1] })), // 2 = -5.0
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, -5.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_multiply() {
    let nodes = vec![
        IRNode::Value(3.0),
        IRNode::Value(4.0),
        IRNode::OpCode(OpCode::Multiply(Multiply { args: vec![0, 1] })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 12.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_multiply_zero() {
    let nodes = vec![
        IRNode::Value(0.0),
        IRNode::Value(999.0),
        IRNode::OpCode(OpCode::Multiply(Multiply { args: vec![0, 1] })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_divide() {
    let nodes = vec![
        IRNode::Value(9.0),
        IRNode::Value(3.0),
        IRNode::OpCode(OpCode::Divide(Divide { args: vec![0, 1] })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 3.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_mod() {
    let nodes = vec![
        IRNode::Value(-5.3),                                   // 0
        IRNode::Value(2.0),                                    // 1
        IRNode::OpCode(OpCode::Mod(Mod { args: vec![0, 1] })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);

        let expected = 0.7;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_mod_negative_operand() {
    let nodes = vec![
        IRNode::Value(17.0),                                   // 0
        IRNode::Value(-12.0),                                  // 1
        IRNode::OpCode(OpCode::Mod(Mod { args: vec![0, 1] })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);

        let expected = -7.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_rem() {
    let nodes = vec![
        IRNode::Value(-5.3),                                   // 0
        IRNode::Value(2.0),                                    // 1
        IRNode::OpCode(OpCode::Rem(Rem { args: vec![0, 1] })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);

        let expected = -1.3;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_rem_negative_operand() {
    let nodes = vec![
        IRNode::Value(17.0),                                   // 0
        IRNode::Value(-12.0),                                  // 1
        IRNode::OpCode(OpCode::Rem(Rem { args: vec![0, 1] })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);

        let expected = 5.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_power_no_args_zero() {
    let nodes = vec![IRNode::OpCode(OpCode::Power(Power { args: vec![] }))];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 0, &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {executor_name}, failed to panic as expected",
        );
    }
}

#[test]
fn test_power_single_arg() {
    let nodes = vec![
        IRNode::Value(5.0),                                     // 0
        IRNode::OpCode(OpCode::Power(Power { args: vec![0] })), // 1 = 5
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);

        assert_eq!(
            result, 5.0,
            "Assertion failed for executor: {executor_name}, failed to panic as expected",
        );
    }
}

#[test]
fn test_power_two_args() {
    let nodes = vec![
        IRNode::Value(2.0),                                        // 0
        IRNode::Value(3.0),                                        // 1
        IRNode::OpCode(OpCode::Power(Power { args: vec![0, 1] })), // 2 = 2^3 = 8
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);

        let expected = 8.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_power_three_args() {
    let nodes = vec![
        IRNode::Value(2.0), // 0
        IRNode::Value(3.0), // 1
        IRNode::Value(2.0), // 2
        IRNode::OpCode(OpCode::Power(Power {
            args: vec![0, 1, 2],
        })), // 3 = (2^3)^2 = 8^2 = 64
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);

        let expected = 64.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_power_negative_base_even_exponent() {
    let nodes = vec![
        IRNode::Value(-2.0),                                       // 0
        IRNode::Value(2.0),                                        // 1
        IRNode::OpCode(OpCode::Power(Power { args: vec![0, 1] })), // 2 = (-2)^2 = 4
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);

        let expected = 4.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_power_zero_base() {
    let nodes = vec![
        IRNode::Value(0.0),                                        // 0
        IRNode::Value(5.0),                                        // 1
        IRNode::OpCode(OpCode::Power(Power { args: vec![0, 1] })), // 2 = 0^5 = 0
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);

        let expected = 0.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_power_negative_exponent() {
    let nodes = vec![
        IRNode::Value(4.0),                                        // 0
        IRNode::Value(-1.0),                                       // 1
        IRNode::OpCode(OpCode::Power(Power { args: vec![0, 1] })), // 2 = 4^-1 = 0.25
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);

        let expected = 0.25;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_clamp_within_bounds() {
    let nodes = vec![
        IRNode::Value(2.0), // 0 = min
        IRNode::Value(5.0), // 1 = max
        IRNode::Value(3.5), // 2 = value
        IRNode::OpCode(OpCode::Clamp(Clamp {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);

        let expected = 3.5;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_clamp_below_min() {
    let nodes = vec![
        IRNode::Value(2.0), // 0 = min
        IRNode::Value(5.0), // 1 = max
        IRNode::Value(1.0), // 2 = value
        IRNode::OpCode(OpCode::Clamp(Clamp {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);

        let expected = 2.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_clamp_above_max() {
    let nodes = vec![
        IRNode::Value(2.0), // 0 = min
        IRNode::Value(5.0), // 1 = max
        IRNode::Value(6.0), // 2 = value
        IRNode::OpCode(OpCode::Clamp(Clamp {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);

        let expected = 5.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_lerp_zero() {
    let nodes = vec![
        IRNode::Value(10.0), // 0 = min
        IRNode::Value(20.0), // 1 = max
        IRNode::Value(0.0),  // 2 = value
        IRNode::OpCode(OpCode::Lerp(Lerp {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);

        let expected = 10.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_lerp_one() {
    let nodes = vec![
        IRNode::Value(10.0), // 0 = min
        IRNode::Value(20.0), // 1 = max
        IRNode::Value(1.0),  // 2 = value
        IRNode::OpCode(OpCode::Lerp(Lerp {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);

        let expected = 20.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_lerp_half() {
    let nodes = vec![
        IRNode::Value(10.0), // 0 = min
        IRNode::Value(20.0), // 1 = max
        IRNode::Value(0.5),  // 2 = value
        IRNode::OpCode(OpCode::Lerp(Lerp {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);

        let expected = 15.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_lerp_past_one() {
    let nodes = vec![
        IRNode::Value(10.0), // 0 = min
        IRNode::Value(20.0), // 1 = max
        IRNode::Value(1.5),  // 2 = value
        IRNode::OpCode(OpCode::Lerp(Lerp {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);

        let expected = 25.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_lerp_clamped_below_zero() {
    let nodes = vec![
        IRNode::Value(100.0), // 0 = min
        IRNode::Value(200.0), // 1 = max
        IRNode::Value(-1.0),  // 2 = value (t < 0)
        IRNode::OpCode(OpCode::LerpClamped(LerpClamped {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);

        // clamped to 0.0 → lerp(min, max, 0.0) = min
        let expected = 100.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_lerp_clamped_above_one() {
    let nodes = vec![
        IRNode::Value(100.0), // 0 = min
        IRNode::Value(200.0), // 1 = max
        IRNode::Value(2.0),   // 2 = value (t > 1)
        IRNode::OpCode(OpCode::LerpClamped(LerpClamped {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);

        // clamped to 1.0 → lerp(min, max, 1.0) = max
        let expected = 200.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_lerp_clamped_midpoint() {
    let nodes = vec![
        IRNode::Value(100.0), // 0 = min
        IRNode::Value(200.0), // 1 = max
        IRNode::Value(0.5),   // 2 = value (t = 0.5)
        IRNode::OpCode(OpCode::LerpClamped(LerpClamped {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);

        // lerp(min, max, 0.5)
        let expected = 150.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_lerp_clamped_exact_zero() {
    let nodes = vec![
        IRNode::Value(10.0), // 0 = min
        IRNode::Value(20.0), // 1 = max
        IRNode::Value(0.0),  // 2 = value
        IRNode::OpCode(OpCode::LerpClamped(LerpClamped {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);

        // no clamping needed
        let expected = 10.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_lerp_clamped_exact_one() {
    let nodes = vec![
        IRNode::Value(10.0), // 0 = min
        IRNode::Value(20.0), // 1 = max
        IRNode::Value(1.0),  // 2 = value
        IRNode::OpCode(OpCode::LerpClamped(LerpClamped {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);

        // no clamping needed
        let expected = 20.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_unlerp_zero() {
    let nodes = vec![
        IRNode::Value(10.0), // 0 = min
        IRNode::Value(20.0), // 1 = max
        IRNode::Value(10.0), // 2 = value
        IRNode::OpCode(OpCode::Unlerp(Unlerp {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);

        let expected = 0.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_unlerp_one() {
    let nodes = vec![
        IRNode::Value(10.0), // 0 = min
        IRNode::Value(20.0), // 1 = max
        IRNode::Value(20.0), // 2 = value
        IRNode::OpCode(OpCode::Unlerp(Unlerp {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);

        let expected = 1.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_unlerp_half() {
    let nodes = vec![
        IRNode::Value(0.0), // 0 = min
        IRNode::Value(2.0), // 1 = max
        IRNode::Value(1.0), // 2 = value
        IRNode::OpCode(OpCode::Unlerp(Unlerp {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);

        let expected = 0.5;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_unlerp_below_min() {
    let nodes = vec![
        IRNode::Value(5.0),  // 0 = min
        IRNode::Value(10.0), // 1 = max
        IRNode::Value(0.0),  // 2 = value
        IRNode::OpCode(OpCode::Unlerp(Unlerp {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);

        let expected = -1.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_unlerp_above_max() {
    let nodes = vec![
        IRNode::Value(5.0),  // 0 = min
        IRNode::Value(10.0), // 1 = max
        IRNode::Value(15.0), // 2 = value
        IRNode::OpCode(OpCode::Unlerp(Unlerp {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);

        let expected = 2.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_unlerp_clamped_within_range() {
    let nodes = vec![
        IRNode::Value(0.0),  // 0 = min
        IRNode::Value(10.0), // 1 = max
        IRNode::Value(5.0),  // 2 = value
        IRNode::OpCode(OpCode::UnlerpClamped(UnlerpClamped {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);

        let expected = 0.5;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_unlerp_clamped_below_min() {
    let nodes = vec![
        IRNode::Value(0.0),  // 0 = min
        IRNode::Value(10.0), // 1 = max
        IRNode::Value(-5.0), // 2 = value
        IRNode::OpCode(OpCode::UnlerpClamped(UnlerpClamped {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);

        let expected = 0.0;
        assert!(
            (result - expected).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected {expected}",
        );
    }
}

#[test]
fn test_unlerp_clamped_above_max() {
    let nodes = vec![
        IRNode::Value(0.0),  // 0 = min
        IRNode::Value(10.0), // 1 = max
        IRNode::Value(15.0), // 2 = value
        IRNode::OpCode(OpCode::UnlerpClamped(UnlerpClamped {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 1.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_unlerp_clamped_min_equals_max() {
    let nodes = vec![
        IRNode::Value(10.0), // 0 = min
        IRNode::Value(10.0), // 1 = max
        IRNode::Value(15.0), // 2 = value (irrelevant)
        IRNode::OpCode(OpCode::UnlerpClamped(UnlerpClamped {
            min: 0,
            max: 1,
            value: 2,
        })), // 3
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 3, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_min_left_smaller() {
    let nodes = vec![
        IRNode::Value(2.0),                              // 0 = x
        IRNode::Value(5.0),                              // 1 = y
        IRNode::OpCode(OpCode::Min(Min { x: 0, y: 1 })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 2.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_min_right_smaller() {
    let nodes = vec![
        IRNode::Value(7.0),                              // 0 = x
        IRNode::Value(3.0),                              // 1 = y
        IRNode::OpCode(OpCode::Min(Min { x: 0, y: 1 })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 3.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_min_equal() {
    let nodes = vec![
        IRNode::Value(4.0),                              // 0 = x
        IRNode::Value(4.0),                              // 1 = y
        IRNode::OpCode(OpCode::Min(Min { x: 0, y: 1 })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 4.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_max_left_greater() {
    let nodes = vec![
        IRNode::Value(8.0),                              // 0 = x
        IRNode::Value(6.0),                              // 1 = y
        IRNode::OpCode(OpCode::Max(Max { x: 0, y: 1 })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 8.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_max_right_greater() {
    let nodes = vec![
        IRNode::Value(1.0),                              // 0 = x
        IRNode::Value(9.0),                              // 1 = y
        IRNode::OpCode(OpCode::Max(Max { x: 0, y: 1 })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 9.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_max_equal() {
    let nodes = vec![
        IRNode::Value(7.0),                              // 0 = x
        IRNode::Value(7.0),                              // 1 = y
        IRNode::OpCode(OpCode::Max(Max { x: 0, y: 1 })), // 2
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 7.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_remap_basic() {
    let nodes = vec![
        IRNode::Value(0.0),   // 0 = from_min
        IRNode::Value(10.0),  // 1 = from_max
        IRNode::Value(0.0),   // 2 = to_min
        IRNode::Value(100.0), // 3 = to_max
        IRNode::Value(5.0),   // 4 = value
        IRNode::OpCode(OpCode::Remap(Remap {
            from_min: 0,
            from_max: 1,
            to_min: 2,
            to_max: 3,
            value: 4,
        })), // 5
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 5, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 50.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_remap_value_below_min() {
    let nodes = vec![
        IRNode::Value(0.0),   // 0 = from_min
        IRNode::Value(10.0),  // 1 = from_max
        IRNode::Value(0.0),   // 2 = to_min
        IRNode::Value(100.0), // 3 = to_max
        IRNode::Value(-5.0),  // 4 = value
        IRNode::OpCode(OpCode::Remap(Remap {
            from_min: 0,
            from_max: 1,
            to_min: 2,
            to_max: 3,
            value: 4,
        })), // 5
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 5, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, -50.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_remap_from_min_equals_max() {
    let nodes = vec![
        IRNode::Value(1.0), // 0 = from_min
        IRNode::Value(1.0), // 1 = from_max
        IRNode::Value(2.0), // 2 = to_min
        IRNode::Value(4.0), // 3 = to_max
        IRNode::Value(1.0), // 4 = value
        IRNode::OpCode(OpCode::Remap(Remap {
            from_min: 0,
            from_max: 1,
            to_min: 2,
            to_max: 3,
            value: 4,
        })), // 5
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 5, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 2.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_remap_clamped_in_range() {
    let nodes = vec![
        IRNode::Value(0.0),   // 0 = from_min
        IRNode::Value(10.0),  // 1 = from_max
        IRNode::Value(0.0),   // 2 = to_min
        IRNode::Value(100.0), // 3 = to_max
        IRNode::Value(5.0),   // 4 = value
        IRNode::OpCode(OpCode::RemapClamped(RemapClamped {
            from_min: 0,
            from_max: 1,
            to_min: 2,
            to_max: 3,
            value: 4,
        })), // 5
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 5, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 50.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_remap_clamped_below_min() {
    let nodes = vec![
        IRNode::Value(0.0),
        IRNode::Value(10.0),
        IRNode::Value(0.0),
        IRNode::Value(100.0),
        IRNode::Value(-5.0),
        IRNode::OpCode(OpCode::RemapClamped(RemapClamped {
            from_min: 0,
            from_max: 1,
            to_min: 2,
            to_max: 3,
            value: 4,
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 5, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_remap_clamped_above_max() {
    let nodes = vec![
        IRNode::Value(0.0),
        IRNode::Value(10.0),
        IRNode::Value(0.0),
        IRNode::Value(100.0),
        IRNode::Value(20.0),
        IRNode::OpCode(OpCode::RemapClamped(RemapClamped {
            from_min: 0,
            from_max: 1,
            to_min: 2,
            to_max: 3,
            value: 4,
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 5, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 100.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_remap_clamped_min_equals_max() {
    let nodes = vec![
        IRNode::Value(1.0),
        IRNode::Value(1.0),
        IRNode::Value(10.0),
        IRNode::Value(20.0),
        IRNode::Value(1.0),
        IRNode::OpCode(OpCode::RemapClamped(RemapClamped {
            from_min: 0,
            from_max: 1,
            to_min: 2,
            to_max: 3,
            value: 4,
        })),
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 5, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 10.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_round_half_up_even() {
    let nodes = vec![
        IRNode::Value(2.5),                                // 0
        IRNode::OpCode(OpCode::Round(Round { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 2.0,
            "Assertion failed for executor: {}",
            executor_name
        ); // ties to even
    }
}

#[test]
fn test_round_half_up_odd() {
    let nodes = vec![
        IRNode::Value(3.5),                                // 0
        IRNode::OpCode(OpCode::Round(Round { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 4.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_round_negative_half() {
    let nodes = vec![
        IRNode::Value(-1.5),                               // 0
        IRNode::OpCode(OpCode::Round(Round { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, -2.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_floor_positive() {
    let nodes = vec![
        IRNode::Value(3.7),                                // 0
        IRNode::OpCode(OpCode::Floor(Floor { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 3.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_floor_negative() {
    let nodes = vec![
        IRNode::Value(-1.2),                               // 0
        IRNode::OpCode(OpCode::Floor(Floor { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, -2.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_ceil_positive() {
    let nodes = vec![
        IRNode::Value(2.1),                              // 0
        IRNode::OpCode(OpCode::Ceil(Ceil { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 3.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_ceil_negative() {
    let nodes = vec![
        IRNode::Value(-3.9),                             // 0
        IRNode::OpCode(OpCode::Ceil(Ceil { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, -3.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_sin() {
    let pi = std::f32::consts::PI;
    let inputs = [0.0, pi / 2.0, pi, -pi / 2.0];
    let expected = [0.0, 1.0, 0.0, -1.0];

    for (&x, &y) in inputs.iter().zip(expected.iter()) {
        let nodes = vec![
            IRNode::Value(x),
            IRNode::OpCode(OpCode::Sin(Sin { value: 0 })),
        ];

        let executors = get_available_executors();
        for (executor_name, mut executor) in executors {
            let runtime_context = TestingRuntimeContext::default();
            let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
            assert!(
                (result - y).abs() < 1e-6,
                "Assertion failed for executor: {executor_name}, sin({x}) = {result}, expected {y}",
            );
        }
    }
}

#[test]
fn test_cos() {
    let pi = std::f32::consts::PI;
    let inputs = [0.0, pi / 2.0, pi, -pi / 2.0];
    let expected = [1.0, 0.0, -1.0, 0.0];

    for (&x, &y) in inputs.iter().zip(expected.iter()) {
        let nodes = vec![
            IRNode::Value(x),
            IRNode::OpCode(OpCode::Cos(Cos { value: 0 })),
        ];

        let executors = get_available_executors();
        for (executor_name, mut executor) in executors {
            let runtime_context = TestingRuntimeContext::default();
            let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
            assert!(
                (result - y).abs() < 1e-6,
                "Assertion failed for executor: {executor_name}, cos({x}) = {result}, expected {y}",
            );
        }
    }
}

#[test]
fn test_tan() {
    let pi = std::f32::consts::PI;
    let inputs = [0.0, pi / 4.0, -pi / 4.0];
    let expected = [0.0, 1.0, -1.0];

    for (&x, &y) in inputs.iter().zip(expected.iter()) {
        let nodes = vec![
            IRNode::Value(x),
            IRNode::OpCode(OpCode::Tan(Tan { value: 0 })),
        ];

        let executors = get_available_executors();
        for (executor_name, mut executor) in executors {
            let runtime_context = TestingRuntimeContext::default();
            let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
            assert!(
                (result - y).abs() < 1e-6,
                "Assertion failed for executor: {executor_name}, tan({x}) = {result}, expected {y}",
            );
        }
    }
}

#[test]
fn test_sinh() {
    let inputs = [0.0, 1.0, -1.0];
    #[allow(clippy::excessive_precision)]
    let expected = [0.0, 1.1752011936, -1.1752011936];

    for (&x, &y) in inputs.iter().zip(expected.iter()) {
        let nodes = vec![
            IRNode::Value(x),
            IRNode::OpCode(OpCode::Sinh(Sinh { value: 0 })),
        ];

        let executors = get_available_executors();
        for (executor_name, mut executor) in executors {
            let runtime_context = TestingRuntimeContext::default();
            let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
            assert!(
                (result - y).abs() < 1e-6,
                "Assertion failed for executor: {executor_name}, sinh({x}) = {result}, expected {y}",
            );
        }
    }
}

#[test]
fn test_cosh() {
    let inputs = [0.0, 1.0, -1.0];
    #[allow(clippy::excessive_precision)]
    let expected = [1.0, 1.5430806348, 1.5430806348];

    for (&x, &y) in inputs.iter().zip(expected.iter()) {
        let nodes = vec![
            IRNode::Value(x),
            IRNode::OpCode(OpCode::Cosh(Cosh { value: 0 })),
        ];

        let executors = get_available_executors();
        for (executor_name, mut executor) in executors {
            let runtime_context = TestingRuntimeContext::default();
            let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
            assert!(
                (result - y).abs() < 1e-6,
                "Assertion failed for executor: {executor_name}, cosh({x}) = {result}, expected {y}",
            );
        }
    }
}

#[test]
fn test_tanh() {
    let inputs = [0.0, 1.0, -1.0];
    #[allow(clippy::excessive_precision)]
    let expected = [0.0, 0.7615941559, -0.7615941559];

    for (&x, &y) in inputs.iter().zip(expected.iter()) {
        let nodes = vec![
            IRNode::Value(x),
            IRNode::OpCode(OpCode::Tanh(Tanh { value: 0 })),
        ];

        let executors = get_available_executors();
        for (executor_name, mut executor) in executors {
            let runtime_context = TestingRuntimeContext::default();
            let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
            assert!(
                (result - y).abs() < 1e-6,
                "Assertion failed for executor: {executor_name}, tanh({x}) = {result}, expected {y}",
            );
        }
    }
}

#[test]
fn test_asin() {
    let inputs = [-1.0, -0.5, 0.0, 0.5, 1.0];
    let expected = [
        -std::f32::consts::FRAC_PI_2,
        -std::f32::consts::FRAC_PI_6,
        0.0,
        std::f32::consts::FRAC_PI_6,
        std::f32::consts::FRAC_PI_2,
    ];

    for (&x, &y) in inputs.iter().zip(expected.iter()) {
        let nodes = vec![
            IRNode::Value(x),
            IRNode::OpCode(OpCode::Arcsin(Arcsin { value: 0 })),
        ];

        let executors = get_available_executors();
        for (executor_name, mut executor) in executors {
            let runtime_context = TestingRuntimeContext::default();
            let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
            assert!(
                (result - y).abs() < 1e-6,
                "Assertion failed for executor: {executor_name}, asin({x}) = {result}, expected {y}",
            );
        }
    }
}

#[test]
fn test_acos() {
    let inputs = [-1.0, -0.5, 0.0, 0.5, 1.0];
    let expected = [
        std::f32::consts::PI,
        #[allow(clippy::excessive_precision)]
        2.0943951024, // 2π/3
        std::f32::consts::FRAC_PI_2,
        std::f32::consts::FRAC_PI_3,
        0.0,
    ];

    for (&x, &y) in inputs.iter().zip(expected.iter()) {
        let nodes = vec![
            IRNode::Value(x),
            IRNode::OpCode(OpCode::Arccos(Arccos { value: 0 })),
        ];

        let executors = get_available_executors();
        for (executor_name, mut executor) in executors {
            let runtime_context = TestingRuntimeContext::default();
            let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
            assert!(
                (result - y).abs() < 1e-6,
                "Assertion failed for executor: {executor_name}, acos({x}) = {result}, expected {y}",
            );
        }
    }
}

#[test]
fn test_atan() {
    let inputs = [-1.0, -0.5, 0.0, 0.5, 1.0];
    let expected = [
        -std::f32::consts::FRAC_PI_4,
        #[allow(clippy::excessive_precision)]
        -0.4636476090,
        0.0,
        #[allow(clippy::excessive_precision)]
        0.4636476090,
        std::f32::consts::FRAC_PI_4,
    ];

    for (&x, &y) in inputs.iter().zip(expected.iter()) {
        let nodes = vec![
            IRNode::Value(x),
            IRNode::OpCode(OpCode::Arctan(Arctan { value: 0 })),
        ];

        let executors = get_available_executors();
        for (executor_name, mut executor) in executors {
            let runtime_context = TestingRuntimeContext::default();
            let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
            assert!(
                (result - y).abs() < 1e-6,
                "Assertion failed for executor: {executor_name}, atan({x}) = {result}, expected {y}",
            );
        }
    }
}

#[test]
fn test_atan2() {
    let pairs = [
        (0.0, 1.0),  // 0
        (1.0, 0.0),  // π/2
        (0.0, -1.0), // π
        (-1.0, 0.0), // -π/2
        (1.0, 1.0),  // π/4
    ];
    let expected = [
        0.0,
        std::f32::consts::FRAC_PI_2,
        std::f32::consts::PI,
        -std::f32::consts::FRAC_PI_2,
        std::f32::consts::FRAC_PI_4,
    ];

    for ((y, x), expected) in pairs.iter().zip(expected.iter()) {
        let nodes = vec![
            IRNode::Value(*y),
            IRNode::Value(*x),
            IRNode::OpCode(OpCode::Arctan2(Arctan2 { y: 0, x: 1 })),
        ];

        let executors = get_available_executors();
        for (executor_name, mut executor) in executors {
            let runtime_context = TestingRuntimeContext::default();
            let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
            assert!(
                (result - expected).abs() < 1e-6,
                "Assertion failed for executor: {executor_name}, atan2({y}, {x}) = {result}, expected {expected}",
            );
        }
    }
}

#[test]
fn test_degree_pi() {
    let nodes = vec![
        IRNode::Value(std::f32::consts::PI),                 // 0
        IRNode::OpCode(OpCode::Degree(Degree { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert!(
            (result - 180.0).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected 180.0"
        );
    }
}

#[test]
fn test_degree_zero() {
    let nodes = vec![
        IRNode::Value(0.0),                                  // 0
        IRNode::OpCode(OpCode::Degree(Degree { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_degree_negative() {
    let nodes = vec![
        IRNode::Value(-std::f32::consts::PI / 2.0),          // 0
        IRNode::OpCode(OpCode::Degree(Degree { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert!(
            (result + 90.0).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}, got {result}, expected -90.0"
        );
    }
}

#[test]
fn test_radian_180() {
    let nodes = vec![
        IRNode::Value(180.0),                                // 0
        IRNode::OpCode(OpCode::Radian(Radian { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert!(
            (result - std::f32::consts::PI).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}"
        );
    }
}

#[test]
fn test_radian_zero() {
    let nodes = vec![
        IRNode::Value(0.0),                                  // 0
        IRNode::OpCode(OpCode::Radian(Radian { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_radian_negative() {
    let nodes = vec![
        IRNode::Value(-90.0),                                // 0
        IRNode::OpCode(OpCode::Radian(Radian { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert!(
            (result + std::f32::consts::FRAC_PI_2).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}"
        );
    }
}

#[test]
fn test_log_regular_value() {
    let nodes = vec![
        IRNode::Value(10.0),                           // 0 = value
        IRNode::OpCode(OpCode::Log(Log { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert!(
            (result - 10f32.ln()).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}"
        );
    }
}

#[test]
fn test_log_of_1() {
    let nodes = vec![
        IRNode::Value(1.0),                            // 0 = value
        IRNode::OpCode(OpCode::Log(Log { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_log_of_e() {
    let nodes = vec![
        IRNode::Value(std::f32::consts::E),            // 0 = value
        IRNode::OpCode(OpCode::Log(Log { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert!(
            (result - 1.0).abs() < 1e-6,
            "Assertion failed for executor: {executor_name}"
        );
    }
}

#[test]
fn test_log_of_zero() {
    let nodes = vec![
        IRNode::Value(0.0),                            // 0 = value
        IRNode::OpCode(OpCode::Log(Log { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert!(
            result.is_infinite() && result.is_sign_negative(),
            "Assertion failed for executor: {executor_name}"
        );
    }
}

#[test]
fn test_log_of_negative() {
    let nodes = vec![
        IRNode::Value(-1.0),                           // 0 = value
        IRNode::OpCode(OpCode::Log(Log { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert!(
            result.is_nan(),
            "Assertion failed for executor: {executor_name}"
        );
    }
}

#[test]
fn test_sign_positive() {
    let nodes = vec![
        IRNode::Value(42.0),                             // 0
        IRNode::OpCode(OpCode::Sign(Sign { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 1.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_sign_negative() {
    let nodes = vec![
        IRNode::Value(-std::f32::consts::PI),            // 0
        IRNode::OpCode(OpCode::Sign(Sign { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, -1.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_sign_zero() {
    let nodes = vec![
        IRNode::Value(0.0),                              // 0
        IRNode::OpCode(OpCode::Sign(Sign { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert_eq!(
            result, 0.0,
            "Assertion failed for executor: {}",
            executor_name
        );
    }
}

#[test]
fn test_sign_nan() {
    let nodes = vec![
        IRNode::Value(f32::NAN),                         // 0
        IRNode::OpCode(OpCode::Sign(Sign { value: 0 })), // 1
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        let result = executor.execute(&nodes, 1, &mut runtime_context.as_ctx() as _);
        assert!(
            result.is_nan(),
            "Assertion failed for executor: {executor_name}"
        );
    }
}

#[test]
fn test_random_range_and_variance() {
    let nodes = vec![
        IRNode::Value(5.0),                                        // min
        IRNode::Value(10.0),                                       // max
        IRNode::OpCode(OpCode::Random(Random { min: 0, max: 1 })), // random node
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        executor.prepare(&nodes, 2);

        let mut results = std::collections::HashSet::new();
        for _ in 0..10 {
            let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
            assert!(
                (5.0..=10.0).contains(&result),
                "Random result {result} out of range [5.0, 10.0] for executor {executor_name}",
            );
            results.insert(result.to_bits()); // use bit pattern to avoid NaN weirdness
        }

        assert!(
            results.len() > 1,
            "All random results were the same: {results:?} for executor {executor_name}",
        );
    }
}

#[test]
fn test_random_integer_range_and_integral() {
    let nodes = vec![
        IRNode::Value(1.0), // min inclusive
        IRNode::Value(5.0), // max exclusive
        IRNode::OpCode(OpCode::RandomInteger(RandomInteger { min: 0, max: 1 })), // random int node
    ];

    let executors = get_available_executors();
    for (executor_name, mut executor) in executors {
        let runtime_context = TestingRuntimeContext::default();
        executor.prepare(&nodes, 2);

        let mut results = std::collections::HashSet::new();
        for _ in 0..10 {
            let result = executor.execute(&nodes, 2, &mut runtime_context.as_ctx() as _);
            assert!(
                (1.0..5.0).contains(&result),
                "RandomInteger result {result} out of range [1.0, 5.0) for executor {executor_name}",
            );
            assert_eq!(
                result.fract(),
                0.0,
                "RandomInteger result {result} is not an integer for executor {executor_name}",
            );
            results.insert(result.to_bits());
        }

        assert!(
            results.len() > 1,
            "All RandomInteger results were the same: {results:?} for executor {executor_name}",
        );
    }
}
