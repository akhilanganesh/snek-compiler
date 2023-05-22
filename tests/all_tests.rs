mod infra;

// Your tests go here!
success_tests! {
    // Number and Boolean Literals
    {
        name: num,
        file: "grading/num.snek",
        expected: "644",
    },
    {
        name: false_val,
        file: "grading/false_val.snek",
        expected: "false",
    },

    // Input Expression
    {
        name: input_default,
        file: "grading/input0.snek",
        expected: "false",
    },
    {
        name: input_bool,
        file: "grading/input0.snek",
        input: "true",
        expected: "true",
    },
    {
        name: input_num,
        file: "grading/input0.snek",
        input: "123",
        expected: "123",
    },

    // Simple Number Expressions
    {
        name: add1,
        file: "grading/add1.snek",
        expected: "73",
    },
    {
        name: add1_sub1,
        file: "grading/add1_sub1.snek",
        expected: "4",
    },
    {
        name: add_num,
        file: "grading/add.snek",
        input: "10",
        expected: "15",
    },

    // Nested Arithmetic Expressions
    {
        name: nested_arith0,
        file: "grading/nested_arith0.snek",
        expected: "35",
    },
    {
        name: nested_arith1,
        file: "grading/nested_arith1.snek",
        expected: "25",
    },
    {
        name: nested_arith2,
        file: "grading/nested_arith2.snek",
        expected: "0",
    },
    {
        name: nested_arith3,
        file: "grading/nested_arith3.snek",
        input: "8",
        expected: "1117",
    },
    {
        name: nested_arith4,
        file: "grading/nested_arith4.snek",
        expected: "-1",
    },

    // Dynamic Type Checks with isnum/isbool
    {
        name: type_check_succ0,
        file: "grading/isnum.snek",
        expected: "false",
    },
    {
        name: type_check_succ1,
        file: "grading/isnum.snek",
        input: "547",
        expected: "true",
    },
    {
        name: type_check_succ2,
        file: "grading/isnum.snek",
        input: "true",
        expected: "false",
    },
    {
        name: type_check_succ3,
        file: "grading/isbool.snek",
        expected: "true",
    },
    {
        name: type_check_succ4,
        file: "grading/isbool.snek",
        input: "689",
        expected: "false",
    },
    {
        name: type_check_succ5,
        file: "grading/type_check_succ5.snek",
        expected: "true",
    },

    // Comparison Expressions
    {
        name: compare_expr_succ0,
        file: "grading/compare_expr_succ0.snek",
        expected: "true",
    },

    {
        name: compare_expr_succ2,
        file: "grading/compare_expr_succ2.snek",
        expected: "true",
    },

    // Let expressions
    {
        name: binding0,
        file: "grading/binding0.snek",
        expected: "5",
    },
    {
        name: binding1,
        file: "grading/binding1.snek",
        expected: "-5",
    },

    {
        name: binding_expr,
        file: "grading/binding_expr.snek",
        expected: "1225",
    },
    {
        name: binding_nested,
        file: "grading/binding_nested.snek",
        expected: "1",
    },

    {
        name: binding_chain,
        file: "grading/binding_chain.snek",
        expected: "3",
    },
    {
        name: binding_nested_chain,
        file: "grading/binding_nested_chain.snek",
        expected: "12",
    },

    // Let expressions with shadowing
    {
        name: shadowed_binding_succ0,
        file: "grading/shadowed_binding_succ0.snek",
        expected: "100",
    },
    {
        name: shadowed_binding_succ1,
        file: "grading/shadowed_binding_succ1.snek",
        expected: "7",
    },
    {
        name: shadowed_binding_succ2,
        file: "grading/shadowed_binding_succ2.snek",
        expected: "150",
    },
    {
        name: shadowed_binding_succ3,
        file: "grading/shadowed_binding_succ3.snek",
        expected: "5",
    },
    {
        name: shadowed_binding_succ4,
        file: "grading/shadowed_binding_succ4.snek",
        expected: "18",
    },
    {
        name: shadowed_binding_succ5,
        file: "grading/shadowed_binding_succ5.snek",
        expected: "5",
    },
    {
        name: shadowed_binding_succ6,
        file: "grading/shadowed_binding_succ6.snek",
        expected: "3",
    },
    {
        name: shadowed_binding_succ7,
        file: "grading/shadowed_binding_succ7.snek",
        expected: "200",
    },

    // Misc complex expressions with arithmetic and let bindings
    {
        name: complex_expr,
        file: "grading/complex_expr.snek",
        expected: "6",
    },
    {
        name: quick_brown_fox,
        file: "grading/quick_brown_fox.snek",
        expected: "-3776",
    },

    // If expressions
    {
        name: if_expr_succ0,
        file: "grading/if_expr_succ0.snek",
        expected: "10",
    },
    {
        name: if_expr_succ1,
        file: "grading/if_expr_input.snek",
        input: "635",
        expected: "20",
    },
    {
        name: if_expr_succ2,
        file: "grading/if_expr_succ2.snek",
        expected: "8",
    },
    {
        name: if_expr_succ3,
        file: "grading/if_expr_succ3.snek",
        expected: "7",
    },

    // Set expr
    {
        name: set_expr_succ0,
        file: "grading/set_expr1.snek",
        expected: "true",
    },
    {
        name: set_expr_succ1,
        file: "grading/set_expr2.snek",
        expected: "25",
    },
    {
        name: set_expr_succ2,
        file: "grading/set_expr3.snek",
        input: "25",
        expected: "true",
    },
    {
        name: set_expr_succ3,
        file: "grading/set_expr3.snek",
        input: "20",
        expected: "false",
    },

    {
        name: loop_expr_succ0,
        file: "grading/loop_expr0.snek",
        input: "3",
        expected: "6",
    },
    {
        name: loop_expr_succ1,
        file: "grading/loop_expr0.snek",
        input: "7",
        expected: "5040",
    },
    {
        name: loop_expr_succ2,
        file: "grading/loop_expr1.snek",
        expected: "-6",
    },

    // ALL OTHER TESTS
    {
        name: test_plus1,
        file: "self/test1.snek",
        expected: "125",
    },
    {
        name: test_plus2,
        file: "self/test2.snek",
        expected: "-86",
    },
    {
        name: test_times,
        file: "self/test3.snek",
        expected: "405",
    },
    {
        name: test_add1,
        file: "self/test4.snek",
        expected: "77",
    },
    {
        name: test_sub1,
        file: "self/test5.snek",
        expected: "188",
    },
    {
        name: test_multiop1,
        file: "self/test6.snek",
        expected: "405",
    },
    {
        name: test_multiop2,
        file: "self/test7.snek",
        expected: "405",
    },
    {
        name: test_letop,
        file: "self/test8.snek",
        expected: "1482",
    },
    {
        name: fact,
        file: "self/fact.snek",
        input: "10",
        expected: "3628800",
    },
    {
        name: fact_recursive,
        file: "self/fact_recursive.snek",
        input: "10",
        expected: "3628800",
    },
    {
        name: even_odd_1,
        file: "self/even_odd.snek",
        input: "10",
        expected: "10\ntrue\ntrue",
    },
    {
        name: even_odd_2,
        file: "self/even_odd.snek",
        input: "9",
        expected: "9\nfalse\nfalse",
    },
}

runtime_error_tests! {
    // integer overflow
    {
        name: number_overflow_fail0,
        file: "grading/number_overflow_fail0.snek",
        expected: "overflow",
    },
    {
        name: number_overflow_fail1,
        file: "grading/number_overflow_fail1.snek",
        expected: "overflow",
    },
    {
        name: number_overflow_fail2,
        file: "grading/add.snek",
        input: "4611686018427387899",
        expected: "overflow",
    },
    {
        name: number_overflow_fail3,
        file: "grading/nested_arith3.snek",
        input: "4611686018427387890",
        expected: "overflow",
    },

    // type mismatch
    {
        name: invalid_argument_fail0,
        file: "grading/invalid_argument_fail0.snek",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail1,
        file: "grading/invalid_argument_fail1.snek",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail2,
        file: "grading/invalid_argument_fail2.snek",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail3,
        file: "grading/invalid_argument_fail3.snek",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail4,
        file: "grading/invalid_argument_fail4.snek",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail5,
        file: "grading/invalid_argument_fail5.snek",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail6,
        file: "grading/invalid_argument_fail6.snek",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail7,
        file: "grading/nested_arith3.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail8,
        file: "grading/if_expr_input.snek",
        input: "665",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail9,
        file: "grading/set_expr3.snek",
        input: "true",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail10,
        file: "grading/loop_expr0.snek",
        input: "5",
        expected: "invalid argument",
    },
    {
        name: invalid_argument_fail11,
        file: "grading/invalid_argument_fail11.snek",
        expected: "invalid argument",
    },

    // self tests
    {
        name: ia_fun_list,
        file: "self/fun_list.snek",
        expected: "invalid argument",
    }
}

static_error_tests! {

    // Invalid S-expressions
    {
        name: parse_sexp_fail1,
        file: "grading/parse_sexp_fail1.snek",
        expected: "Invalid",
    },
    {
        name: parse_sexp_fail2,
        file: "grading/parse_sexp_fail2.snek",
        expected: "Invalid",
    },

    // Invalid tokens/operators
    {
        name: parse_token_fail1,
        file: "grading/parse_token_fail1.snek",
        expected: "Invalid",
    },
    {
        name: parse_token_fail2,
        file: "grading/parse_token_fail2.snek",
        expected: "Invalid",
    },
    {
        name: parse_token_fail3,
        file: "grading/parse_token_fail3.snek",
        expected: "Invalid",
    },
    {
        name: parse_token_fail4,
        file: "grading/parse_token_fail4.snek",
        expected: "Invalid",
    },


    // Invalid/Out of bounds Number Literal
    {
        name: number_bounds_fail0,
        file: "grading/number_bounds_fail0.snek",
        expected: "Invalid",
    },
    {
        name: number_bounds_fail1,
        file: "grading/number_bounds_fail1.snek",
        expected: "Invalid",
    },

    // Invalid operator arguments
    {
        name: parse_op_fail1,
        file: "grading/parse_op_fail1.snek",
        expected: "Invalid",
    },
    {
        name: parse_op_fail2,
        file: "grading/parse_op_fail2.snek",
        expected: "Invalid",
    },
    {
        name: parse_op_fail3,
        file: "grading/parse_op_fail3.snek",
        expected: "Invalid",
    },
    {
        name: parse_op_fai4,
        file: "grading/parse_op_fail4.snek",
        expected: "Invalid",
    },
    {
        name: parse_op_fail5,
        file: "grading/parse_op_fail5.snek",
        expected: "Invalid",
    },
    {
        name: parse_op_fail6,
        file: "grading/parse_op_fail6.snek",
        expected: "Invalid",
    },
    {
        name: parse_op_fail7,
        file: "grading/parse_op_fail7.snek",
        expected: "Invalid",
    },
    {
        name: parse_op_fail8,
        file: "grading/parse_op_fail8.snek",
        expected: "Invalid",
    },

    // Invalid let expressions
    {
        name: parse_let_nobindings_fail,
        file: "grading/parse_let_nobindings_fail.snek",
        expected: "Invalid",
    },
    {
        name: parse_let_improperargs_fail1,
        file: "grading/parse_let_improperargs_fail1.snek",
        expected: "Invalid",
    },
    {
        name: parse_let_improperargs_fail2,
        file: "grading/parse_let_improperargs_fail2.snek",
        expected: "Invalid",
    },
    {
        name: parse_let_improperargs_fail3,
        file: "grading/parse_let_improperargs_fail3.snek",
        expected: "Invalid",
    },
    {
        name: parse_let_improperargs_fail4,
        file: "grading/parse_let_improperargs_fail4.snek",
        expected: "Invalid",
    },
    {
        name: parse_let_improperargs_fail5,
        file: "grading/parse_let_improperargs_fail5.snek",
        expected: "keyword",
    },

    {
        name: duplicate_binding_fail0,
        file: "grading/duplicate_binding_fail0.snek",
        expected: "Duplicate binding",
    },
    {
        name: duplicate_binding_fail1,
        file: "grading/duplicate_binding_fail1.snek",
        expected: "Duplicate binding",
    },
    {
        name: duplicate_binding_fail2,
        file: "grading/duplicate_binding_fail2.snek",
        expected: "Duplicate binding",
    },

    // Invalid if expressions
    {
        name: parse_if_fail0,
        file: "grading/parse_if_fail0.snek",
        expected: "Invalid",
    },
    {
        name: parse_if_fail1,
        file: "grading/parse_if_fail1.snek",
        expected: "Invalid",
    },

    // Unbound identifier
    {
        name: unbound_identifier_fail0,
        file: "grading/unbound_identifier_fail0.snek",
        expected: "Unbound variable identifier x",
    },
    {
        name: unbound_identifier_fail1,
        file: "grading/unbound_identifier_fail1.snek",
        expected: "Unbound variable identifier y",
    },
    {
        name: unbound_identifier_fail2,
        file: "grading/unbound_identifier_fail2.snek",
        expected: "Unbound variable identifier x",
    },
    {
        name: unbound_identifier_fail3,
        file: "grading/unbound_identifier_fail3.snek",
        expected: "Unbound variable identifier z",
    },
    {
        name: unbound_identifier_fail4,
        file: "grading/unbound_identifier_fail4.snek",
        expected: "Unbound variable identifier t",
    },
    {
        name: unbound_identifier_fail5,
        file: "grading/unbound_identifier_fail5.snek",
        expected: "Unbound variable identifier x",
    },

    // Invalid block
    {
        name: parse_block_fail0,
        file: "grading/parse_block_fail0.snek",
        expected: "Invalid",
    },

    // Invalid break
    {
        name: invalid_break_fail0,
        file: "grading/invalid_break_fail0.snek",
        expected: "break",
    },

    // Invalid loop
    {
        name: invalid_loop_fail0,
        file: "grading/invalid_loop_fail0.snek",
        expected: "Invalid",
    },

    // Duplicate parameters in function

    {
        name: duplicate_params,
        file: "self/duplicate_params.snek",
        expected: "",
    },
}
