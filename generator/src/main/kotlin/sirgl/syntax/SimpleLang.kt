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
            KeywordLeaf("print", "print"),
            KeywordLeaf("fun", "fun"),
            RuleLeafNode("id"),
            RuleLeafNode("num"),
            RuleLeafNode("whitespace"),
            BranchNode("block"),
            BranchNode("args"),
            BranchNode("function"),
            BranchNode("assign_stmt"),
            BranchNode("print_stmt"),
            BranchNode("bin_expr")
    )
}

fun main(args: Array<String>) {
    println(generate(SimpleDescription.nodes, startIndex))
}