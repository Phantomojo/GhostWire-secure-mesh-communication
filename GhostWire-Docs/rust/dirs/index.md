Crate dirs Copy item path

<span class="sub-heading"><a href="../src/dirs/lib.rs.html#1-328" class="src">Source</a>
</span>

Expand description

The *dirs* crate is

-   a tiny library with a minimal API (18 public functions)
-   that provides the platform-specific, user-accessible locations
-   for finding and storing configuration, cache and other data
-   on Linux, Redox, Windows (≥ Vista) and macOS.

The library provides the location of these directories by leveraging the
mechanisms defined by

-   the [XDG base
    directory](https://standards.freedesktop.org/basedir-spec/basedir-spec-latest.html)
    and the [XDG user
    directory](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)
    specifications on Linux,
-   the [Known
    Folder](https://msdn.microsoft.com/en-us/library/windows/desktop/bb776911(v=vs.85).aspx)
    system on Windows, and
-   the [Standard
    Directories](https://developer.apple.com/library/content/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW6)
    on macOS.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.audio_dir.html" class="fn"
title="fn dirs::audio_dir">audio_dir</a>  
Returns the path to the user’s audio directory.

<a href="fn.cache_dir.html" class="fn"
title="fn dirs::cache_dir">cache_dir</a>  
Returns the path to the user’s cache directory.

<a href="fn.config_dir.html" class="fn"
title="fn dirs::config_dir">config_dir</a>  
Returns the path to the user’s config directory.

<a href="fn.config_local_dir.html" class="fn"
title="fn dirs::config_local_dir">config_local_dir</a>  
Returns the path to the user’s local config directory.

<a href="fn.data_dir.html" class="fn"
title="fn dirs::data_dir">data_dir</a>  
Returns the path to the user’s data directory.

<a href="fn.data_local_dir.html" class="fn"
title="fn dirs::data_local_dir">data_local_dir</a>  
Returns the path to the user’s local data directory.

<a href="fn.desktop_dir.html" class="fn"
title="fn dirs::desktop_dir">desktop_dir</a>  
Returns the path to the user’s desktop directory.

<a href="fn.document_dir.html" class="fn"
title="fn dirs::document_dir">document_dir</a>  
Returns the path to the user’s document directory.

<a href="fn.download_dir.html" class="fn"
title="fn dirs::download_dir">download_dir</a>  
Returns the path to the user’s download directory.

<a href="fn.executable_dir.html" class="fn"
title="fn dirs::executable_dir">executable_dir</a>  
Returns the path to the user’s executable directory.

<a href="fn.font_dir.html" class="fn"
title="fn dirs::font_dir">font_dir</a>  
Returns the path to the user’s font directory.

<a href="fn.home_dir.html" class="fn"
title="fn dirs::home_dir">home_dir</a>  
Returns the path to the user’s home directory.

<a href="fn.picture_dir.html" class="fn"
title="fn dirs::picture_dir">picture_dir</a>  
Returns the path to the user’s picture directory.

<a href="fn.preference_dir.html" class="fn"
title="fn dirs::preference_dir">preference_dir</a>  
Returns the path to the user’s preference directory.

<a href="fn.public_dir.html" class="fn"
title="fn dirs::public_dir">public_dir</a>  
Returns the path to the user’s public directory.

<a href="fn.runtime_dir.html" class="fn"
title="fn dirs::runtime_dir">runtime_dir</a>  
Returns the path to the user’s runtime directory.

<a href="fn.state_dir.html" class="fn"
title="fn dirs::state_dir">state_dir</a>  
Returns the path to the user’s state directory.

<a href="fn.template_dir.html" class="fn"
title="fn dirs::template_dir">template_dir</a>  
Returns the path to the user’s template directory.

<a href="fn.video_dir.html" class="fn"
title="fn dirs::video_dir">video_dir</a>  
Returns the path to the user’s video directory.
