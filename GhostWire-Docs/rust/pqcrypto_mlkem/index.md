Crate pqcrypto\_mlkem Copy item path

<span class="sub-heading"><a href="../src/pqcrypto_mlkem/lib.rs.html#1-45" class="src">Source</a>
</span>

Expand description

# <a href="#mlkem" class="doc-anchor">§</a>mlkem

This crate provides bindings to and wrappers around the following
implementations from [PQClean](https://github.com/pqclean/pqclean/):

-   ml-kem-512 - clean
-   ml-kem-768 - clean
-   ml-kem-1024 - clean

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use crate::mlkem1024::`<a href="mlkem1024/fn.ciphertext_bytes.html" class="fn"
title="fn pqcrypto_mlkem::mlkem1024::ciphertext_bytes"><code>ciphertext_bytes</code></a>` as mlkem1024_ciphertext_bytes;`

`pub use crate::mlkem1024::`<a href="mlkem1024/fn.decapsulate.html" class="fn"
title="fn pqcrypto_mlkem::mlkem1024::decapsulate"><code>decapsulate</code></a>` as mlkem1024_decapsulate;`

`pub use crate::mlkem1024::`<a href="mlkem1024/fn.encapsulate.html" class="fn"
title="fn pqcrypto_mlkem::mlkem1024::encapsulate"><code>encapsulate</code></a>` as mlkem1024_encapsulate;`

`pub use crate::mlkem1024::`<a href="mlkem1024/fn.keypair.html" class="fn"
title="fn pqcrypto_mlkem::mlkem1024::keypair"><code>keypair</code></a>` as mlkem1024_keypair;`

`pub use crate::mlkem1024::`<a href="mlkem1024/fn.public_key_bytes.html" class="fn"
title="fn pqcrypto_mlkem::mlkem1024::public_key_bytes"><code>public_key_bytes</code></a>` as mlkem1024_public_key_bytes;`

`pub use crate::mlkem1024::`<a href="mlkem1024/fn.secret_key_bytes.html" class="fn"
title="fn pqcrypto_mlkem::mlkem1024::secret_key_bytes"><code>secret_key_bytes</code></a>` as mlkem1024_secret_key_bytes;`

`pub use crate::mlkem1024::`<a href="mlkem1024/fn.shared_secret_bytes.html" class="fn"
title="fn pqcrypto_mlkem::mlkem1024::shared_secret_bytes"><code>shared_secret_bytes</code></a>` as mlkem1024_shared_secret_bytes;`

`pub use crate::mlkem512::`<a href="mlkem512/fn.ciphertext_bytes.html" class="fn"
title="fn pqcrypto_mlkem::mlkem512::ciphertext_bytes"><code>ciphertext_bytes</code></a>` as mlkem512_ciphertext_bytes;`

`pub use crate::mlkem512::`<a href="mlkem512/fn.decapsulate.html" class="fn"
title="fn pqcrypto_mlkem::mlkem512::decapsulate"><code>decapsulate</code></a>` as mlkem512_decapsulate;`

`pub use crate::mlkem512::`<a href="mlkem512/fn.encapsulate.html" class="fn"
title="fn pqcrypto_mlkem::mlkem512::encapsulate"><code>encapsulate</code></a>` as mlkem512_encapsulate;`

`pub use crate::mlkem512::`<a href="mlkem512/fn.keypair.html" class="fn"
title="fn pqcrypto_mlkem::mlkem512::keypair"><code>keypair</code></a>` as mlkem512_keypair;`

`pub use crate::mlkem512::`<a href="mlkem512/fn.public_key_bytes.html" class="fn"
title="fn pqcrypto_mlkem::mlkem512::public_key_bytes"><code>public_key_bytes</code></a>` as mlkem512_public_key_bytes;`

`pub use crate::mlkem512::`<a href="mlkem512/fn.secret_key_bytes.html" class="fn"
title="fn pqcrypto_mlkem::mlkem512::secret_key_bytes"><code>secret_key_bytes</code></a>` as mlkem512_secret_key_bytes;`

`pub use crate::mlkem512::`<a href="mlkem512/fn.shared_secret_bytes.html" class="fn"
title="fn pqcrypto_mlkem::mlkem512::shared_secret_bytes"><code>shared_secret_bytes</code></a>` as mlkem512_shared_secret_bytes;`

`pub use crate::mlkem768::`<a href="mlkem768/fn.ciphertext_bytes.html" class="fn"
title="fn pqcrypto_mlkem::mlkem768::ciphertext_bytes"><code>ciphertext_bytes</code></a>` as mlkem768_ciphertext_bytes;`

`pub use crate::mlkem768::`<a href="mlkem768/fn.decapsulate.html" class="fn"
title="fn pqcrypto_mlkem::mlkem768::decapsulate"><code>decapsulate</code></a>` as mlkem768_decapsulate;`

`pub use crate::mlkem768::`<a href="mlkem768/fn.encapsulate.html" class="fn"
title="fn pqcrypto_mlkem::mlkem768::encapsulate"><code>encapsulate</code></a>` as mlkem768_encapsulate;`

`pub use crate::mlkem768::`<a href="mlkem768/fn.keypair.html" class="fn"
title="fn pqcrypto_mlkem::mlkem768::keypair"><code>keypair</code></a>` as mlkem768_keypair;`

`pub use crate::mlkem768::`<a href="mlkem768/fn.public_key_bytes.html" class="fn"
title="fn pqcrypto_mlkem::mlkem768::public_key_bytes"><code>public_key_bytes</code></a>` as mlkem768_public_key_bytes;`

`pub use crate::mlkem768::`<a href="mlkem768/fn.secret_key_bytes.html" class="fn"
title="fn pqcrypto_mlkem::mlkem768::secret_key_bytes"><code>secret_key_bytes</code></a>` as mlkem768_secret_key_bytes;`

`pub use crate::mlkem768::`<a href="mlkem768/fn.shared_secret_bytes.html" class="fn"
title="fn pqcrypto_mlkem::mlkem768::shared_secret_bytes"><code>shared_secret_bytes</code></a>` as mlkem768_shared_secret_bytes;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="ffi/index.html" class="mod"
title="mod pqcrypto_mlkem::ffi">ffi</a>  
Foreign function interfaces

<a href="mlkem512/index.html" class="mod"
title="mod pqcrypto_mlkem::mlkem512">mlkem512</a>  
ml-kem-512

<a href="mlkem768/index.html" class="mod"
title="mod pqcrypto_mlkem::mlkem768">mlkem768</a>  
ml-kem-768

<a href="mlkem1024/index.html" class="mod"
title="mod pqcrypto_mlkem::mlkem1024">mlkem1024</a>  
ml-kem-1024
