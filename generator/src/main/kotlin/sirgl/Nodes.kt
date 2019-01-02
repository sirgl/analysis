package sirgl

import sirgl.java.escapeChar


sealed class RegexNode {
    var name: String? = null

    abstract override fun toString(): String
}

class CharNode(val ch: Char) : RegexNode() {
    override fun toString(): String = escapeChar(ch)
}

fun ch(ch: Char) : CharNode = CharNode(ch)

class ChoiceNode(val variants: List<RegexNode>) : RegexNode() {
    override fun toString(): String {
        if (variants.all { it is CharNode }) {
            val sortedChars = variants.map { (it as CharNode).ch }.sorted()
            val groups = mutableListOf<Group>()
            var isInGroup = false
            for ((index, current) in sortedChars.withIndex()) {
                val prev = sortedChars.getOrNull(index - 1)
                if (prev == null) {
                    isInGroup = true
                    groups.add(Group(current))

                } else {
                    if (prev == current - 1) {
                        if (isInGroup) {
                            groups.last().end = current
                        } else {
                            groups.add(Group(current))
                            isInGroup = true
                        }
                    } else {
                        groups.add(Group(current))
                        isInGroup = true

                    }
                }
            }
            return groups.joinToString("", "[", "]")
        }
        val s = "(" +
                "[([1-9]([[0-9]|([0-9](([0-9_])+)?[0-9])])?)](lL)?" +
                ")"
        return variants.joinToString("|", "[", "]")
    }

    private class Group (val start: Char, var end: Char = start) {
        override fun toString(): String {
            return if (start == end) {
                escapeChar(start)
            } else {
                "${escapeChar(start)}-${escapeChar(end)}"
            }
        }
    }
}

fun or(vararg variants: RegexNode) : ChoiceNode = ChoiceNode(variants.toList())

fun or(text: String) : ChoiceNode = ChoiceNode(text.map { ch(it) })


class OneOrMoreNode(val node: RegexNode) : RegexNode() {
    override fun toString(): String = "($node)+"
}

fun RegexNode.oneOrMore() : OneOrMoreNode = OneOrMoreNode(this)

class ZeroOrMoreNode(val node: RegexNode) : RegexNode() {
    override fun toString(): String = "($node)*"
}

fun RegexNode.zeroOrMore() : ZeroOrMoreNode = ZeroOrMoreNode(this)

class SeqNode(val nodes: List<RegexNode>) : RegexNode() {
    override fun toString(): String {
        return when {
            isPlain() -> nodes.joinToString("")
            else -> nodes.joinToString("", "(", ")")
        }
    }

    fun isPlain() = nodes.size == 1 || nodes.all { it is CharNode }
}

fun seq(vararg nodes: RegexNode): SeqNode = SeqNode(nodes.toList())

fun seq(text: String): SeqNode = SeqNode(text.map { ch(it) })

fun RegexNode.named(name: String) : RegexNode {
    this.name = name
    return this
}

class OptionalNode(val node: RegexNode) : RegexNode() {
    override fun toString(): String = "($node)?"
}

fun RegexNode.optional() : RegexNode = OptionalNode(this)

class InvertedNode(val node: RegexNode) : RegexNode() {
    override fun toString(): String = "^($node)"

}

fun RegexNode.inverted(): RegexNode = InvertedNode(this)