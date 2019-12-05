extern crate alox;

use alox::ast::Path;
use alox::ir::debug::{Printer, PrintMode};
use alox::parser;
use alox::ir::Compiler;
use std::str::from_utf8_unchecked;

pub fn check_ast(test_name: &str, module: &str, expected_ast: &str) {
    // parse the module and compiler it to ir
    let mut parsed_program = parser::parse(Path::of("test"), test_name.to_string(), module.to_string());

    let mut ast = if let Some(program) = parsed_program {
        format!("{:#?}", program)
    } else {
        panic!("expected ast to exist");
    };

    let mut expected_ast = expected_ast.to_string();
    if expected_ast.ends_with('\n') {
        expected_ast.pop();
    }

    println!("========== Expected ==========");
    println!("{}", expected_ast);
    println!("=========== Actual ===========");
    println!("{}", ast);
    println!("==========");
    assert_eq!(ast, expected_ast);
}

#[test]
pub fn basic_function() {
    check_ast("basic_function", "\
fun test(a: Int32): Int32 {
    return a
}", r#"Program {
    path: Path(
        [
            "test",
        ],
    ),
    file_name: "basic_function",
    imports: [],
    nodes: [
        Function(
            Function {
                name: "test",
                arguments: [
                    (
                        "a",
                        (
                            Path(
                                [],
                            ),
                            "Int32",
                        ),
                    ),
                ],
                return_type: (
                    Path(
                        [],
                    ),
                    "Int32",
                ),
                statements: [
                    Return(
                        Return {
                            expression: VariableReference(
                                VariableReference {
                                    path: None,
                                    name: "a",
                                },
                            ),
                        },
                    ),
                ],
            },
        ),
    ],
}"#);
}

#[test]
pub fn function_call() {
    check_ast("function_call", "\
fun foo(a: Int32): Int32 {
    let b = a
    return foo(b)
}

fun bar(a: Int32): Int32 {
    return foo(a)
}", r#"Program {
    path: Path(
        [
            "test",
        ],
    ),
    file_name: "function_call",
    imports: [],
    nodes: [
        Function(
            Function {
                name: "foo",
                arguments: [
                    (
                        "a",
                        (
                            Path(
                                [],
                            ),
                            "Int32",
                        ),
                    ),
                ],
                return_type: (
                    Path(
                        [],
                    ),
                    "Int32",
                ),
                statements: [
                    VariableDeclaration(
                        VariableDeclaration {
                            name: "b",
                            type_name: None,
                            initial_expression: Some(
                                VariableReference(
                                    VariableReference {
                                        path: None,
                                        name: "a",
                                    },
                                ),
                            ),
                        },
                    ),
                    Return(
                        Return {
                            expression: FunctionCall(
                                FunctionCall {
                                    function: VariableReference(
                                        VariableReference {
                                            path: None,
                                            name: "foo",
                                        },
                                    ),
                                    arguments: [
                                        VariableReference(
                                            VariableReference {
                                                path: None,
                                                name: "b",
                                            },
                                        ),
                                    ],
                                },
                            ),
                        },
                    ),
                ],
            },
        ),
        Function(
            Function {
                name: "bar",
                arguments: [
                    (
                        "a",
                        (
                            Path(
                                [],
                            ),
                            "Int32",
                        ),
                    ),
                ],
                return_type: (
                    Path(
                        [],
                    ),
                    "Int32",
                ),
                statements: [
                    Return(
                        Return {
                            expression: FunctionCall(
                                FunctionCall {
                                    function: VariableReference(
                                        VariableReference {
                                            path: None,
                                            name: "foo",
                                        },
                                    ),
                                    arguments: [
                                        VariableReference(
                                            VariableReference {
                                                path: None,
                                                name: "a",
                                            },
                                        ),
                                    ],
                                },
                            ),
                        },
                    ),
                ],
            },
        ),
    ],
}"#);
}

#[test]
pub fn if_statement() {
    check_ast("if_statement", "\
fun test(a: Int32): Int32 {
    if true {
        return 0
    } else if false {
        return 1
    } else if 2 {
        return 3
    } else {
        return 4
    }
}", r#"Program {
    path: Path(
        [
            "test",
        ],
    ),
    file_name: "if_statement",
    imports: [],
    nodes: [
        Function(
            Function {
                name: "test",
                arguments: [
                    (
                        "a",
                        (
                            Path(
                                [],
                            ),
                            "Int32",
                        ),
                    ),
                ],
                return_type: (
                    Path(
                        [],
                    ),
                    "Int32",
                ),
                statements: [
                    If(
                        IfStatement {
                            condition: BooleanLiteral(
                                BooleanLiteral(
                                    true,
                                ),
                            ),
                            block: [
                                Return(
                                    Return {
                                        expression: IntegerLiteral(
                                            IntegerLiteral(
                                                0,
                                            ),
                                        ),
                                    },
                                ),
                            ],
                            elseif: Some(
                                IfStatement {
                                    condition: BooleanLiteral(
                                        BooleanLiteral(
                                            false,
                                        ),
                                    ),
                                    block: [
                                        Return(
                                            Return {
                                                expression: IntegerLiteral(
                                                    IntegerLiteral(
                                                        1,
                                                    ),
                                                ),
                                            },
                                        ),
                                    ],
                                    elseif: Some(
                                        IfStatement {
                                            condition: IntegerLiteral(
                                                IntegerLiteral(
                                                    2,
                                                ),
                                            ),
                                            block: [
                                                Return(
                                                    Return {
                                                        expression: IntegerLiteral(
                                                            IntegerLiteral(
                                                                3,
                                                            ),
                                                        ),
                                                    },
                                                ),
                                            ],
                                            elseif: Some(
                                                IfStatement {
                                                    condition: BooleanLiteral(
                                                        BooleanLiteral(
                                                            true,
                                                        ),
                                                    ),
                                                    block: [
                                                        Return(
                                                            Return {
                                                                expression: IntegerLiteral(
                                                                    IntegerLiteral(
                                                                        4,
                                                                    ),
                                                                ),
                                                            },
                                                        ),
                                                    ],
                                                    elseif: None,
                                                },
                                            ),
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                ],
            },
        ),
    ],
}"#);
}