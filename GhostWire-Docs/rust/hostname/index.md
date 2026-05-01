Crate hostname Copy item path

<span class="sub-heading"><a href="../src/hostname/lib.rs.html#1-128" class="src">Source</a>
</span>

Expand description

A crate with utilities to get and set the system’s host name.

## <a href="#examples" class="doc-anchor">§</a>Examples

Get the host name:

``` {.rust .rust-example-rendered}
let name = hostname::get()?;
println!("{:?}", name);
```

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.get.html" class="fn" title="fn hostname::get">get</a>  
Return the system hostname.
