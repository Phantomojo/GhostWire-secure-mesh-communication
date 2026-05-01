Crate unicode\_width Copy item path

<span class="sub-heading"><a href="../src/unicode_width/lib.rs.html#11-258" class="src">Source</a>
</span>

Expand description

Determine displayed width of `char` and `str` types according to
[Unicode Standard Annex \#11](http://www.unicode.org/reports/tr11/) and
other portions of the Unicode standard. See the [Rules for determining
width](#rules-for-determining-width) section for the exact rules.

This crate is `#![no_std]`.

``` {.rust .rust-example-rendered}
use unicode_width::UnicodeWidthStr;

let teststr = "Ｈｅｌｌｏ, ｗｏｒｌｄ!";
let width = UnicodeWidthStr::width(teststr);
println!("{}", teststr);
println!("The above string is {} columns wide.", width);
```

# <a href="#cjk-feature-flag" class="doc-anchor">§</a>`"cjk"` feature flag

This crate has one Cargo feature flag, `"cjk"` (enabled by default). It
enables the
[`UnicodeWidthChar::width_cjk`](trait.UnicodeWidthChar.html#tymethod.width_cjk "method unicode_width::UnicodeWidthChar::width_cjk")
and
[`UnicodeWidthStr::width_cjk`](trait.UnicodeWidthStr.html#tymethod.width_cjk "method unicode_width::UnicodeWidthStr::width_cjk"),
which perform an alternate width calculation more suited to CJK
contexts. The flag also unseals the
[`UnicodeWidthChar`](trait.UnicodeWidthChar.html "trait unicode_width::UnicodeWidthChar")
and
[`UnicodeWidthStr`](trait.UnicodeWidthStr.html "trait unicode_width::UnicodeWidthStr")
traits.

Disabling the flag (with `no_default_features` in `Cargo.toml`) will
reduce the amount of static data needed by the crate.

``` {.rust .rust-example-rendered}
use unicode_width::UnicodeWidthStr;

let teststr = "“𘀀”";
assert_eq!(teststr.width(), 4);

#[cfg(feature = "cjk")]
assert_eq!(teststr.width_cjk(), 6);
```

# <a href="#rules-for-determining-width" class="doc-anchor">§</a>Rules for determining width

This crate currently uses the following rules to determine the width of
a character or string, in order of decreasing precedence. These may be
tweaked in the future.

1.  In the following cases, the width of a string differs from the sum
    of the widths of its constituent characters:
    -   The sequence `"\r\n"` has width 1.
    -   Emoji-specific ligatures:
        -   Well-formed, fully-qualified [emoji ZWJ
            sequences](https://www.unicode.org/reports/tr51/#def_emoji_sequence)
            have width 2.
        -   [Emoji modifier
            sequences](https://www.unicode.org/reports/tr51/#def_emoji_modifier_sequence)
            have width 2.
        -   [Emoji presentation
            sequences](https://unicode.org/reports/tr51/#def_emoji_presentation_sequence)
            have width 2.
        -   Outside of an East Asian context, [text presentation
            sequences](https://unicode.org/reports/tr51/#def_text_presentation_sequence)
            have width 1 if their base character:
            -   Has the
                [`Emoji_Presentation`](https://unicode.org/reports/tr51/#def_emoji_presentation)
                property, and
            -   Is not in the [Enclosed Ideographic
                Supplement](https://unicode.org/charts/nameslist/n_1F200.html)
                block.
    -   Script-specific ligatures:
        -   For all the following ligatures, the insertion of any number
            of
            [default-ignorable](https://www.unicode.org/versions/Unicode15.0.0/ch05.pdf#G40095)
            [combining
            marks](https://www.unicode.org/versions/Unicode15.0.0/ch03.pdf#G30602)
            anywhere in the sequence will not change the total width. In
            addition, for all non-Arabic ligatures, the insertion of any
            number of [`'\u{200D}'` ZERO WIDTH
            JOINER](https://www.unicode.org/versions/Unicode15.0.0/ch23.pdf#G23126)s
            will not affect the width.
        -   **[Arabic](https://www.unicode.org/versions/Unicode15.0.0/ch09.pdf#G7480)**:
            A character sequence consisting of one character with
            [`Joining_Group`](https://www.unicode.org/versions/Unicode14.0.0/ch09.pdf#G36862)`=Lam`,
            followed by any number of characters with
            [`Joining_Type`](http://www.unicode.org/versions/Unicode15.0.0/ch09.pdf#G50009)`=Transparent`,
            followed by one character with
            [`Joining_Group`](https://www.unicode.org/versions/Unicode14.0.0/ch09.pdf#G36862)`=Alef`,
            has total width 1. For example: `لا`‎, `لآ`‎, `ڸا`‎, `لٟٞأ`
        -   **[Buginese](https://www.unicode.org/versions/Unicode15.0.0/ch17.pdf#G26743)**:
            `"\u{1A15}\u{1A17}\u{200D}\u{1A10}"` (&lt;a, -i&gt; ya,
            `ᨕᨗ‍ᨐ`) has total width 1.
        -   **[Hebrew](https://www.unicode.org/versions/Unicode15.0.0/ch09.pdf#G6528)**:
            `"א\u{200D}ל"` (Alef-Lamed, `א‍ל`) has total width 1.
        -   **[Khmer](https://www.unicode.org/versions/Unicode15.0.0/ch16.pdf#G64642)**:
            Coeng signs consisting of `'\u{17D2}'` followed by a
            character in
            `'\u{1780}'..='\u{1782}' | '\u{1784}'..='\u{1787}' | '\u{1789}'..='\u{178C}' | '\u{178E}'..='\u{1793}' | '\u{1795}'..='\u{1798}' | '\u{179B}'..='\u{179D}' | '\u{17A0}' | '\u{17A2}' | '\u{17A7}' | '\u{17AB}'..='\u{17AC}' | '\u{17AF}'`
            have width 0.
        -   **[Lisu](https://www.unicode.org/versions/Unicode15.0.0/ch18.pdf#G44587)**:
            Tone letter combinations consisting of a character in the
            range `'\u{A4F8}'..='\u{A4FB}'` followed by a character in
            the range `'\u{A4FC}'..='\u{A4FD}'` have width 1. For
            example: `ꓹꓼ`
        -   **[Old
            Turkic](https://www.unicode.org/versions/Unicode15.0.0/ch14.pdf#G41975)**:
            `"\u{10C32}\u{200D}\u{10C03}"` (`𐰲‍𐰃`) has total width 1.
        -   **[Tifinagh](http://www.unicode.org/versions/Unicode15.0.0/ch19.pdf#G43184)**:
            A sequence of a Tifinagh consonant in the range
            `'\u{2D31}'..='\u{2D65}' | '\u{2D6F}'`, followed by either
            [`'\u{2D7F}'` TIFINAGH CONSONANT
            JOINER](https://util.unicode.org/UnicodeJsps/character.jsp?a=2D7F)
            or `'\u{200D}'`, followed by another Tifinangh consonant,
            has total width 1. For example: `ⵏ⵿ⴾ`
    -   In an East Asian context only, `<`, `=`, or `>` have width 2
        when followed by [`'\u{0338}'` COMBINING LONG SOLIDUS
        OVERLAY](https://util.unicode.org/UnicodeJsps/character.jsp?a=0338).
        The two characters may be separated by any number of characters
        whose canonical decompositions consist only of characters
        meeting one of the following requirements:
        -   Has
            [`Canonical_Combining_Class`](https://www.unicode.org/versions/Unicode15.0.0/ch03.pdf#G50313)
            greater than 1, or
        -   Is a
            [default-ignorable](https://www.unicode.org/versions/Unicode15.0.0/ch05.pdf#G40095)
            [combining
            mark](https://www.unicode.org/versions/Unicode15.0.0/ch03.pdf#G30602).
2.  In all other cases, the width of the string equals the sum of its
    character widths:
    1.  [`'\u{2D7F}'` TIFINAGH CONSONANT
        JOINER](https://util.unicode.org/UnicodeJsps/character.jsp?a=2D7F)
        has width 1 (outside of the ligatures described previously).
    2.  [`'\u{115F}'` HANGUL CHOSEONG
        FILLER](https://util.unicode.org/UnicodeJsps/character.jsp?a=115F)
        and [`'\u{17A4}'` KHMER INDEPENDENT VOWEL
        QAA](https://util.unicode.org/UnicodeJsps/character.jsp?a=17A4)
        have width 2.
    3.  [`'\u{17D8}'` KHMER SIGN
        BEYYAL](https://util.unicode.org/UnicodeJsps/character.jsp?a=17D8)
        has width 3.
    4.  The following have width 0:
        -   [Characters](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BDefault_Ignorable_Code_Point%7D)
            with the
            [`Default_Ignorable_Code_Point`](https://www.unicode.org/versions/Unicode15.0.0/ch05.pdf#G40095)
            property.
        -   [Characters](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BGrapheme_Extend%7D)
            with the
            [`Grapheme_Extend`](https://www.unicode.org/versions/Unicode15.0.0/ch03.pdf#G52443)
            property.
        -   The following 8 characters, all of which have NFD
            decompositions consisting of two
            [`Grapheme_Extend`](https://www.unicode.org/versions/Unicode15.0.0/ch03.pdf#G52443)
            characters:
            -   [`'\u{0CC0}'` KANNADA VOWEL SIGN
                II](https://util.unicode.org/UnicodeJsps/character.jsp?a=0CC0),
            -   [`'\u{0CC7}'` KANNADA VOWEL SIGN
                EE](https://util.unicode.org/UnicodeJsps/character.jsp?a=0CC7),
            -   [`'\u{0CC8}'` KANNADA VOWEL SIGN
                AI](https://util.unicode.org/UnicodeJsps/character.jsp?a=0CC8),
            -   [`'\u{0CCA}'` KANNADA VOWEL SIGN
                O](https://util.unicode.org/UnicodeJsps/character.jsp?a=0CCA),
            -   [`'\u{0CCB}'` KANNADA VOWEL SIGN
                OO](https://util.unicode.org/UnicodeJsps/character.jsp?a=0CCB),
            -   [`'\u{1B3B}'` BALINESE VOWEL SIGN RA REPA
                TEDUNG](https://util.unicode.org/UnicodeJsps/character.jsp?a=1B3B),
            -   [`'\u{1B3D}'` BALINESE VOWEL SIGN LA LENGA
                TEDUNG](https://util.unicode.org/UnicodeJsps/character.jsp?a=1B3D),
                and
            -   [`'\u{1B43}'` BALINESE VOWEL SIGN PEPET
                TEDUNG](https://util.unicode.org/UnicodeJsps/character.jsp?a=1B43).
        -   [Characters](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BHangul_Syllable_Type%3DV%7D%5Cp%7BHangul_Syllable_Type%3DT%7D)
            with a
            [`Hangul_Syllable_Type`](https://www.unicode.org/versions/Unicode15.0.0/ch03.pdf#G45593)
            of `Vowel_Jamo` (`V`) or `Trailing_Jamo` (`T`).
        -   The following
            [`Prepended_Concatenation_Mark`](https://www.unicode.org/versions/Unicode15.0.0/ch23.pdf#G37908)s:
            -   [`'\u{0605}'` NUMBER MARK
                ABOVE](https://util.unicode.org/UnicodeJsps/character.jsp?a=0605),
            -   [`'\u{070F}'` SYRIAC ABBREVIATION
                MARK](https://util.unicode.org/UnicodeJsps/character.jsp?a=070F),
            -   [`'\u{0890}'` POUND MARK
                ABOVE](https://util.unicode.org/UnicodeJsps/character.jsp?a=0890),
            -   [`'\u{0891}'` PIASTRE MARK
                ABOVE](https://util.unicode.org/UnicodeJsps/character.jsp?a=0891),
                and
            -   [`'\u{08E2}'` DISPUTED END OF
                AYAH](https://util.unicode.org/UnicodeJsps/character.jsp?a=08E2).
        -   [Characters](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BGrapheme_Cluster_Break%3DPrepend%7D-%5Cp%7BPrepended_Concatenation_Mark%7D)
            with the
            [`Grapheme_Extend=Prepend`](https://www.unicode.org/reports/tr29/#Prepend)
            property, that are not also
            [`Prepended_Concatenation_Mark`](https://www.unicode.org/versions/Unicode15.0.0/ch23.pdf#G37908)s.
        -   [`'\u{A8FA}'` DEVANAGARI
            CARET](https://util.unicode.org/UnicodeJsps/character.jsp?a=A8FA).
    5.  [Characters](https://util.unicode.org/UnicodeJsps/list-unicodeset.jsp?a=%5Cp%7BEast_Asian_Width%3DF%7D%5Cp%7BEast_Asian_Width%3DW%7D)
        with an
        [`East_Asian_Width`](https://www.unicode.org/reports/tr11/#ED1)
        of [`Fullwidth`](https://www.unicode.org/reports/tr11/#ED2) or
        [`Wide`](https://www.unicode.org/reports/tr11/#ED4) have width
        2.
    6.  Characters fulfilling all of the following conditions have width
        2 in an East Asian context, and width 1 otherwise:
        -   Has an
            [`East_Asian_Width`](https://www.unicode.org/reports/tr11/#ED1)
            of [`Ambiguous`](https://www.unicode.org/reports/tr11/#ED6),
            or has a canonical decomposition to an
            [`Ambiguous`](https://www.unicode.org/reports/tr11/#ED6)
            character followed by [`'\u{0338}'` COMBINING LONG SOLIDUS
            OVERLAY](https://util.unicode.org/UnicodeJsps/character.jsp?a=0338),
            or is [`'\u{0387}'` GREEK ANO
            TELEIA](https://util.unicode.org/UnicodeJsps/character.jsp?a=0387),
            and
        -   Does not have a
            [`General_Category`](https://www.unicode.org/versions/Unicode15.0.0/ch04.pdf#G124142)
            of `Letter` or `Modifier_Symbol`.
    7.  All other characters have width 1.

## <a href="#canonical-equivalence" class="doc-anchor">§</a>Canonical equivalence

Canonically equivalent strings are assigned the same width (CJK and
non-CJK).

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.UNICODE_VERSION.html" class="constant"
title="constant unicode_width::UNICODE_VERSION">UNICODE_VERSION</a>  
The version of [Unicode](http://www.unicode.org/) that this version of
unicode-width is based on.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.UnicodeWidthChar.html" class="trait"
title="trait unicode_width::UnicodeWidthChar">UnicodeWidthChar</a>  
Methods for determining displayed width of Unicode characters.

<a href="trait.UnicodeWidthStr.html" class="trait"
title="trait unicode_width::UnicodeWidthStr">UnicodeWidthStr</a>  
Methods for determining displayed width of Unicode strings.
