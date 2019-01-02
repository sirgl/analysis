package sirgl.syntax

object SimpleDescription : SyntaxDescription {
    override val nodes: List<SyntaxNode> = listOf(
            TextLeaf("{", "lbrace"),
            TextLeaf("}", "rbrace"),
            TextLeaf("(", "lpar"),
            TextLeaf(")", "rpar"),
            TextLeaf(",", "comma"),
            TextLeaf("+", "plus"),
            TextLeaf("-", "minus"),
            TextLeaf("*", "mul"),
            TextLeaf("/", "div"),
            TextLeaf("=", "eq"),
            TextLeaf(";", "semi"),
            KeywordLeaf("fun", "fun"),
            RuleLeafNode("id"),
            RuleLeafNode("num"),
            RuleLeafNode("whitespace"),
            BranchNode("args"),
            BranchNode("function")
    )
}

fun main(args: Array<String>) {
    println(generate(SimpleDescription.nodes, startIndex))
}