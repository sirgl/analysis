@file:Suppress("unused", "MemberVisibilityCanBePrivate")

package sirgl.java

import sirgl.*

// JLS ch. 3
object JavaLexerSpec {
    val topLevel = mutableListOf<Token>()

    // helper strings

    private const val nonZeroDigitStr: String = "123456789"
    private const val digitsStr: String = "0$nonZeroDigitStr"
    private const val hexCharsLowercasedStr = "abcdef"
    private const val octalDigitStr = "01234567"
    private const val floatTypeSuffixStr = "fFdD"
    private const val lettersOnlyStr = "abcdefghigklmnopqrstuvwxyz"
    private val lettersStr: String = "_$" + lettersOnlyStr + lettersOnlyStr.toUpperCase()

    // Java lexer spec

    val lineTerminator = or(
        ch('\n'),
        ch('\r'),
        seq("\n\r")
    ).named("Line terminator")

    val whitespace = +or("\r\n\t ").named("Whitespace").token("Whitespace")


    // id

    val letterOrDigit = or(lettersStr + lettersStr.toUpperCase() + digitsStr)

    val letter = or(lettersStr)

    val identifier = +seq(
            letter,
            letterOrDigit.zeroOrMore()
        ).named("Identifer").token("Identifier")


    // keywords
    val abstractKw = kw("abstract")
    val continueKw = kw("continue")
    val forKw = kw("for")
    val newKw = kw("new")
    val switchKw = kw("switch")
    val assertKw = kw("assert")
    val defaultKw = kw("default")
    val ifKw = kw("if")
    val packageKw = kw("package")
    val synchronizedKw = kw("synchronized")
    val booleanKw = kw("boolean")
    val doKw = kw("do")
    val gotoKw = kw("goto")
    val privateKw = kw("private")
    val thisKw = kw("this")
    val breakKw = kw("break")
    val doubleKw = kw("double")
    val implementsKw = kw("implements")
    val protectedKw = kw("protected")
    val throwKw = kw("throw")
    val byteKw = kw("byte")
    val elseKw = kw("else")
    val importKw = kw("import")
    val publicKw = kw("public")
    val throwsKw = kw("throws")
    val caseKw = kw("case")
    val enumKw = kw("enum")
    val instanceofKw = kw("instanceof")
    val returnKw = kw("return")
    val transientKw = kw("transient")
    val catchKw = kw("catch")
    val extendsKw = kw("extends")
    val intKw = kw("int")
    val shortKw = kw("short")
    val tryKw = kw("try")
    val charKw = kw("char")
    val finalKw = kw("final")
    val interfaceKw = kw("interface")
    val staticKw = kw("static")
    val voidKw = kw("void")
    val classKw = kw("class")
    val finallyKw = kw("finally")
    val longKw = kw("long")
    val strictfpKw = kw("strictfp")
    val volatileKw = kw("volatile")
    val constKw = kw("const")
    val floatKw = kw("float")
    val nativeKw = kw("native")
    val superKw = kw("super")
    val whileKw = kw("while")

    // digits
    val integerTypeSuffix = seq("lL")
    val underscores = ch('_').oneOrMore()
    val digitsOrUnderscores = or(digitsStr + "_")
    val digitsAndUnderscores = digitsOrUnderscores.oneOrMore()
    val nonZeroDigit = or(nonZeroDigitStr)

    val digits = or(
        or(digitsStr),
        seq(
            or(digitsStr),
            digitsAndUnderscores.optional(),
            or(digitsStr)
        )
    ).named("Digits")

    // decimal literal

    val decimalNumeral = or(
//        ch('0'),
        seq(
            nonZeroDigit,
            digits.optional()
        )
//        ,
//        seq(
//            nonZeroDigit,
//            underscores,
//            digits
//        )
    ).named("Decimal numeral")

    val decimalIntegerLiteral = seq(
        decimalNumeral,
        integerTypeSuffix.optional()
    ).named("Decimal integer literal")

    // hex literal

    val hexDigit = or(digitsStr + hexCharsLowercasedStr + hexCharsLowercasedStr.toUpperCase())

    val hexDigitOrUnderscore = or(
        ch('_'),
        hexDigit
    )

    val hexDigitAndUnderscore = hexDigitOrUnderscore.oneOrMore()

    val hexDigits = or(
        hexDigit,
        seq(
            hexDigit,
            hexDigitAndUnderscore.optional(),
            hexDigit
        )
    ).named("Hex digits")

    val hexNumeral = or(
        seq(
            seq("0x"),
            hexDigits
        ),
        seq(
            seq("0X"),
            hexDigits
        )
    )

    val hexIntegerLiteral = seq(
        hexNumeral,
        integerTypeSuffix.optional()
    ).named("Hex integer literal")

    // octal integer literal

    val octalDigit = or(octalDigitStr)

    val octalDigitOrUnderscore = or(
        ch('_'),
        octalDigit
    )

    val octalDigitAndUnderscore = octalDigitOrUnderscore.oneOrMore()

    val octalDigits = or(
        octalDigit,
        seq(
            octalDigit,
            octalDigitAndUnderscore.optional(),
            octalDigit
        )
    )

    val octalNumeral = or(
        seq(
            ch('0'),
            octalDigits
        ),
        seq(
            ch('0'),
            underscores,
            octalDigits
        )
    )

    val octalIntegerLiteral = seq(
        octalNumeral,
        integerTypeSuffix.optional()
    ).named("Octal integer literal")

    // Binary integer literal

    val binaryDigit = or("01")

    val binaryDigitOrUnderscore = or(
        ch('_'),
        binaryDigit
    )

    val binaryDigitAndUnderscore = binaryDigitOrUnderscore.oneOrMore()

    val binaryDigits = or(
        binaryDigit,
        seq(
            binaryDigit,
            binaryDigitAndUnderscore.optional(),
            binaryDigit
        )
    )

    val binaryNumeral = or(
        seq(
            or("0b"),
            binaryDigits
        ),
        seq(
            or("0B"),
            binaryDigits
        )
    )


    val binaryIntegerLiteral = seq(
        binaryNumeral,
        integerTypeSuffix.optional()
    ).named("Binary integer literal")

    // numerals finished

    val integerLiteral = +
//    or(
        decimalIntegerLiteral
//    ,
//        hexIntegerLiteral,
//        octalIntegerLiteral,
//        binaryIntegerLiteral
//    )
    .named("Integer literal").token("IntegerLiteral")

    // float literals

    val floatTypeSuffix = or(floatTypeSuffixStr)

    val sign = or("+-")

    val signedInteger = seq(
        sign.optional(),
        digits
    )

    val exponentIndicator = or("eE")

    val exponentPart = seq(
        exponentIndicator,
        signedInteger
    )

    val decimalFloatingPointLiteral = or(
        seq(
            digits,
            ch('.'),
            digits.optional(),
            exponentPart.optional(),
            floatTypeSuffix.optional()
        ),
        seq(
            ch('.'),
            digits,
            exponentPart.optional(),
            floatTypeSuffix.optional()
        ),
        seq(
            digits,
            exponentPart,
            floatTypeSuffix.optional()
        ),
        seq(
            digits,
            exponentPart.optional(),
            floatTypeSuffix
        )
    )


    val binaryExponentIndicator = or("pP")

    val binaryExponent = seq(
        binaryExponentIndicator,
        signedInteger
    )

    val hexSignificand = or(
        seq(
            hexNumeral,
            ch('.').optional()
        ),
        seq(
            seq("0x"),
            hexDigits.optional(),
            ch('.'),
            hexDigits
        ),
        seq(
            seq("0X"),
            hexDigits.optional(),
            ch('.'),
            hexDigits
        )
    )


    val hexadecimalFloatingPointLiteral = seq(
        hexSignificand,
        binaryExponent,
        floatTypeSuffix.optional()
    )

    val floatingPointLiteral = +or(
        decimalFloatingPointLiteral,
        hexadecimalFloatingPointLiteral
    ).named("Floating point literal").token("FloatLiteral")


    // escape sequences

    val zeroToThree = or("0123")

    val octalEscape = or(
        seq(
            ch('\\'),
            octalDigit
        ),
        seq(
            ch('\\'),
            octalDigit,
            octalDigit
        ),
        seq(
            ch('\\'),
            zeroToThree,
            octalDigit,
            octalDigit
        )
    )

    val escapeSequence = or(
        seq("\\b"),
        seq("\\t"),
        seq("\\n"),
        seq("\\f"),
        seq("\\r"),
        seq("\\\""),
        seq("\\'"),
        seq("\\\\'"),
        octalEscape
    )

    // char literal

    val singleCharacter = or("\\'").inverted()

    val characterLiteral = +or(
            seq(
                ch('\''),
                singleCharacter,
                ch('\'')
            ),
            seq(
                ch('\''),
                escapeSequence,
                ch('\'')
            )
        ).named("Character literal").token("CharLiteral")

    // string literal

    val stringCharacter = or(
            or("\\\"").inverted(),
            escapeSequence
        )

    val stringLiteral = +seq(
            CharNode('"'),
            stringCharacter.oneOrMore(),
            CharNode('"')
        ).named("String literal")
        .token("StringLiteral")

    // Punctuators

    val eq = punct("=", "eq")
    val gt = punct(">", "gt")
    val lt = punct("<", "lt")
    val excl = punct("!", "excl")
    val tilde = punct("~", "tilde")
    val question = punct("?", "question")
    val colon = punct(":", "colon")
    val dashGt = punct("->", "dashGt")
    val eqEq = punct("==", "eqEq")
    val gtEq = punct(">=", "gtEq")
    val ltEq = punct("<=", "ltEq")
    val exclEq = punct("!=", "exclEq")
    val andAnd = punct("&&", "andAnd")
    val orOr = punct("||", "orOr")
    val plusPlus = punct("++", "plusPlus")
    val dashDash = punct("--", "dashDash")
    val plus = punct("+", "plus")
    val dash = punct("-", "dash")
    val asterisk = punct("*", "asterisk")
    val div = punct("/", "div")
    val and = punct("&", "and")
    val or = punct("|", "or")
    val caret = punct("^", "caret")
    val percent = punct("%", "percent")
    val ltLt = punct("<<", "ltLt")
    val gtGt = punct(">>", "gtGt")
    val gtGtGt = punct(">>>", "gtGtGt")
    val plusEq = punct("+=", "plusEq")
    val dashEq = punct("-=", "dashEq")
    val asteriskEq = punct("*=", "asteriskEq")
    val divEq = punct("/=", "divEq")
    val andEq = punct("&=", "andEq")
    val orEq = punct("|=", "orEq")
    val caretEq = punct("^=", "caretEq")
    val percentEq = punct("%=", "percentEq")
    val ltLtEq = punct("<<=", "ltLtEq")
    val gtGtEq = punct(">>=", "gtGtEq")
    val gtGtGtEq = punct(">>>=", "gtGtGtEq")
    val lpar = punct("(", "lpar")
    val rpar = punct(")", "rpar")
    val lBrace = punct("{", "lBrace")
    val rBrace = punct("}", "rBrace")
    val lBracket = punct("[", "lBracket")
    val rBracket = punct("]", "rBracket")
    val semicolon = punct(";", "semicolon")
    val comma = punct(",", "comma")
    val dot = punct(".", "dot")
    val dotDotDot = punct("...", "dotDotDot")
    val at = punct("@", "at")
    val colonColon = punct("::", "colonColon")

    // service functions

    private operator fun Token.unaryPlus(): Token {
        topLevel += this
        return this
    }

    private fun RegexNode.token(name: String): Token = Token(this, name.titleCase())

    private fun kw(text: String): Token = +seq(text).named("Keyword $text").token("${text}Kw")
    // TODO name for token
    private fun punct(text: String, name: String): Token = +seq(text).named(text).token(name)

    private fun String.titleCase(): String = this[0].toUpperCase() + this.substring(1)
}

