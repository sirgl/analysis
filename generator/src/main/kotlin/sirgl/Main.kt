package sirgl

import sirgl.java.JavaLexerSpec
import sirgl.java.Token

fun main(args: Array<String>) {
    println(JavaLexerSpec.topLevel.joinToString(",\n") { generateToken(it) })
}

fun generateToken(token: Token) : String {
    val regex = token.regex
    return if (regex is SeqNode && regex.isPlain()) {
        "    #[token = \"$regex\" ]\n" +
                "    ${token.name}"
    } else {
        "    #[regex = \"$regex\" ]\n" +
                "    ${token.name}"
    }
}