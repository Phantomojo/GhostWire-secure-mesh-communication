Crate icu\_collections Copy item path

<span class="sub-heading"><a href="../src/icu_collections/lib.rs.html#5-44" class="src">Source</a>
</span>

Expand description

Efficient collections for Unicode data.

This module is published as its own crate
([`icu_collections`](https://docs.rs/icu_collections/latest/icu_collections/))
and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See
the latter for more details on the ICU4X project.

ICU4X
[`CodePointTrie`](codepointtrie/struct.CodePointTrie.html "struct icu_collections::codepointtrie::CodePointTrie")
provides a read-only view of `CodePointTrie` data that is exported from
ICU4C. Detailed information about the design of the data structure can
be found in the documentation for the
[`CodePointTrie`](codepointtrie/struct.CodePointTrie.html "struct icu_collections::codepointtrie::CodePointTrie")
struct.

ICU4X
[`CodePointInversionList`](codepointinvlist/struct.CodePointInversionList.html "struct icu_collections::codepointinvlist::CodePointInversionList")
provides necessary functionality for highly efficient querying of sets
of Unicode characters. It is an implementation of the existing [ICU4C
UnicodeSet
API](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/classicu_1_1UnicodeSet.html).

ICU4X
[`Char16Trie`](char16trie/struct.Char16Trie.html "struct icu_collections::char16trie::Char16Trie")
provides a data structure for a space-efficient and time-efficient
lookup of sequences of 16-bit units (commonly but not necessarily UTF-16
code units) which map to integer values. It is an implementation of the
existing [ICU4C
UCharsTrie](https://unicode-org.github.io/icu-docs/apidoc/released/icu4c/classicu_1_1UCharsTrie.html)
/ [ICU4J
CharsTrie](https://unicode-org.github.io/icu-docs/apidoc/released/icu4j/com/ibm/icu/util/CharsTrie.html)
API.

# Modules<a href="#modules" class="anchor">§</a>

<a href="char16trie/index.html" class="mod"
title="mod icu_collections::char16trie">char16trie</a>  
This module provides a data structure for a space-efficient and
time-efficient lookup of sequences of 16-bit units (commonly but not
necessarily UTF-16 code units) which map to integer values.

<a href="codepointinvlist/index.html" class="mod"
title="mod icu_collections::codepointinvlist">codepointinvlist</a>  
This module provides necessary functionality for highly efficient
querying of sets of Unicode characters.

<a href="codepointinvliststringlist/index.html" class="mod"
title="mod icu_collections::codepointinvliststringlist">codepointinvliststringlist</a>  
This module provides functionality for querying of sets of Unicode code
points and strings.

<a href="codepointtrie/index.html" class="mod"
title="mod icu_collections::codepointtrie">codepointtrie</a>  
This module provides a data structure for an time-efficient lookup of
values associated to code points.
