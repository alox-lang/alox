package dev.alox.compiler

import dev.alox.compiler.ast.AstModule
import dev.alox.compiler.ast.Path
import dev.alox.compiler.ir.PrettyPrinter
import dev.alox.compiler.ir.Translator
import dev.alox.compiler.parser.AstParser

fun main(args: Array<String>) {
    println("Alox compiler")
    /*
    main module:
    struct Box {
        let x: Int32
    }

    fun foo(box: Box): Int32 {
        let x: Int32 = 1
        return 1
        return box.x
    }
     */
    val mainModule = AstModule(
        Path(listOf("alox")), "main", listOf(
            AstModule.Declaration.Struct(
                "Box",
                AstModule.Declaration.Struct.Kind.STRUCT,
                listOf(), // type parameters
                listOf(
                    AstModule.Declaration.Struct.Field(
                        "x",
                        AstModule.TypeName(Path.empty, "Int32", listOf())
                    )
                ), // fields
                listOf() // functions
            ),
            AstModule.Declaration.Function(
                "foo",
                AstModule.Declaration.Function.Kind.FUNCTION,
                listOf(), // type parameters
                listOf(
                    AstModule.Declaration.Function.Argument(
                        "box",
                        AstModule.TypeName(Path.empty, "Box", listOf())
                    )
                ), // arguments
                listOf(
                    AstModule.Statement.Return(
                        AstModule.Expression.GetField(
                            AstModule.Expression.VariableReference(
                                Path.empty,
                                "box"
                            ), "x"
                        )
                    )
                ), // statements
                AstModule.TypeName(Path.empty, "Int32", listOf())
            )
        )
    )

    val mainIrModule = Translator(mainModule).generateModule()

    PrettyPrinter(mainIrModule)

    /*

     */
    val parsedModule = AstParser.parseModule(Path(listOf("alox")), "parsed", """
struct Box {
    let x : Int32
}

fun foo(box: Box): Int32 {
    let x: Int32 = 1
    return box
}

actor A {
    let state : Int32

    behave ping(n: Int32, b: B) {
        (b.pong)(n, this)
    }
}
    """.trimIndent())

    val parsedIrModule = Translator(parsedModule).generateModule()

    PrettyPrinter(parsedIrModule)
}
