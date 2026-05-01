Crate strsim Copy item path

<span class="sub-heading"><a href="../src/strsim/lib.rs.html#1-1307" class="src">Source</a>
</span>

Expand description

This library implements string similarity metrics.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.StrSimError.html" class="enum"
title="enum strsim::StrSimError">StrSimError</a>

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.damerau_levenshtein.html" class="fn"
title="fn strsim::damerau_levenshtein">damerau_levenshtein</a>  
Like optimal string alignment, but substrings can be edited an unlimited
number of times, and the triangle inequality holds.

<a href="fn.generic_damerau_levenshtein.html" class="fn"
title="fn strsim::generic_damerau_levenshtein">generic_damerau_levenshtein</a>  
Like optimal string alignment, but substrings can be edited an unlimited
number of times, and the triangle inequality holds.

<a href="fn.generic_hamming.html" class="fn"
title="fn strsim::generic_hamming">generic_hamming</a>  
Calculates the number of positions in the two sequences where the
elements differ. Returns an error if the sequences have different
lengths.

<a href="fn.generic_jaro.html" class="fn"
title="fn strsim::generic_jaro">generic_jaro</a>  
Calculates the Jaro similarity between two sequences. The returned value
is between 0.0 and 1.0 (higher value means more similar).

<a href="fn.generic_jaro_winkler.html" class="fn"
title="fn strsim::generic_jaro_winkler">generic_jaro_winkler</a>  
Like Jaro but gives a boost to sequences that have a common prefix.

<a href="fn.generic_levenshtein.html" class="fn"
title="fn strsim::generic_levenshtein">generic_levenshtein</a>  
Calculates the minimum number of insertions, deletions, and
substitutions required to change one sequence into the other.

<a href="fn.hamming.html" class="fn"
title="fn strsim::hamming">hamming</a>  
Calculates the number of positions in the two strings where the
characters differ. Returns an error if the strings have different
lengths.

<a href="fn.jaro.html" class="fn" title="fn strsim::jaro">jaro</a>  
Calculates the Jaro similarity between two strings. The returned value
is between 0.0 and 1.0 (higher value means more similar).

<a href="fn.jaro_winkler.html" class="fn"
title="fn strsim::jaro_winkler">jaro_winkler</a>  
Like Jaro but gives a boost to strings that have a common prefix.

<a href="fn.levenshtein.html" class="fn"
title="fn strsim::levenshtein">levenshtein</a>  
Calculates the minimum number of insertions, deletions, and
substitutions required to change one string into the other.

<a href="fn.normalized_damerau_levenshtein.html" class="fn"
title="fn strsim::normalized_damerau_levenshtein">normalized_damerau_levenshtein</a>  
Calculates a normalized score of the Damerau–Levenshtein algorithm
between 0.0 and 1.0 (inclusive), where 1.0 means the strings are the
same.

<a href="fn.normalized_levenshtein.html" class="fn"
title="fn strsim::normalized_levenshtein">normalized_levenshtein</a>  
Calculates a normalized score of the Levenshtein algorithm between 0.0
and 1.0 (inclusive), where 1.0 means the strings are the same.

<a href="fn.osa_distance.html" class="fn"
title="fn strsim::osa_distance">osa_distance</a>  
Like Levenshtein but allows for adjacent transpositions. Each substring
can only be edited once.

<a href="fn.sorensen_dice.html" class="fn"
title="fn strsim::sorensen_dice">sorensen_dice</a>  
Calculates a Sørensen-Dice similarity distance using bigrams. See
<https://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient>.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.HammingResult.html" class="type"
title="type strsim::HammingResult">HammingResult</a>
