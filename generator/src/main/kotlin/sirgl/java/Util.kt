package sirgl.java

fun escapeChar(ch: Char) : String {
    return when (ch) {
        '\n' -> "\\n"
        '\r' -> "\\r"
        '\t' -> "\\t"
        '\"' -> "\\\""
        '\\' -> "\\\\"
        else -> ch.toString()
    }
}

fun escape(str: String) : String {
    val sb = StringBuilder()
    for (c in str) {
        sb.append(escapeChar(c))
    }
    return sb.toString()
}