Crate wildmatch Copy item path

<span class="sub-heading"><a href="../src/wildmatch/lib.rs.html#1-502" class="src">Source</a>
</span>

Expand description

Match strings against a simple wildcard pattern. Tests a wildcard
pattern `p` against an input string `s`. Returns true only when `p`
matches the entirety of `s`.

See also the example described on
[wikipedia](https://en.wikipedia.org/wiki/Matching_wildcards) for
matching wildcards.

No escape characters are defined.

-   `?` matches exactly one occurrence of any character.
-   `*` matches arbitrary many (including zero) occurrences of any
    character.

Examples matching wildcards:

``` {.rust .rust-example-rendered}
assert!(WildMatch::new("cat").matches("cat"));
assert!(WildMatch::new("*cat*").matches("dog_cat_dog"));
assert!(WildMatch::new("c?t").matches("cat"));
assert!(WildMatch::new("c?t").matches("cot"));
```

Examples not matching wildcards:

``` {.rust .rust-example-rendered}
assert!(!WildMatch::new("dog").matches("cat"));
assert!(!WildMatch::new("*d").matches("cat"));
assert!(!WildMatch::new("????").matches("cat"));
assert!(!WildMatch::new("?").matches("cat"));
```

You can specify custom `char` values for the single and multi-character
wildcards. For example, to use `%` as the multi-character wildcard and
`_` as the single-character wildcard:

``` {.rust .rust-example-rendered}
assert!(WildMatchPattern::<'%', '_'>::new("%cat%").matches("dog_cat_dog"));
```

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.WildMatchPattern.html" class="struct"
title="struct wildmatch::WildMatchPattern">WildMatchPattern</a>  
Wildcard matcher used to match strings.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.WildMatch.html" class="type"
title="type wildmatch::WildMatch">WildMatch</a>  
A wildcard matcher using `*` as the multi-character wildcard and `?` as
the single-character wildcard.
