Crate oauth2 Copy item path

<span class="sub-heading"><a href="../src/oauth2/lib.rs.html#1-613" class="src">Source</a>
</span>

Expand description

An extensible, strongly-typed implementation of OAuth2 ([RFC
6749](https://tools.ietf.org/html/rfc6749)) including token
introspection ([RFC 7662](https://tools.ietf.org/html/rfc7662)) and
token revocation ([RFC 7009](https://tools.ietf.org/html/rfc7009)).

# <a href="#contents" class="doc-anchor">§</a>Contents

-   [Importing `oauth2`: selecting an HTTP client
    interface](#importing-oauth2-selecting-an-http-client-interface)
-   [Getting started: Authorization Code Grant w/
    PKCE](#getting-started-authorization-code-grant-w-pkce)
    -   [Example: Synchronous (blocking)
        API](#example-synchronous-blocking-api)
    -   [Example: Asynchronous API](#example-asynchronous-api)
-   [Implicit Grant](#implicit-grant)
-   [Resource Owner Password Credentials
    Grant](#resource-owner-password-credentials-grant)
-   [Client Credentials Grant](#client-credentials-grant)
-   [Device Authorization Flow](#device-authorization-flow)
-   [Other examples](#other-examples)
    -   [Contributed Examples](#contributed-examples)

# <a href="#importing-oauth2-selecting-an-http-client-interface"
class="doc-anchor">§</a>Importing `oauth2`: selecting an HTTP client interface

This library offers a flexible HTTP client interface with two modes:

-   **Synchronous (blocking)**

    NOTE: Be careful not to use a blocking HTTP client within `async`
    Rust code, which may panic or cause other issues. The
    [`tokio::task::spawn_blocking`](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html)
    function may be useful in this situation.

-   **Asynchronous**

## <a href="#security-warning" class="doc-anchor">§</a>Security Warning

To prevent
[SSRF](https://cheatsheetseries.owasp.org/cheatsheets/Server_Side_Request_Forgery_Prevention_Cheat_Sheet.html)
vulnerabilities, be sure to configure the HTTP client **not to follow
redirects**. For example, use
[`redirect::Policy::none`](../reqwest/redirect/struct.Policy.html#method.none "associated function reqwest::redirect::Policy::none")
when using [`reqwest`](../reqwest/index.html "mod reqwest"), or
[`redirects(0)`](ureq::AgentBuilder::redirects) when using \[`ureq`\].

## <a href="#http-clients" class="doc-anchor">§</a>HTTP Clients

For the HTTP client modes described above, the following HTTP client
implementations can be used:

-   **[`reqwest`](../reqwest/index.html "mod reqwest")**

    The `reqwest` HTTP client supports both the synchronous and
    asynchronous modes and is enabled by default.

    Synchronous client: \[`reqwest::blocking::Client`\] (requires the
    `reqwest-blocking` feature flag)

    Asynchronous client:
    [`reqwest::Client`](../reqwest/async_impl/client/struct.Client.html "struct reqwest::async_impl::client::Client")
    (requires either the `reqwest` or `reqwest-blocking` feature flags)

-   **[`curl`](curl)**

    The `curl` HTTP client only supports the synchronous HTTP client
    mode and can be enabled in `Cargo.toml` via the `curl` feature flag.

    Synchronous client: [`oauth2::CurlHttpClient`](CurlHttpClient)

-   **[`ureq`](ureq)**

    The `ureq` HTTP client is a simple HTTP client with minimal
    dependencies. It only supports the synchronous HTTP client mode and
    can be enabled in `Cargo.toml` via the `ureq` feature flag.

    Synchronous client: \[`ureq::Agent`\]

-   **Custom**

    In addition to the clients above, users may define their own HTTP
    clients, which must accept an
    [`HttpRequest`](type.HttpRequest.html "type oauth2::HttpRequest")
    and return an
    [`HttpResponse`](type.HttpResponse.html "type oauth2::HttpResponse")
    or error. Users writing their own clients may wish to disable the
    default `reqwest` dependency by specifying
    `default-features = false` in `Cargo.toml` (replacing `...` with the
    desired version of this crate):

    ``` language-toml
    oauth2 = { version = "...", default-features = false }
    ```

    Synchronous HTTP clients should implement the
    [`SyncHttpClient`](trait.SyncHttpClient.html "trait oauth2::SyncHttpClient")
    trait, which is automatically implemented for any function/closure
    that implements:

    <a href="#" class="tooltip" title="This example is not tested">ⓘ</a>
    ``` {.rust .rust-example-rendered}
    Fn(HttpRequest) -> Result<HttpResponse, E>
    where
      E: std::error::Error + 'static
    ```

    Asynchronous HTTP clients should implement the
    [`AsyncHttpClient`](trait.AsyncHttpClient.html "trait oauth2::AsyncHttpClient")
    trait, which is automatically implemented for any function/closure
    that implements:

    <a href="#" class="tooltip" title="This example is not tested">ⓘ</a>
    ``` {.rust .rust-example-rendered}
    Fn(HttpRequest) -> F
    where
      E: std::error::Error + 'static,
      F: Future<Output = Result<HttpResponse, E>>,
    ```

# <a href="#comparing-secrets-securely" class="doc-anchor">§</a>Comparing secrets securely

OAuth flows require comparing secrets received from the provider
servers. To do so securely while avoiding [timing
side-channels](https://en.wikipedia.org/wiki/Timing_attack), the
comparison must be done in constant time, either using a constant-time
crate such as
[`constant_time_eq`](https://crates.io/crates/constant_time_eq) (which
could break if a future compiler version decides to be overly smart
about its optimizations), or by first computing a
cryptographically-secure hash (e.g., SHA-256) of both values and then
comparing the hashes using `==`.

The `timing-resistant-secret-traits` feature flag adds a safe (but
comparatively expensive)
[`PartialEq`](https://doc.rust-lang.org/1.93.1/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")
implementation to the secret types. Timing side-channels are why
[`PartialEq`](https://doc.rust-lang.org/1.93.1/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")
is not auto-derived for this crate’s secret types, and the lack of
[`PartialEq`](https://doc.rust-lang.org/1.93.1/core/cmp/trait.PartialEq.html "trait core::cmp::PartialEq")
is intended to prompt users to think more carefully about these
comparisons.

# <a href="#getting-started-authorization-code-grant-w-pkce"
class="doc-anchor">§</a>Getting started: Authorization Code Grant w/ PKCE

This is the most common OAuth2 flow. PKCE is recommended whenever the
OAuth2 client has no client secret or has a client secret that cannot
remain confidential (e.g., native, mobile, or client-side web
applications).

## <a href="#example-synchronous-blocking-api" class="doc-anchor">§</a>Example: Synchronous (blocking) API

This example works with `oauth2`’s default feature flags, which include
`reqwest`.

``` {.rust .rust-example-rendered}
use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl
};
use oauth2::basic::BasicClient;
use oauth2::reqwest;
use url::Url;

// Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
// token URL.
let client = BasicClient::new(ClientId::new("client_id".to_string()))
    .set_client_secret(ClientSecret::new("client_secret".to_string()))
    .set_auth_uri(AuthUrl::new("http://authorize".to_string())?)
    .set_token_uri(TokenUrl::new("http://token".to_string())?)
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new("http://redirect".to_string())?);

// Generate a PKCE challenge.
let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

// Generate the full authorization URL.
let (auth_url, csrf_token) = client
    .authorize_url(CsrfToken::new_random)
    // Set the desired scopes.
    .add_scope(Scope::new("read".to_string()))
    .add_scope(Scope::new("write".to_string()))
    // Set the PKCE code challenge.
    .set_pkce_challenge(pkce_challenge)
    .url();

// This is the URL you should redirect the user to, in order to trigger the authorization
// process.
println!("Browse to: {}", auth_url);

// Once the user has been redirected to the redirect URL, you'll have access to the
// authorization code. For security reasons, your code should verify that the `state`
// parameter returned by the server matches `csrf_token`.

let http_client = reqwest::blocking::ClientBuilder::new()
    // Following redirects opens the client up to SSRF vulnerabilities.
    .redirect(reqwest::redirect::Policy::none())
    .build()
    .expect("Client should build");

// Now you can trade it for an access token.
let token_result =
    client
        .exchange_code(AuthorizationCode::new("some authorization code".to_string()))
        // Set the PKCE code verifier.
        .set_pkce_verifier(pkce_verifier)
        .request(&http_client)?;

// Unwrapping token_result will either produce a Token or a RequestTokenError.
```

## <a href="#example-asynchronous-api" class="doc-anchor">§</a>Example: Asynchronous API

The example below uses async/await:

``` {.rust .rust-example-rendered}
use oauth2::{
    AuthorizationCode,
    AuthUrl,
    ClientId,
    ClientSecret,
    CsrfToken,
    PkceCodeChallenge,
    RedirectUrl,
    Scope,
    TokenResponse,
    TokenUrl
};
use oauth2::basic::BasicClient;
use oauth2::reqwest;
use url::Url;

// Create an OAuth2 client by specifying the client ID, client secret, authorization URL and
// token URL.
let client = BasicClient::new(ClientId::new("client_id".to_string()))
    .set_client_secret(ClientSecret::new("client_secret".to_string()))
    .set_auth_uri(AuthUrl::new("http://authorize".to_string())?)
    .set_token_uri(TokenUrl::new("http://token".to_string())?)
    // Set the URL the user will be redirected to after the authorization process.
    .set_redirect_uri(RedirectUrl::new("http://redirect".to_string())?);

// Generate a PKCE challenge.
let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

// Generate the full authorization URL.
let (auth_url, csrf_token) = client
    .authorize_url(CsrfToken::new_random)
    // Set the desired scopes.
    .add_scope(Scope::new("read".to_string()))
    .add_scope(Scope::new("write".to_string()))
    // Set the PKCE code challenge.
    .set_pkce_challenge(pkce_challenge)
    .url();

// This is the URL you should redirect the user to, in order to trigger the authorization
// process.
println!("Browse to: {}", auth_url);

// Once the user has been redirected to the redirect URL, you'll have access to the
// authorization code. For security reasons, your code should verify that the `state`
// parameter returned by the server matches `csrf_token`.

let http_client = reqwest::ClientBuilder::new()
    // Following redirects opens the client up to SSRF vulnerabilities.
    .redirect(reqwest::redirect::Policy::none())
    .build()
    .expect("Client should build");

// Now you can trade it for an access token.
let token_result = client
    .exchange_code(AuthorizationCode::new("some authorization code".to_string()))
    // Set the PKCE code verifier.
    .set_pkce_verifier(pkce_verifier)
    .request_async(&http_client)
    .await?;

// Unwrapping token_result will either produce a Token or a RequestTokenError.
```

# <a href="#implicit-grant" class="doc-anchor">§</a>Implicit Grant

This flow fetches an access token directly from the authorization
endpoint. Be sure to understand the security implications of this flow
before using it. In most cases, the Authorization Code Grant flow is
preferable to the Implicit Grant flow.

## <a href="#example" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use oauth2::{
    AuthUrl,
    ClientId,
    CsrfToken,
    RedirectUrl,
    Scope
};
use oauth2::basic::BasicClient;
use url::Url;

let client = BasicClient::new(ClientId::new("client_id".to_string()))
    .set_auth_uri(AuthUrl::new("http://authorize".to_string())?);

// Generate the full authorization URL.
let (auth_url, csrf_token) = client
    .authorize_url(CsrfToken::new_random)
    .use_implicit_flow()
    .url();

// This is the URL you should redirect the user to, in order to trigger the authorization
// process.
println!("Browse to: {}", auth_url);

// Once the user has been redirected to the redirect URL, you'll have the access code.
// For security reasons, your code should verify that the `state` parameter returned by the
// server matches `csrf_token`.
```

# <a href="#resource-owner-password-credentials-grant"
class="doc-anchor">§</a>Resource Owner Password Credentials Grant

You can ask for a *password* access token by calling the
`Client::exchange_password` method, while including the username and
password.

## <a href="#example-1" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use oauth2::{
    AuthUrl,
    ClientId,
    ClientSecret,
    ResourceOwnerPassword,
    ResourceOwnerUsername,
    Scope,
    TokenResponse,
    TokenUrl
};
use oauth2::basic::BasicClient;
use oauth2::reqwest;
use url::Url;

let client = BasicClient::new(ClientId::new("client_id".to_string()))
    .set_client_secret(ClientSecret::new("client_secret".to_string()))
    .set_auth_uri(AuthUrl::new("http://authorize".to_string())?)
    .set_token_uri(TokenUrl::new("http://token".to_string())?);

let http_client = reqwest::blocking::ClientBuilder::new()
    // Following redirects opens the client up to SSRF vulnerabilities.
    .redirect(reqwest::redirect::Policy::none())
    .build()
    .expect("Client should build");

let token_result =
    client
        .exchange_password(
            &ResourceOwnerUsername::new("user".to_string()),
            &ResourceOwnerPassword::new("pass".to_string())
        )
        .add_scope(Scope::new("read".to_string()))
        .request(&http_client)?;
```

# <a href="#client-credentials-grant" class="doc-anchor">§</a>Client Credentials Grant

You can ask for a *client credentials* access token by calling the
`Client::exchange_client_credentials` method.

## <a href="#example-2" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use oauth2::{
    AuthUrl,
    ClientId,
    ClientSecret,
    Scope,
    TokenResponse,
    TokenUrl
};
use oauth2::basic::BasicClient;
use oauth2::reqwest;
use url::Url;

let client = BasicClient::new(ClientId::new("client_id".to_string()))
    .set_client_secret(ClientSecret::new("client_secret".to_string()))
    .set_auth_uri(AuthUrl::new("http://authorize".to_string())?)
    .set_token_uri(TokenUrl::new("http://token".to_string())?);

let http_client = reqwest::blocking::ClientBuilder::new()
    // Following redirects opens the client up to SSRF vulnerabilities.
    .redirect(reqwest::redirect::Policy::none())
    .build()
    .expect("Client should build");

let token_result = client
    .exchange_client_credentials()
    .add_scope(Scope::new("read".to_string()))
    .request(&http_client)?;
```

# <a href="#device-authorization-flow" class="doc-anchor">§</a>Device Authorization Flow

Device Authorization Flow allows users to sign in on browserless or
input-constrained devices. This is a two-stage process; first a
user-code and verification URL are obtained by using the
`Client::exchange_client_credentials` method. Those are displayed to the
user, then are used in a second client to poll the token endpoint for a
token.

## <a href="#example-3" class="doc-anchor">§</a>Example

``` {.rust .rust-example-rendered}
use oauth2::{
    AuthUrl,
    ClientId,
    ClientSecret,
    DeviceAuthorizationUrl,
    Scope,
    StandardDeviceAuthorizationResponse,
    TokenResponse,
    TokenUrl
};
use oauth2::basic::BasicClient;
use oauth2::reqwest;
use url::Url;

let device_auth_url = DeviceAuthorizationUrl::new("http://deviceauth".to_string())?;
let client = BasicClient::new(ClientId::new("client_id".to_string()))
    .set_client_secret(ClientSecret::new("client_secret".to_string()))
    .set_auth_uri(AuthUrl::new("http://authorize".to_string())?)
    .set_token_uri(TokenUrl::new("http://token".to_string())?)
    .set_device_authorization_url(device_auth_url);

let http_client = reqwest::blocking::ClientBuilder::new()
    // Following redirects opens the client up to SSRF vulnerabilities.
    .redirect(reqwest::redirect::Policy::none())
    .build()
    .expect("Client should build");

let details: StandardDeviceAuthorizationResponse = client
    .exchange_device_code()
    .add_scope(Scope::new("read".to_string()))
    .request(&http_client)?;

println!(
    "Open this URL in your browser:\n{}\nand enter the code: {}",
    details.verification_uri().to_string(),
    details.user_code().secret().to_string()
);

let token_result =
    client
    .exchange_device_access_token(&details)
    .request(&http_client, std::thread::sleep, None)?;
```

# <a href="#other-examples" class="doc-anchor">§</a>Other examples

More specific implementations are available as part of the examples:

-   [Google](https://github.com/ramosbugs/oauth2-rs/blob/main/examples/google.rs)
    (includes token revocation)
-   [Github](https://github.com/ramosbugs/oauth2-rs/blob/main/examples/github.rs)
-   [Microsoft Device Authorization Flow
    (async)](https://github.com/ramosbugs/oauth2-rs/blob/main/examples/microsoft_devicecode.rs)
-   [Microsoft
    Graph](https://github.com/ramosbugs/oauth2-rs/blob/main/examples/msgraph.rs)
-   [Wunderlist](https://github.com/ramosbugs/oauth2-rs/blob/main/examples/wunderlist.rs)

## <a href="#contributed-examples" class="doc-anchor">§</a>Contributed Examples

-   [`actix-web-oauth2`](https://github.com/pka/actix-web-oauth2)
    (version 2.x of this crate)

# Re-exports<a href="#reexports" class="anchor">§</a>

`pub use `<a href="../http/index.html" class="mod"
title="mod http"><code>http</code></a>`;`

`pub use `<a href="../url/index.html" class="mod"
title="mod url"><code>url</code></a>`;`

`pub use ::`<a href="../reqwest/index.html" class="mod"
title="mod reqwest"><code>reqwest</code></a>`;`

# Modules<a href="#modules" class="anchor">§</a>

<a href="basic/index.html" class="mod"
title="mod oauth2::basic">basic</a>  
Basic OAuth2 implementation with no extensions ([RFC
6749](https://tools.ietf.org/html/rfc6749)).

<a href="helpers/index.html" class="mod"
title="mod oauth2::helpers">helpers</a>  
Helper methods used by OAuth2 implementations/extensions.

# Structs<a href="#structs" class="anchor">§</a>

<a href="struct.AccessToken.html" class="struct"
title="struct oauth2::AccessToken">AccessToken</a>  
Access token returned by the token endpoint and used to access protected
resources.

<a href="struct.AuthUrl.html" class="struct"
title="struct oauth2::AuthUrl">AuthUrl</a>  
URL of the authorization server’s authorization endpoint.

<a href="struct.AuthorizationCode.html" class="struct"
title="struct oauth2::AuthorizationCode">AuthorizationCode</a>  
Authorization code returned from the authorization endpoint.

<a href="struct.AuthorizationRequest.html" class="struct"
title="struct oauth2::AuthorizationRequest">AuthorizationRequest</a>  
A request to the authorization endpoint

<a href="struct.Client.html" class="struct"
title="struct oauth2::Client">Client</a>  
Stores the configuration for an OAuth2 client.

<a href="struct.ClientCredentialsTokenRequest.html" class="struct"
title="struct oauth2::ClientCredentialsTokenRequest">ClientCredentialsTokenRequest</a>  
A request to exchange client credentials for an access token.

<a href="struct.ClientId.html" class="struct"
title="struct oauth2::ClientId">ClientId</a>  
Client identifier issued to the client during the registration process
described by [Section
2.2](https://tools.ietf.org/html/rfc6749#section-2.2).

<a href="struct.ClientSecret.html" class="struct"
title="struct oauth2::ClientSecret">ClientSecret</a>  
Client password issued to the client during the registration process
described by [Section
2.2](https://tools.ietf.org/html/rfc6749#section-2.2).

<a href="struct.CodeTokenRequest.html" class="struct"
title="struct oauth2::CodeTokenRequest">CodeTokenRequest</a>  
A request to exchange an authorization code for an access token.

<a href="struct.CsrfToken.html" class="struct"
title="struct oauth2::CsrfToken">CsrfToken</a>  
Value used for [CSRF](https://tools.ietf.org/html/rfc6749#section-10.12)
protection via the `state` parameter.

<a href="struct.DeviceAccessTokenRequest.html" class="struct"
title="struct oauth2::DeviceAccessTokenRequest">DeviceAccessTokenRequest</a>  
The request for a device access token from the authorization server.

<a href="struct.DeviceAuthorizationRequest.html" class="struct"
title="struct oauth2::DeviceAuthorizationRequest">DeviceAuthorizationRequest</a>  
The request for a set of verification codes from the authorization
server.

<a href="struct.DeviceAuthorizationResponse.html" class="struct"
title="struct oauth2::DeviceAuthorizationResponse">DeviceAuthorizationResponse</a>  
Standard OAuth2 device authorization response.

<a href="struct.DeviceAuthorizationUrl.html" class="struct"
title="struct oauth2::DeviceAuthorizationUrl">DeviceAuthorizationUrl</a>  
URL of the client’s device authorization endpoint.

<a href="struct.DeviceCode.html" class="struct"
title="struct oauth2::DeviceCode">DeviceCode</a>  
Device code returned by the device authorization endpoint and used to
query the token endpoint.

<a href="struct.EmptyExtraDeviceAuthorizationFields.html" class="struct"
title="struct oauth2::EmptyExtraDeviceAuthorizationFields">EmptyExtraDeviceAuthorizationFields</a>  
Empty (default) extra token fields.

<a href="struct.EmptyExtraTokenFields.html" class="struct"
title="struct oauth2::EmptyExtraTokenFields">EmptyExtraTokenFields</a>  
Empty (default) extra token fields.

<a href="struct.EndUserVerificationUrl.html" class="struct"
title="struct oauth2::EndUserVerificationUrl">EndUserVerificationUrl</a>  
URL of the end-user verification URI on the authorization server.

<a href="struct.EndpointMaybeSet.html" class="struct"
title="struct oauth2::EndpointMaybeSet">EndpointMaybeSet</a>  
[Typestate](https://cliffle.com/blog/rust-typestate/) indicating that an
endpoint may have been set and can be used via fallible methods.

<a href="struct.EndpointNotSet.html" class="struct"
title="struct oauth2::EndpointNotSet">EndpointNotSet</a>  
[Typestate](https://cliffle.com/blog/rust-typestate/) indicating that an
endpoint has not been set and cannot be used.

<a href="struct.EndpointSet.html" class="struct"
title="struct oauth2::EndpointSet">EndpointSet</a>  
[Typestate](https://cliffle.com/blog/rust-typestate/) indicating that an
endpoint has been set and is ready to be used.

<a href="struct.IntrospectionRequest.html" class="struct"
title="struct oauth2::IntrospectionRequest">IntrospectionRequest</a>  
A request to introspect an access token.

<a href="struct.IntrospectionUrl.html" class="struct"
title="struct oauth2::IntrospectionUrl">IntrospectionUrl</a>  
URL of the client’s [RFC 7662 OAuth 2.0 Token
Introspection](https://tools.ietf.org/html/rfc7662) endpoint.

<a href="struct.PasswordTokenRequest.html" class="struct"
title="struct oauth2::PasswordTokenRequest">PasswordTokenRequest</a>  
A request to exchange resource owner credentials for an access token.

<a href="struct.PkceCodeChallenge.html" class="struct"
title="struct oauth2::PkceCodeChallenge">PkceCodeChallenge</a>  
Code Challenge used for [PKCE](https://tools.ietf.org/html/rfc7636)
protection via the `code_challenge` parameter.

<a href="struct.PkceCodeChallengeMethod.html" class="struct"
title="struct oauth2::PkceCodeChallengeMethod">PkceCodeChallengeMethod</a>  
Code Challenge Method used for
[PKCE](https://tools.ietf.org/html/rfc7636) protection via the
`code_challenge_method` parameter.

<a href="struct.PkceCodeVerifier.html" class="struct"
title="struct oauth2::PkceCodeVerifier">PkceCodeVerifier</a>  
Code Verifier used for [PKCE](https://tools.ietf.org/html/rfc7636)
protection via the `code_verifier` parameter. The value must have a
minimum length of 43 characters and a maximum length of 128 characters.
Each character must be ASCII alphanumeric or one of the characters “-” /
“.” / “\_” / “~”.

<a href="struct.RedirectUrl.html" class="struct"
title="struct oauth2::RedirectUrl">RedirectUrl</a>  
URL of the client’s redirection endpoint.

<a href="struct.RefreshToken.html" class="struct"
title="struct oauth2::RefreshToken">RefreshToken</a>  
Refresh token used to obtain a new access token (if supported by the
authorization server).

<a href="struct.RefreshTokenRequest.html" class="struct"
title="struct oauth2::RefreshTokenRequest">RefreshTokenRequest</a>  
A request to exchange a refresh token for an access token.

<a href="struct.ResourceOwnerPassword.html" class="struct"
title="struct oauth2::ResourceOwnerPassword">ResourceOwnerPassword</a>  
Resource owner’s password used directly as an authorization grant to
obtain an access token.

<a href="struct.ResourceOwnerUsername.html" class="struct"
title="struct oauth2::ResourceOwnerUsername">ResourceOwnerUsername</a>  
Resource owner’s username used directly as an authorization grant to
obtain an access token.

<a href="struct.ResponseType.html" class="struct"
title="struct oauth2::ResponseType">ResponseType</a>  
Authorization endpoint response (grant) type defined in [Section
3.1.1](https://tools.ietf.org/html/rfc6749#section-3.1.1).

<a href="struct.RevocationRequest.html" class="struct"
title="struct oauth2::RevocationRequest">RevocationRequest</a>  
A request to revoke a token via an
[`RFC 7009`](https://tools.ietf.org/html/rfc7009#section-2.1) compatible
endpoint.

<a href="struct.RevocationUrl.html" class="struct"
title="struct oauth2::RevocationUrl">RevocationUrl</a>  
URL of the authorization server’s RFC 7009 token revocation endpoint.

<a href="struct.Scope.html" class="struct"
title="struct oauth2::Scope">Scope</a>  
Access token scope, as defined by the authorization server.

<a href="struct.StandardErrorResponse.html" class="struct"
title="struct oauth2::StandardErrorResponse">StandardErrorResponse</a>  
Error response returned by server after requesting an access token.

<a href="struct.StandardTokenIntrospectionResponse.html" class="struct"
title="struct oauth2::StandardTokenIntrospectionResponse">StandardTokenIntrospectionResponse</a>  
Standard OAuth2 token introspection response.

<a href="struct.StandardTokenResponse.html" class="struct"
title="struct oauth2::StandardTokenResponse">StandardTokenResponse</a>  
Standard OAuth2 token response.

<a href="struct.TokenUrl.html" class="struct"
title="struct oauth2::TokenUrl">TokenUrl</a>  
URL of the authorization server’s token endpoint.

<a href="struct.UserCode.html" class="struct"
title="struct oauth2::UserCode">UserCode</a>  
User code returned by the device authorization endpoint and used by the
user to authorize at the verification URI.

<a href="struct.VerificationUriComplete.html" class="struct"
title="struct oauth2::VerificationUriComplete">VerificationUriComplete</a>  
Verification URI returned by the device authorization endpoint and
visited by the user to authorize. Contains the user code.

# Enums<a href="#enums" class="anchor">§</a>

<a href="enum.AuthType.html" class="enum"
title="enum oauth2::AuthType">AuthType</a>  
Indicates whether requests to the authorization server should use basic
authentication or include the parameters in the request body for
requests in which either is valid.

<a href="enum.ConfigurationError.html" class="enum"
title="enum oauth2::ConfigurationError">ConfigurationError</a>  
There was a problem configuring the request.

<a href="enum.DeviceCodeErrorResponseType.html" class="enum"
title="enum oauth2::DeviceCodeErrorResponseType">DeviceCodeErrorResponseType</a>  
Basic access token error types.

<a href="enum.HttpClientError.html" class="enum"
title="enum oauth2::HttpClientError">HttpClientError</a>  
Error type returned by built-in HTTP clients when requests fail.

<a href="enum.RequestTokenError.html" class="enum"
title="enum oauth2::RequestTokenError">RequestTokenError</a>  
Error encountered while requesting access token.

<a href="enum.RevocationErrorResponseType.html" class="enum"
title="enum oauth2::RevocationErrorResponseType">RevocationErrorResponseType</a>  
OAuth 2.0 Token Revocation error response types.

<a href="enum.StandardRevocableToken.html" class="enum"
title="enum oauth2::StandardRevocableToken">StandardRevocableToken</a>  
A token representation usable with authorization servers that support
[RFC 7009](https://tools.ietf.org/html/rfc7009) token revocation.

# Traits<a href="#traits" class="anchor">§</a>

<a href="trait.AsyncHttpClient.html" class="trait"
title="trait oauth2::AsyncHttpClient">AsyncHttpClient</a>  
An asynchronous (future-based) HTTP client.

<a href="trait.EndpointState.html" class="trait"
title="trait oauth2::EndpointState">EndpointState</a>  
[Typestate](https://cliffle.com/blog/rust-typestate/) base trait
indicating whether an endpoint has been configured via its corresponding
setter.

<a href="trait.ErrorResponse.html" class="trait"
title="trait oauth2::ErrorResponse">ErrorResponse</a>  
Server Error Response

<a href="trait.ErrorResponseType.html" class="trait"
title="trait oauth2::ErrorResponseType">ErrorResponseType</a>  
Error types enum.

<a href="trait.ExtraDeviceAuthorizationFields.html" class="trait"
title="trait oauth2::ExtraDeviceAuthorizationFields">ExtraDeviceAuthorizationFields</a>  
Trait for adding extra fields to the `DeviceAuthorizationResponse`.

<a href="trait.ExtraTokenFields.html" class="trait"
title="trait oauth2::ExtraTokenFields">ExtraTokenFields</a>  
Trait for adding extra fields to the `TokenResponse`.

<a href="trait.RevocableToken.html" class="trait"
title="trait oauth2::RevocableToken">RevocableToken</a>  
A revocable token.

<a href="trait.SyncHttpClient.html" class="trait"
title="trait oauth2::SyncHttpClient">SyncHttpClient</a>  
A synchronous (blocking) HTTP client.

<a href="trait.TokenIntrospectionResponse.html" class="trait"
title="trait oauth2::TokenIntrospectionResponse">TokenIntrospectionResponse</a>  
Common methods shared by all OAuth2 token introspection implementations.

<a href="trait.TokenResponse.html" class="trait"
title="trait oauth2::TokenResponse">TokenResponse</a>  
Common methods shared by all OAuth2 token implementations.

<a href="trait.TokenType.html" class="trait"
title="trait oauth2::TokenType">TokenType</a>  
Type of OAuth2 access token.

# Type Aliases<a href="#types" class="anchor">§</a>

<a href="type.DeviceCodeErrorResponse.html" class="type"
title="type oauth2::DeviceCodeErrorResponse">DeviceCodeErrorResponse</a>  
Error response specialization for device code OAuth2 implementation.

<a href="type.HttpRequest.html" class="type"
title="type oauth2::HttpRequest">HttpRequest</a>  
An HTTP request.

<a href="type.HttpResponse.html" class="type"
title="type oauth2::HttpResponse">HttpResponse</a>  
An HTTP response.

<a href="type.StandardDeviceAuthorizationResponse.html" class="type"
title="type oauth2::StandardDeviceAuthorizationResponse">StandardDeviceAuthorizationResponse</a>  
Standard implementation of DeviceAuthorizationResponse which throws away
extra received response fields.
