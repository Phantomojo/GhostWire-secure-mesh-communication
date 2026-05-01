Crate matchit Copy item path

<span class="sub-heading"><a href="../src/matchit/lib.rs.html#1-123" class="src">Source</a>
</span>

Expand description

# <a href="#matchit" class="doc-anchor">§</a>`matchit`

[![Documentation](https://img.shields.io/badge/docs-0.7.3-4d76ae?style=for-the-badge)](https://docs.rs/matchit)
[![Version](https://img.shields.io/crates/v/matchit?style=for-the-badge)](https://crates.io/crates/matchit)
[![License](https://img.shields.io/crates/l/matchit?style=for-the-badge)](https://crates.io/crates/matchit)

A blazing fast URL router.

``` {.rust .rust-example-rendered}
use matchit::Router;

let mut router = Router::new();
router.insert("/home", "Welcome!")?;
router.insert("/users/:id", "A User")?;

let matched = router.at("/users/978")?;
assert_eq!(matched.params.get("id"), Some("978"));
assert_eq!(*matched.value, "A User");
```

## <a href="#parameters" class="doc-anchor">§</a>Parameters

Along with static routes, the router also supports dynamic route
segments. These can either be named or catch-all parameters:

### <a href="#named-parameters" class="doc-anchor">§</a>Named Parameters

Named parameters like `/:id` match anything until the next `/` or the
end of the path:

``` {.rust .rust-example-rendered}
let mut m = Router::new();
m.insert("/users/:id", true)?;

assert_eq!(m.at("/users/1")?.params.get("id"), Some("1"));
assert_eq!(m.at("/users/23")?.params.get("id"), Some("23"));
assert!(m.at("/users").is_err());
```

### <a href="#catch-all-parameters" class="doc-anchor">§</a>Catch-all Parameters

Catch-all parameters start with `*` and match everything after the `/`.
They must always be at the **end** of the route:

``` {.rust .rust-example-rendered}
let mut m = Router::new();
m.insert("/*p", true)?;

assert_eq!(m.at("/foo.js")?.params.get("p"), Some("foo.js"));
assert_eq!(m.at("/c/bar.css")?.params.get("p"), Some("c/bar.css"));

// note that this would not match
assert!(m.at("/").is_err());
```

## <a href="#routing-priority" class="doc-anchor">§</a>Routing Priority

Static and dynamic route segments are allowed to overlap. If they do,
static segments will be given higher priority:

``` {.rust .rust-example-rendered}
let mut m = Router::new();
m.insert("/", "Welcome!").unwrap()    ;  // priority: 1
m.insert("/about", "About Me").unwrap(); // priority: 1
m.insert("/*filepath", "...").unwrap();  // priority: 2
```

## <a href="#how-does-it-work" class="doc-anchor">§</a>How does it work?

The router takes advantage of the fact that URL routes generally follow
a hierarchical structure. Routes are stored them in a radix trie that
makes heavy use of common prefixes:

``` language-text
Priority   Path             Value
9          \                1
3          ├s               None
2          |├earch\         2
1          |└upport\        3
2          ├blog\           4
1          |    └:post      None
1          |         └\     5
2          ├about-us\       6
1          |        └team\  7
1          └contact\        8
```

This allows us to reduce the route search to a small number of branches.
Child nodes on the same level of the tree are also prioritized by the
number of children with registered values, increasing the chance of
choosing the correct branch of the first try.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Match.html" class="struct"
title="struct matchit::Match">Match</a>  
A successful match consisting of the registered value and URL
parameters, returned by
[`Router::at`](struct.Router.html#method.at "method matchit::Router::at").

<a href="struct.Params.html" class="struct"
title="struct matchit::Params">Params</a>  
A list of parameters returned by a route match.

<a href="struct.ParamsIter.html" class="struct"
title="struct matchit::ParamsIter">ParamsIter</a>  
An iterator over the keys and values of a route’s
[parameters](struct.Params.html "struct matchit::Params").

<a href="struct.Router.html" class="struct"
title="struct matchit::Router">Router</a>  
A URL router.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.InsertError.html" class="enum"
title="enum matchit::InsertError">InsertError</a>  
Represents errors that can occur when inserting a new route.

<a href="enum.MatchError.html" class="enum"
title="enum matchit::MatchError">MatchError</a>  
A failed match attempt.
