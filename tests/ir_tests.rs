extern crate alox;

use alox::ast::Path;
use alox::ir::debug::{Printer, PrintMode};
use alox::parser;
use alox::ir::Compiler;
use alox::ir::pass::{DeadBranchRemovalPass, Pass};

pub fn check_ir(test_name: &str, code: &str, expected_ir: &str) {
    // parse the module and compiler it to ir
    let mut parsed_program = parser::parse(Path::of("test"), test_name.to_string(), code.to_string());
    let compiler = Compiler::new();

    let module = compiler.generate_ir(parsed_program.unwrap());
    let pass = DeadBranchRemovalPass {};
    pass.pass(&module);
    compiler.add_module(module);

    // print the module and store it in the buffer
    let mut printer = Printer::new(PrintMode::Buffer);
    for module in compiler.modules.read().unwrap().iter() {
        printer.print_module(module);
    }

    // remove trailing new lines
    if printer.buffer.ends_with('\n') {
        printer.buffer.pop();
    }
    let mut expected_ir = expected_ir.to_string();
    if expected_ir.ends_with('\n') {
        expected_ir.pop();
    }

    println!("========== Expected ==========");
    println!("{}", expected_ir);
    println!("=========== Actual ===========");
    println!("{}", printer.buffer);
    println!("==========");
    assert_eq!(printer.buffer, expected_ir);
}

#[test]
pub fn basic_function() {
    check_ir("basic_function", "\
fun test(a: Int32): Int32 {
    return a
}", "\
; Module: test::basic_function
fun @test(%a: Int32) -> Int32:
  block#0:
    %0 : Int32 = param %a
    ret %0");
}

#[test]
pub fn function_call() {
    check_ir("function_call", "\
fun test(a: Int32): Int32 {
    return a
}

fun bar(a: Int32): Int32 {
    return test(a)
}", "\
; Module: test::function_call
fun @test(%a: Int32) -> Int32:
  block#0:
    %0 : Int32 = param %a
    ret %0
fun @bar(%a: Int32) -> Int32:
  block#0:
    %0 : Int32 -> Int32 = @test::function_call::test
    %1 : Int32 = param %a
    %2 : Int32 = %0(%1)
    ret %2");
}

#[test]
pub fn nested_expression() {
    check_ir("nested_expression", "\
fun foo(c: Int32): Int32 {
    return c
}

fun bar(g: Int32): Int32 {
    return foo(foo(foo(foo(foo(g)))))
}
", "\
; Module: test::nested_expression
fun @foo(%c: Int32) -> Int32:
  block#0:
    %0 : Int32 = param %c
    ret %0
fun @bar(%g: Int32) -> Int32:
  block#0:
    %0 : Int32 -> Int32 = @test::nested_expression::foo
    %1 : Int32 -> Int32 = @test::nested_expression::foo
    %2 : Int32 -> Int32 = @test::nested_expression::foo
    %3 : Int32 -> Int32 = @test::nested_expression::foo
    %4 : Int32 -> Int32 = @test::nested_expression::foo
    %5 : Int32 = param %g
    %6 : Int32 = %4(%5)
    %7 : Int32 = %3(%6)
    %8 : Int32 = %2(%7)
    %9 : Int32 = %1(%8)
    %10 : Int32 = %0(%9)
    ret %10
")
}

#[test]
pub fn fields_in_struct() {
    check_ir("fields_in_struct", "\
struct X {
    let x: Int32
    let y: Float32
    let b: Bool
}
", "\
; Module: test::fields_in_struct
struct X:
  let x: Int32
  let y: Float32
  let b: Bool
")
}

#[test]
pub fn fields_in_actor() {
    check_ir("fields_in_actor", "\
actor A {
    let x: Int32
    let y: Float32
    let b: Bool
}
", "\
; Module: test::fields_in_actor
actor A:
  let x: Int32
  let y: Float32
  let b: Bool
")
}

#[test]
pub fn methods_in_struct() {
    check_ir("method_in_struct", "\
struct X {
    let x: Int32
    let y: Float32
    let b: Bool

    fun fooX(a: Int32): Int32 {
        return a
    }
}
", "\
; Module: test::method_in_struct
struct X:
  let x: Int32
  let y: Float32
  let b: Bool
  fun @fooX(%a: Int32) -> Int32:
    block#0:
      %0 : Int32 = param %a
      ret %0
")
}

#[test]
pub fn method_in_actor() {
    check_ir("method_in_actor", "\
actor A {
    let x: Int32
    let y: Float32
    let b: Bool

    fun fooA(a: Int32): Int32 {
        return a
    }
}
", "\
; Module: test::method_in_actor
actor A:
  let x: Int32
  let y: Float32
  let b: Bool
  fun @fooA(%a: Int32) -> Int32:
    block#0:
      %0 : Int32 = param %a
      ret %0
")
}

#[test]
pub fn void_function() {
    check_ir("void_function", "\
fun test(a: Int32) {
}", "\
; Module: test::void_function
fun @test(%a: Int32) -> Void:
");
}

#[test]
pub fn if_statement() {
    check_ir("if_statement", "\
fun test(a: Int32): Int32 {
    if true {
        return a
    }
    return 1234
}", "\
; Module: test::if_statement
fun @test(%a: Int32) -> Int32:
  block#0:
    %0 : Bool = true
    jump block#1
  block#1:
    %2 : Int32 = param %a
    ret %2");
}

#[test]
pub fn if_else_statement() {
    check_ir("if_else_statement", "\
fun test(a: Bool): Int32 {
    if a {
        return 1
    } else {
        return 0
    }
}", "\
; Module: test::if_else_statement
fun @test(%a: Bool) -> Int32:
  block#0:
    %0 : Bool = param %a
    branch %0 block#1 block#2
  block#1:
    %2 : ComptimeInt = 1
    ret %2
  block#2:
    %4 : Bool = true
    jump block#3
  block#3:
    %6 : ComptimeInt = 0
    ret %6");
}

#[test]
pub fn if_elif_else_statement() {
    check_ir("if_elif_else_statement", "\
fun test(a: Bool): Int32 {
    if a {
        return 1
    } else if false {
        return 2
    } else {
        return 3
    }
}", "\
; Module: test::if_elif_else_statement
fun @test(%a: Bool) -> Int32:
  block#0:
    %0 : Bool = param %a
    branch %0 block#1 block#2
  block#1:
    %2 : ComptimeInt = 1
    ret %2
  block#2:
    %4 : Bool = false
    jump block#3
  block#3:
    %6 : Bool = true
    jump block#4
  block#4:
    %8 : ComptimeInt = 3
    ret %8");
}

#[test]
pub fn if_many_elif_statement() {
    check_ir("if_many_elif_statement", "\
fun foo(a: Bool, b: Bool, c: Bool, d: Bool, e: Bool): Int32 {
    if a {
        return 1
    } else if b {
        return 2
    } else if c {
        return 3
    } else if d {
        return 4
    } else if e {
        return 5
    } else {
        return 6
    }
}", "\
; Module: test::if_many_elif_statement
fun @foo(%a: Bool, %b: Bool, %c: Bool, %d: Bool, %e: Bool) -> Int32:
  block#0:
    %0 : Bool = param %a
    branch %0 block#1 block#2
  block#1:
    %2 : ComptimeInt = 1
    ret %2
  block#2:
    %4 : Bool = param %b
    branch %4 block#3 block#4
  block#3:
    %6 : ComptimeInt = 2
    ret %6
  block#4:
    %8 : Bool = param %c
    branch %8 block#5 block#6
  block#5:
    %10 : ComptimeInt = 3
    ret %10
  block#6:
    %12 : Bool = param %d
    branch %12 block#7 block#8
  block#7:
    %14 : ComptimeInt = 4
    ret %14
  block#8:
    %16 : Bool = param %e
    branch %16 block#9 block#10
  block#9:
    %18 : ComptimeInt = 5
    ret %18
  block#10:
    %20 : Bool = true
    jump block#11
  block#11:
    %22 : ComptimeInt = 6
    ret %22");
}

#[test]
pub fn if_in_if_statement() {
    check_ir("if_in_if_statement", "\
fun foo(a: Bool, b: Bool): Int32 {
    if a {
        if b {
            return 1
        } else {
            return 2
        }
    } else {
        return 3
    }
}", "\
; Module: test::if_in_if_statement
fun @foo(%a: Bool, %b: Bool) -> Int32:
  block#0:
    %0 : Bool = param %a
    branch %0 block#1 block#5
  block#1:
    %2 : Bool = param %b
    branch %2 block#2 block#3
  block#2:
    %4 : ComptimeInt = 1
    ret %4
  block#3:
    %6 : Bool = true
    jump block#4
  block#4:
    %8 : ComptimeInt = 2
    ret %8
  block#5:
    %10 : Bool = true
    jump block#6
  block#6:
    %12 : ComptimeInt = 3
    ret %12");
}
