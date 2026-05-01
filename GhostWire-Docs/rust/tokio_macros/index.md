Crate tokio\_macros Copy item path

<span class="sub-heading"><a href="../src/tokio_macros/lib.rs.html#1-666" class="src">Source</a>
</span>

Expand description

Macros for use with Tokio

# Attribute Macros<a href="#attributes" class="anchor">§</a>

<a href="attr.main.html" class="attr"
title="attr tokio_macros::main">main</a>  
Marks async function to be executed by the selected runtime. This macro
helps set up a `Runtime` without requiring the user to use
[Runtime](../tokio/runtime/struct.Runtime.html) or
[Builder](../tokio/runtime/struct.Builder.html) directly.

<a href="attr.main_fail.html" class="attr"
title="attr tokio_macros::main_fail">main_fail</a>  
Always fails with the error message below.

<a href="attr.main_rt.html" class="attr"
title="attr tokio_macros::main_rt">main_rt</a>  
Marks async function to be executed by selected runtime. This macro
helps set up a `Runtime` without requiring the user to use
[Runtime](../tokio/runtime/struct.Runtime.html) or
[Builder](../tokio/runtime/struct.Builder.html) directly.

<a href="attr.test.html" class="attr"
title="attr tokio_macros::test">test</a>  
Marks async function to be executed by runtime, suitable to test
environment. This macro helps set up a `Runtime` without requiring the
user to use [Runtime](../tokio/runtime/struct.Runtime.html) or
[Builder](../tokio/runtime/struct.Builder.html) directly.

<a href="attr.test_fail.html" class="attr"
title="attr tokio_macros::test_fail">test_fail</a>  
Always fails with the error message below.

<a href="attr.test_rt.html" class="attr"
title="attr tokio_macros::test_rt">test_rt</a>  
Marks async function to be executed by runtime, suitable to test
environment
