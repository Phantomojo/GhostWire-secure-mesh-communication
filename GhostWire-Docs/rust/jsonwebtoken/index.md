Crate jsonwebtoken Copy item path

<span class="sub-heading"><a href="../src/jsonwebtoken/lib.rs.html#1-24" class="src">Source</a>
</span>

Expand description

Create and parses JWT (JSON Web Tokens)

Documentation: [stable](https://docs.rs/jsonwebtoken/)

# Modules<a href="#modules" class="anchor">§</a>

<a href="crypto/index.html" class="mod"
title="mod jsonwebtoken::crypto">crypto</a>  
Lower level functions, if you want to do something other than JWTs

<a href="errors/index.html" class="mod"
title="mod jsonwebtoken::errors">errors</a>  
All the errors that can be encountered while encoding/decoding JWTs

<a href="jwk/index.html" class="mod"
title="mod jsonwebtoken::jwk">jwk</a>  
This crate contains types only for working JWK and JWK Sets This is only
meant to be used to deal with public JWK, not generate ones. Most of the
code in this file is taken from https://github.com/lawliet89/biscuit but
tweaked to remove the private bits as it’s not the goal for this crate
currently.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.DecodingKey.html" class="struct"
title="struct jsonwebtoken::DecodingKey">DecodingKey</a>  
All the different kind of keys we can use to decode a JWT. This key can
be re-used so make sure you only initialize it once if you can for
better performance.

<a href="struct.EncodingKey.html" class="struct"
title="struct jsonwebtoken::EncodingKey">EncodingKey</a>  
A key to encode a JWT with. Can be a secret, a PEM-encoded key or a
DER-encoded key. This key can be re-used so make sure you only
initialize it once if you can for better performance.

<a href="struct.Header.html" class="struct"
title="struct jsonwebtoken::Header">Header</a>  
A basic JWT header, the alg defaults to HS256 and typ is automatically
set to `JWT`. All the other fields are optional.

<a href="struct.TokenData.html" class="struct"
title="struct jsonwebtoken::TokenData">TokenData</a>  
The return type of a successful call to [decode](fn.decode.html).

<a href="struct.Validation.html" class="struct"
title="struct jsonwebtoken::Validation">Validation</a>  
Contains the various validations that are applied after decoding a JWT.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.Algorithm.html" class="enum"
title="enum jsonwebtoken::Algorithm">Algorithm</a>  
The algorithms supported for signing/verifying JWTs

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.decode.html" class="fn"
title="fn jsonwebtoken::decode">decode</a>  
Decode and validate a JWT

<a href="fn.decode_header.html" class="fn"
title="fn jsonwebtoken::decode_header">decode_header</a>  
Decode a JWT without any signature verification/validations and return
its [Header](struct.Header.html).

<a href="fn.encode.html" class="fn"
title="fn jsonwebtoken::encode">encode</a>  
Encode the header and claims given and sign the payload using the
algorithm from the header and the key. If the algorithm given is RSA or
EC, the key needs to be in the PEM format.

<a href="fn.get_current_timestamp.html" class="fn"
title="fn jsonwebtoken::get_current_timestamp">get_current_timestamp</a>  
Gets the current timestamp in the format expected by JWTs.
