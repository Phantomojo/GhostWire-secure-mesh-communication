Crate openssl\_probe Copy item path

<span class="sub-heading"><a href="../src/openssl_probe/lib.rs.html#1-225" class="src">Source</a>
</span>

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.ProbeResult.html" class="struct"
title="struct openssl_probe::ProbeResult">ProbeResult</a>

# Constants<a href="#constants" class="anchor">§</a>

<a href="constant.ENV_CERT_DIR.html" class="constant"
title="constant openssl_probe::ENV_CERT_DIR">ENV_CERT_DIR</a>  
The OpenSSL environment variable to configure what certificates
directory to use.

<a href="constant.ENV_CERT_FILE.html" class="constant"
title="constant openssl_probe::ENV_CERT_FILE">ENV_CERT_FILE</a>  
The OpenSSL environment variable to configure what certificate file to
use.

# Functions<a href="#functions" class="anchor">§</a>

<a href="fn.candidate_cert_dirs.html" class="fn"
title="fn openssl_probe::candidate_cert_dirs">candidate_cert_dirs</a>  
Probe the system for the directory in which CA certificates should
likely be found.

<a href="fn.has_ssl_cert_env_vars.html" class="fn"
title="fn openssl_probe::has_ssl_cert_env_vars">has_ssl_cert_env_vars</a>  
Check whether the OpenSSL `SSL_CERT_FILE` and/or `SSL_CERT_DIR`
environment variable is configured in this process with an existing file
or directory.

<a href="fn.probe.html" class="fn"
title="fn openssl_probe::probe">probe</a>  
Probe the current system for the “cert file” and “cert dir” variables
that OpenSSL typically requires.

<a href="fn.try_init_openssl_env_vars.html" class="fn"
title="fn openssl_probe::try_init_openssl_env_vars">try_init_openssl_env_vars</a><sup>⚠</sup>  
Probe for SSL certificates on the system, then configure the SSL
certificate `SSL_CERT_FILE` and `SSL_CERT_DIR` environment variables in
this process for OpenSSL to use.
