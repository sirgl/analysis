package sirgl.java


fun main(args: Array<String>) {
//    println(keywords().joinToString (separator = "\n", transform = { "val ${it}Kw = kw(\"$it\")" }))
//    println(keywords().joinToString (separator = "\n", transform = { "val ${it}Kw = regex(keyword(\"$it\"), JavaTokenNames.${it}Kw)" }))
//    println(operatorToAbbrs().joinToString(separator = "\n") { "    const val ${it.second} = \"${it.first}\"" })
    println(operatorToAbbrs().joinToString(separator = "\n") { "val ${it.second} = punct(\"${it.first}\", \"${it.second}\")" })
}

fun operatorToAbbrs(): List<Pair<String, String>> {
    return operators().map { operator ->
        operator to buildString {
            for ((index, c) in operator.withIndex()) {
                val abbr = charToAbbr[c] ?: throw IllegalArgumentException("$c")
                append(if (index != 0) {
                    abbr[0].toUpperCase() + abbr.substring(1)
                } else {
                    abbr
                })
            }
        }
    }
}

private fun keywords(): List<String> {
    return """
            abstract continue for new switch
    assert default if package synchronized
    boolean do goto private this
    break double implements protected throw
    byte else import public throws
    case enum instanceof return transient
    catch extends int short try
    char final interface static void
    class finally long strictfp volatile
    const float native super while
        """.trimIndent()
        .split(Regex("\\s"))
        .filter { it.isNotEmpty() }
}

private fun operators(): List<String> {
    return """= > < ! ~ ? : ->
== >= <= != && || ++ --
+ - * / & | ^ % << >> >>>
+= -= *= /= &= |= ^= %= <<= >>= >>>=

( ) { } [ ] ; , . ... @ ::
""".trimIndent().trim()
        .split(Regex("\\s"))
        .filter { it.isNotEmpty() }
}

val charToAbbr = mapOf(
    '=' to "eq",
    '>' to "gt",
    '<' to "lt",
    '!' to "excl",
    '~' to "tilde",
    '?' to "question",
    ':' to "colon",
    '&' to "and",
    '|' to "or",
    '%' to "percent",
    '+' to "plus",
    '-' to "minus",
    '^' to "caret",
    '-' to "dash",
    '*' to "asterisk",
    '/' to "div",
    '(' to "lpar",
    ')' to "rpar",
    '{' to "lBrace",
    '}' to "rBrace",
    '[' to "lBracket",
    ']' to "rBracket",
    '.' to "dot",
    ',' to "comma",
    ';' to "semicolon",
    ':' to "colon",
    '@' to "at"
)