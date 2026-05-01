Crate ruma\_identifiers\_validation Copy item path

<span class="sub-heading"><a href="../src/ruma_identifiers_validation/lib.rs.html#1-68"
class="src">Source</a> </span>

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use error::`<a href="error/enum.Error.html" class="enum"
title="enum ruma_identifiers_validation::error::Error"><code>Error</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="base64_public_key/index.html" class="mod"
title="mod ruma_identifiers_validation::base64_public_key">base64_public_key</a>

<a href="client_secret/index.html" class="mod"
title="mod ruma_identifiers_validation::client_secret">client_secret</a>

<a href="error/index.html" class="mod"
title="mod ruma_identifiers_validation::error">error</a>

Error conditions.

<a href="event_id/index.html" class="mod"
title="mod ruma_identifiers_validation::event_id">event_id</a>

<a href="key_id/index.html" class="mod"
title="mod ruma_identifiers_validation::key_id">key_id</a>

<a href="mxc_uri/index.html" class="mod"
title="mod ruma_identifiers_validation::mxc_uri">mxc_uri</a>

<a href="room_alias_id/index.html" class="mod"
title="mod ruma_identifiers_validation::room_alias_id">room_alias_id</a>

<a href="room_id/index.html" class="mod"
title="mod ruma_identifiers_validation::room_id">room_id</a>

<a href="room_id_or_alias_id/index.html" class="mod"
title="mod ruma_identifiers_validation::room_id_or_alias_id">room_id_or_alias_id</a>

<a href="room_version_id/index.html" class="mod"
title="mod ruma_identifiers_validation::room_version_id">room_version_id</a>

<a href="server_name/index.html" class="mod"
title="mod ruma_identifiers_validation::server_name">server_name</a>

<a href="server_signing_key_version/index.html" class="mod"
title="mod ruma_identifiers_validation::server_signing_key_version">server_signing_key_version</a>

<a href="space_child_order/index.html" class="mod"
title="mod ruma_identifiers_validation::space_child_order">space_child_order</a>

<a href="user_id/index.html" class="mod"
title="mod ruma_identifiers_validation::user_id">user_id</a>

<a href="voip_version_id/index.html" class="mod"
title="mod ruma_identifiers_validation::voip_version_id">voip_version_id</a>

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.ID_MAX_BYTES.html" class="constant"
title="constant ruma_identifiers_validation::ID_MAX_BYTES">ID_MAX_BYTES</a>  
The maximum allowed length of Matrix identifiers, in bytes.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.KeyName.html" class="trait"
title="trait ruma_identifiers_validation::KeyName">KeyName</a>  
Helper trait to validate the name of a key.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.localpart_is_backwards_compatible.html" class="fn"
title="fn ruma_identifiers_validation::localpart_is_backwards_compatible">localpart_is_backwards_compatible</a>  
Check whether the Matrix identifier localpart is [allowed over
federation](https://spec.matrix.org/latest/appendices/#historical-user-ids).
