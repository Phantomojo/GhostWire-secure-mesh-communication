Crate find\_msvc\_tools Copy item path

<span class="sub-heading"><a href="../src/find_msvc_tools/lib.rs.html#1-28" class="src">Source</a>
</span>

Expand description

These modules are all glue to support reading the MSVC version from the
registry and from COM interfaces.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.Tool.html" class="struct"
title="struct find_msvc_tools::Tool">Tool</a>  
`Tool` found by `find-msvc-tools`

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Env.html" class="enum"
title="enum find_msvc_tools::Env">Env</a>  
<a href="enum.VsVers.html" class="enum"
title="enum find_msvc_tools::VsVers">VsVers</a>  
A version of Visual Studio

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.EnvGetter.html" class="trait"
title="trait find_msvc_tools::EnvGetter">EnvGetter</a>

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.find.html" class="fn"
title="fn find_msvc_tools::find">find</a>  
Attempts to find a tool within an MSVC installation using the Windows
registry as a point to search from.

<a href="fn.find_tool.html" class="fn"
title="fn find_msvc_tools::find_tool">find_tool</a>  
Similar to the `find` function above, this function will attempt the
same operation (finding a MSVC tool in a local install) but instead
returns a `Tool` which may be introspected.

<a href="fn.find_tool_with_env.html" class="fn"
title="fn find_msvc_tools::find_tool_with_env">find_tool_with_env</a>  
<a href="fn.find_vs_version.html" class="fn"
title="fn find_msvc_tools::find_vs_version">find_vs_version</a>  
Find the most recent installed version of Visual Studio

<a href="fn.get_ucrt_dir.html" class="fn"
title="fn find_msvc_tools::get_ucrt_dir">get_ucrt_dir</a>  
To find the Universal CRT we look in a specific registry key for where
all the Universal CRTs are located and then sort them asciibetically to
find the newest version. While this sort of sorting isn’t ideal, it is
what vcvars does so that’s good enough for us.
