# 🛠️ GhostWire Implementation Guide
## Step-by-Step Instructions for Critical Improvements

**Version:** 1.0  
**Target Audience:** Developers implementing recommendations from COMPREHENSIVE_CODE_REVIEW.md  
**Estimated Time:** P0: 1 week, P1: 2 weeks, P2: 1 month

---

## 🚨 Priority 0: Critical Fixes (Week 1)

### 1. Add AGPL-3.0 License File

**Issue:** Empty LICENSE file creates legal uncertainty
**Time:** 5 minutes
**Risk:** HIGH - Cannot legally distribute software

```bash
# Download official AGPL-3.0 license
wget https://www.gnu.org/licenses/agpl-3.0.txt -O LICENSE

# Or manually add from https://www.gnu.org/licenses/agpl-3.0.txt
# Commit
git add LICENSE
git commit -m "fix: add AGPL-3.0 license text"
git push
```

**Verification:**
```bash
wc -l LICENSE  # Should show ~661 lines
head -1 LICENSE  # Should show "GNU AFFERO GENERAL PUBLIC LICENSE"
```

---

### 2. Fix Dependency Vulnerabilities

**Issue:** 11 security vulnerabilities in dependencies
**Time:** 2 hours
**Risk:** CRITICAL - DoS, panics, command injection

#### Step 2.1: Update Rust Dependencies

```bash
cd ghostwire

# Update Cargo.toml
```

Edit `ghostwire/Cargo.toml`:

```toml
[dependencies]
# Fix: Update matrix-sdk to patch DoS vulnerability
matrix-sdk = "0.16"  # was 0.7 - fixes RUSTSEC-2025-0135, RUSTSEC-2025-0065

# Fix: Update ring to patch AES panic
ring = "0.17.12"  # was 0.16.20 - fixes RUSTSEC-2025-0009, RUSTSEC-2025-0010

# Fix: Replace unmaintained pqcrypto-kyber
# pqcrypto-kyber = "0.8"  # Remove this line
pqcrypto-mlkem = "0.3"  # Add this - NIST ML-KEM standard

# Fix: Update instant (transitive dependency)
instant = "0.2"  # was 0.1.13 - fixes RUSTSEC-2024-0384

# Fix: Update backoff
backoff = "0.5"  # was 0.4 - fixes RUSTSEC-2025-0012

# Fix: Update rustls-pemfile
rustls-pemfile = "2.0"  # was 1.0.4 - fixes RUSTSEC-2025-0134
```

Update code referencing pqcrypto-kyber:

```bash
# Find all references
grep -r "pqcrypto-kyber" ghostwire/src/

# Update imports
# Before: use pqcrypto_kyber::*;
# After: use pqcrypto_mlkem::*;

# API is similar, but verify:
# - kyber::keypair() → mlkem::keypair()
# - kyber::encapsulate() → mlkem::encapsulate()
# - kyber::decapsulate() → mlkem::decapsulate()
```

```bash
# Update and verify
cargo update
cargo audit  # Should show 0 vulnerabilities
cargo test   # Ensure nothing broke
```

#### Step 2.2: Update Frontend Dependencies

```bash
cd ../webui

# Fix glob command injection and Vite vulnerabilities
npm audit fix --force

# If automatic fix doesn't work, manually update package.json:
```

Edit `webui/package.json`:

```json
{
  "dependencies": {
    "glob": "^11.0.0",  // Update from 10.x to 11.x
    "vite": "^7.0.8"     // Update to latest patch
  }
}
```

```bash
npm install
npm audit  # Should show 0 vulnerabilities
npm run build  # Verify build works
```

**Verification:**
```bash
cd ghostwire
cargo audit --json | jq '.vulnerabilities.count'  # Should be 0

cd ../webui
npm audit --json | jq '.metadata.vulnerabilities | add'  # Should be 0
```

---

### 3. Remove Hardcoded Password

**Issue:** Password literal `"password"` in source code
**Time:** 1 hour
**Risk:** CRITICAL - Authentication bypass

#### Step 3.1: Update Configuration Structure

Create `ghostwire/src/config/credentials.rs`:

```rust
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PasswordSource {
    /// Load from environment variable
    EnvVar(String),
    /// Load from encrypted file
    EncryptedFile { path: PathBuf, key_source: String },
    /// Load from system keychain (macOS/Linux/Windows)
    SystemKeychain { service: String, account: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatrixCredentials {
    pub homeserver: String,
    pub username: String,
    pub password_source: PasswordSource,
}

#[derive(Zeroize, ZeroizeOnDrop)]
pub struct SecurePassword(String);

impl MatrixCredentials {
    pub async fn get_password(&self) -> Result<SecurePassword> {
        match &self.password_source {
            PasswordSource::EnvVar(var_name) => {
                let password = env::var(var_name)
                    .with_context(|| format!("Environment variable {} not set", var_name))?;
                Ok(SecurePassword(password))
            }
            
            PasswordSource::EncryptedFile { path, key_source } => {
                let encrypted_data = fs::read(path)
                    .with_context(|| format!("Failed to read password file: {:?}", path))?;
                
                let key = env::var(key_source)
                    .with_context(|| format!("Encryption key env var {} not set", key_source))?;
                
                let password = decrypt_password(&encrypted_data, key.as_bytes())?;
                Ok(SecurePassword(password))
            }
            
            PasswordSource::SystemKeychain { service, account } => {
                #[cfg(target_os = "macos")]
                {
                    use security_framework::passwords::get_generic_password;
                    let password = get_generic_password(service, account)
                        .map_err(|e| anyhow::anyhow!("Failed to get password from keychain: {}", e))?;
                    Ok(SecurePassword(String::from_utf8(password)?))
                }
                
                #[cfg(target_os = "linux")]
                {
                    use secret_service::SecretService;
                    let service = SecretService::connect(secret_service::EncryptionType::Dh)?;
                    let collection = service.get_default_collection()?;
                    let items = collection.search_items(vec![
                        ("service", service),
                        ("account", account),
                    ])?;
                    
                    let password = items.first()
                        .ok_or_else(|| anyhow::anyhow!("Password not found in keychain"))?
                        .get_secret()?;
                    Ok(SecurePassword(String::from_utf8(password)?))
                }
                
                #[cfg(target_os = "windows")]
                {
                    use windows_credentials::Credential;
                    let cred = Credential::read(&format!("{}:{}", service, account))
                        .ok_or_else(|| anyhow::anyhow!("Password not found in credential store"))?;
                    Ok(SecurePassword(cred.password))
                }
                
                #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
                {
                    Err(anyhow::anyhow!("System keychain not supported on this platform"))
                }
            }
        }
    }
}

impl SecurePassword {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn decrypt_password(encrypted: &[u8], key: &[u8]) -> Result<String> {
    use aes_gcm::{Aes256Gcm, KeyInit};
    use aes_gcm::aead::{Aead, OsRng};
    
    // Key derivation
    let key = argon2::hash_raw(key, b"ghostwire-password-salt", &argon2::Config::default())?;
    let cipher = Aes256Gcm::new_from_slice(&key[..32])?;
    
    // Decrypt (nonce is first 12 bytes)
    let nonce = &encrypted[..12];
    let ciphertext = &encrypted[12..];
    
    let plaintext = cipher.decrypt(nonce.into(), ciphertext)
        .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;
    
    Ok(String::from_utf8(plaintext)?)
}
```

#### Step 3.2: Update Core to Use New Credentials

Edit `ghostwire/src/core/mod.rs`:

```rust
// Before:
.login_username(&self.user, "password")  // TODO: get from config

// After:
use crate::config::credentials::MatrixCredentials;

impl Core {
    async fn login_matrix(&self) -> Result<()> {
        let credentials = self.config.matrix_credentials
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Matrix credentials not configured"))?;
        
        let password = credentials.get_password().await?;
        
        self.matrix_client
            .login_username(&credentials.username, password.as_str())
            .await?;
        
        // password is automatically zeroized on drop
        Ok(())
    }
}
```

#### Step 3.3: Update Configuration File

Edit `ghostwire.toml.example`:

```toml
[matrix]
homeserver = "https://matrix.org"
username = "ghostwire_user"

# Option 1: Environment variable (recommended for servers)
password_source = { EnvVar = "GHOSTWIRE_MATRIX_PASSWORD" }

# Option 2: Encrypted file (recommended for local use)
# password_source = { EncryptedFile = { path = "~/.config/ghostwire/matrix_password.enc", key_source = "GHOSTWIRE_ENCRYPTION_KEY" } }

# Option 3: System keychain (recommended for desktop)
# password_source = { SystemKeychain = { service = "ghostwire", account = "matrix" } }
```

#### Step 3.4: Create Password Setup Tool

Create `ghostwire/src/cli/setup_password.rs`:

```rust
use crate::config::credentials::MatrixCredentials;
use anyhow::Result;
use std::io::{self, Write};

pub async fn setup_matrix_password() -> Result<()> {
    println!("Matrix Password Setup");
    println!("=====================\n");
    
    println!("Choose password storage method:");
    println!("1. Environment variable (recommended for servers)");
    println!("2. Encrypted file (recommended for local use)");
    println!("3. System keychain (recommended for desktop)");
    
    print!("Choice [1-3]: ");
    io::stdout().flush()?;
    
    let mut choice = String::new();
    io::stdin().read_line(&mut choice)?;
    
    match choice.trim() {
        "1" => setup_env_var().await,
        "2" => setup_encrypted_file().await,
        "3" => setup_keychain().await,
        _ => {
            println!("Invalid choice");
            Ok(())
        }
    }
}

async fn setup_env_var() -> Result<()> {
    println!("\nAdd this to your shell profile (~/.bashrc, ~/.zshrc):");
    println!("export GHOSTWIRE_MATRIX_PASSWORD='your_password_here'");
    println!("\nThen update ghostwire.toml:");
    println!("password_source = {{ EnvVar = \"GHOSTWIRE_MATRIX_PASSWORD\" }}");
    Ok(())
}

async fn setup_encrypted_file() -> Result<()> {
    print!("Enter Matrix password: ");
    io::stdout().flush()?;
    
    let password = rpassword::read_password()?;
    
    // Generate encryption key
    let key = rand::random::<[u8; 32]>();
    let key_hex = hex::encode(key);
    
    // Encrypt password
    let encrypted = encrypt_password(&password, &key)?;
    
    // Save to file
    let path = dirs::config_dir()
        .unwrap()
        .join("ghostwire")
        .join("matrix_password.enc");
    
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(&path, encrypted)?;
    
    println!("\nPassword encrypted and saved to: {:?}", path);
    println!("\nAdd this to your shell profile:");
    println!("export GHOSTWIRE_ENCRYPTION_KEY='{}'", key_hex);
    println!("\nThen update ghostwire.toml:");
    println!("password_source = {{ EncryptedFile = {{ path = \"{}\", key_source = \"GHOSTWIRE_ENCRYPTION_KEY\" }} }}", path.display());
    
    Ok(())
}

async fn setup_keychain() -> Result<()> {
    print!("Enter Matrix password: ");
    io::stdout().flush()?;
    
    let password = rpassword::read_password()?;
    
    #[cfg(target_os = "macos")]
    {
        use security_framework::passwords::set_generic_password;
        set_generic_password("ghostwire", "matrix", password.as_bytes())?;
        println!("\nPassword saved to macOS Keychain");
    }
    
    #[cfg(target_os = "linux")]
    {
        use secret_service::SecretService;
        let service = SecretService::connect(secret_service::EncryptionType::Dh)?;
        let collection = service.get_default_collection()?;
        collection.create_item(
            "GhostWire Matrix Password",
            vec![("service", "ghostwire"), ("account", "matrix")],
            password.as_bytes(),
            true,  // replace existing
            "text/plain",
        )?;
        println!("\nPassword saved to Linux Secret Service");
    }
    
    #[cfg(target_os = "windows")]
    {
        use windows_credentials::Credential;
        let mut cred = Credential::new("ghostwire:matrix");
        cred.password = password;
        cred.write()?;
        println!("\nPassword saved to Windows Credential Manager");
    }
    
    println!("\nUpdate ghostwire.toml:");
    println!("password_source = {{ SystemKeychain = {{ service = \"ghostwire\", account = \"matrix\" }} }}");
    
    Ok(())
}
```

Add to CLI:

```rust
// ghostwire/src/cli/mod.rs
#[derive(Subcommand)]
enum Commands {
    // ... existing commands
    
    /// Setup Matrix password storage
    SetupPassword,
}

// In main handler:
Commands::SetupPassword => {
    setup_password::setup_matrix_password().await?;
}
```

**Usage:**
```bash
cargo run -- setup-password
# Follow prompts to set up password
```

**Verification:**
```bash
# Search for hardcoded password
grep -r '"password"' ghostwire/src/  # Should find 0 results (except in setup tool)

# Test login
cargo run -- web  # Should login to Matrix successfully
```

---

### 4. Remove Secrets from Repository

**Issue:** `ghostwire_token.txt` contains sensitive data
**Time:** 30 minutes
**Risk:** HIGH - Credential exposure

```bash
# Step 1: Add to .gitignore
echo "ghostwire_token.txt" >> .gitignore
echo "*.enc" >> .gitignore  # Encrypted files
echo ".env" >> .gitignore  # Environment files

# Step 2: Remove from repository (keep local copy)
cp ghostwire_token.txt ~/.config/ghostwire/token.txt
git rm --cached ghostwire_token.txt

# Step 3: Remove from git history
git filter-branch --force --index-filter \
  'git rm --cached --ignore-unmatch ghostwire_token.txt' \
  --prune-empty --tag-name-filter cat -- --all

# Step 4: Commit
git add .gitignore
git commit -m "security: remove token from version control and git history"

# Step 5: Force push (WARNING: coordinate with team)
git push origin --force --all

# Step 6: Update code to read from new location
```

Edit code that reads token:

```rust
// Before:
let token = fs::read_to_string("ghostwire_token.txt")?;

// After:
use dirs::config_dir;

let token_path = config_dir()
    .ok_or_else(|| anyhow::anyhow!("Could not determine config directory"))?
    .join("ghostwire")
    .join("token.txt");

let token = fs::read_to_string(&token_path)
    .with_context(|| format!("Failed to read token from {:?}", token_path))?;
```

**Create migration script:**

```bash
# scripts/migrate_token.sh
#!/bin/bash

if [ -f "ghostwire_token.txt" ]; then
    echo "Migrating token to config directory..."
    mkdir -p ~/.config/ghostwire
    mv ghostwire_token.txt ~/.config/ghostwire/token.txt
    chmod 600 ~/.config/ghostwire/token.txt
    echo "Token migrated successfully"
else
    echo "No token file found to migrate"
fi
```

**Verification:**
```bash
# Verify file not in repo
git ls-files | grep token.txt  # Should be empty

# Verify not in git history
git log --all --full-history -- ghostwire_token.txt  # Should be empty

# Verify .gitignore working
echo "test" > ghostwire_token.txt
git status  # Should NOT show ghostwire_token.txt
rm ghostwire_token.txt
```

---

### 5. Fix CORS Configuration

**Issue:** Permissive CORS allows any origin
**Time:** 20 minutes
**Risk:** MEDIUM - XSS, CSRF attacks

Edit `ghostwire/src/web.rs`:

```rust
// Before:
use tower_http::cors::CorsLayer;
let app = Router::new()
    // ...
    .layer(CorsLayer::permissive());

// After:
use tower_http::cors::{CorsLayer, AllowOrigin, AllowMethods, AllowHeaders};
use http::{Method, header};

fn create_cors_layer() -> CorsLayer {
    // Allowed origins (configure via environment)
    let allowed_origins = env::var("GHOSTWIRE_ALLOWED_ORIGINS")
        .unwrap_or_else(|_| "http://localhost:3001,http://localhost:5173".to_string())
        .split(',')
        .filter_map(|s| s.parse::<HeaderValue>().ok())
        .collect::<Vec<_>>();
    
    CorsLayer::new()
        .allow_origin(AllowOrigin::list(allowed_origins))
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
        ])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600))
}

let app = Router::new()
    // ...
    .layer(create_cors_layer());
```

Add to configuration:

```toml
# ghostwire.toml
[web]
# Comma-separated list of allowed origins
allowed_origins = "http://localhost:3001,https://ghostwire.example.com"
```

**Verification:**
```bash
# Start server
cargo run -- web

# Test from browser console:
fetch('http://localhost:3002/api/status', {
  headers: { 'Origin': 'http://evil.com' }
})
// Should fail with CORS error

fetch('http://localhost:3002/api/status', {
  headers: { 'Origin': 'http://localhost:3001' }
})
// Should succeed
```

---

## 🔥 Priority 1: High-Priority Improvements (Week 2-3)

### 6. Add API Authentication

**Issue:** No authentication on API endpoints
**Time:** 4 hours
**Risk:** HIGH - Unauthorized access

#### Step 6.1: Add JWT Dependency

```toml
# ghostwire/Cargo.toml
[dependencies]
jsonwebtoken = "9.0"
```

#### Step 6.2: Create JWT Service

Create `ghostwire/src/web/auth.rs`:

```rust
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use time::{Duration, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (user ID)
    pub exp: i64,     // Expiration
    pub iat: i64,     // Issued at
    pub role: String, // User role
}

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtService {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret.as_bytes()),
            decoding_key: DecodingKey::from_secret(secret.as_bytes()),
        }
    }
    
    pub fn generate_token(&self, user_id: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
        let now = OffsetDateTime::now_utc();
        let exp = now + Duration::hours(24);
        
        let claims = Claims {
            sub: user_id.to_string(),
            exp: exp.unix_timestamp(),
            iat: now.unix_timestamp(),
            role: role.to_string(),
        };
        
        encode(&Header::default(), &claims, &self.encoding_key)
    }
    
    pub fn verify_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let token_data = decode::<Claims>(
            token,
            &self.decoding_key,
            &Validation::default(),
        )?;
        
        Ok(token_data.claims)
    }
}

// Middleware for JWT authentication
pub async fn auth_middleware(
    State(jwt_service): State<Arc<JwtService>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    let claims = jwt_service
        .verify_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // Add claims to request extensions
    req.extensions_mut().insert(claims);
    
    Ok(next.run(req).await)
}
```

#### Step 6.3: Add Login Endpoint

```rust
// ghostwire/src/web.rs
use crate::web::auth::{JwtService, auth_middleware};

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    expires_in: i64,
}

async fn login(
    State(jwt_service): State<Arc<JwtService>>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    // Verify credentials (use proper password hashing!)
    if !verify_credentials(&req.username, &req.password).await {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    let token = jwt_service.generate_token(&req.username, "user")
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(LoginResponse {
        token,
        expires_in: 86400, // 24 hours
    }))
}

pub async fn start_web_server(core: Arc<Core>, host: String, port: u16) -> Result<()> {
    let jwt_secret = env::var("GHOSTWIRE_JWT_SECRET")
        .expect("GHOSTWIRE_JWT_SECRET environment variable must be set");
    
    let jwt_service = Arc::new(JwtService::new(&jwt_secret));
    
    let app = Router::new()
        // Public endpoints
        .route("/api/login", post(login))
        .route("/api/health", get(health_check))
        
        // Protected endpoints
        .route("/api/status", get(get_status))
        .route("/api/peers", get(get_peers))
        .route("/api/send_message", post(send_message))
        .layer(axum::middleware::from_fn_with_state(
            jwt_service.clone(),
            auth_middleware
        ))
        
        .with_state(core)
        .layer(create_cors_layer())
        .layer(TraceLayer::new_for_http());
    
    // ... rest of server setup
}

async fn verify_credentials(username: &str, password: &str) -> bool {
    // TODO: Implement proper credential verification
    // Use argon2 for password hashing:
    use argon2::{Argon2, PasswordHash, PasswordVerifier};
    
    // Load user from database
    let stored_hash = get_user_password_hash(username).await.ok()?;
    
    let parsed_hash = PasswordHash::new(&stored_hash).ok()?;
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}
```

#### Step 6.4: Update Frontend to Use JWT

```typescript
// webui/src/services/auth.ts
export class AuthService {
    private token: string | null = null;
    
    async login(username: string, password: string): Promise<void> {
        const response = await fetch(`${config.API_BASE_URL}/login`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ username, password }),
        });
        
        if (!response.ok) {
            throw new Error('Login failed');
        }
        
        const data = await response.json();
        this.token = data.token;
        localStorage.setItem('ghostwire_token', data.token);
    }
    
    logout(): void {
        this.token = null;
        localStorage.removeItem('ghostwire_token');
    }
    
    getToken(): string | null {
        if (!this.token) {
            this.token = localStorage.getItem('ghostwire_token');
        }
        return this.token;
    }
    
    isAuthenticated(): boolean {
        const token = this.getToken();
        if (!token) return false;
        
        // Check if token is expired
        try {
            const payload = JSON.parse(atob(token.split('.')[1]));
            return payload.exp * 1000 > Date.now();
        } catch {
            return false;
        }
    }
}

export const authService = new AuthService();

// Update API client to include token
export async function authenticatedFetch(url: string, options: RequestInit = {}): Promise<Response> {
    const token = authService.getToken();
    
    const headers = {
        ...options.headers,
        'Authorization': token ? `Bearer ${token}` : '',
    };
    
    const response = await fetch(url, { ...options, headers });
    
    if (response.status === 401) {
        // Token expired, redirect to login
        authService.logout();
        window.location.href = '/login';
    }
    
    return response;
}
```

**Verification:**
```bash
# Test login
curl -X POST http://localhost:3002/api/login \
  -H "Content-Type: application/json" \
  -d '{"username":"test","password":"test"}'
# Returns: {"token":"eyJ...","expires_in":86400}

# Test protected endpoint without token
curl http://localhost:3002/api/status
# Returns: 401 Unauthorized

# Test with token
TOKEN="eyJ..."
curl http://localhost:3002/api/status \
  -H "Authorization: Bearer $TOKEN"
# Returns: {"status":"ok",...}
```

---

### 7. Add Comprehensive Test Suite

**Issue:** <10% test coverage
**Time:** 16 hours
**Risk:** HIGH - Bugs in production

#### Step 7.1: Set Up Test Infrastructure

```toml
# ghostwire/Cargo.toml
[dev-dependencies]
tokio-test = "0.4"
mockall = "0.12"
proptest = "1.0"
criterion = "0.5"  # For benchmarks
```

#### Step 7.2: Crypto Tests

Create `ghostwire/tests/encryption_test.rs`:

```rust
use ghostwire::core::encryption::{EncryptionManager, KeyType, MessageType};
use std::time::SystemTime;

#[test]
fn test_key_generation() {
    let manager = EncryptionManager::new().unwrap();
    
    let key_ed25519 = manager.generate_key(KeyType::Ed25519).unwrap();
    assert_eq!(key_ed25519.len(), 32);
    
    let key_x25519 = manager.generate_key(KeyType::X25519).unwrap();
    assert_eq!(key_x25519.len(), 32);
}

#[test]
fn test_encryption_roundtrip() {
    let manager = EncryptionManager::new().unwrap();
    let plaintext = b"test secret message";
    
    let encrypted = manager.encrypt_message(
        plaintext,
        "sender123",
        "recipient456",
        MessageType::Text
    ).unwrap();
    
    let decrypted = manager.decrypt_message(&encrypted).unwrap();
    assert_eq!(plaintext, &decrypted[..]);
}

#[test]
fn test_signature_verification() {
    let manager = EncryptionManager::new().unwrap();
    let message = b"test message";
    
    let signature = manager.sign(message).unwrap();
    assert!(manager.verify(message, &signature, &manager.key_pair.public_key).is_ok());
    
    // Tampered message should fail
    let tampered = b"tampered message";
    assert!(manager.verify(tampered, &signature, &manager.key_pair.public_key).is_err());
}

#[test]
fn test_key_rotation() {
    let mut manager = EncryptionManager::new().unwrap();
    let old_key_id = manager.key_pair.key_id.clone();
    
    manager.rotate_keys().unwrap();
    
    assert_ne!(old_key_id, manager.key_pair.key_id);
    assert!(manager.key_pair.created_at > 0);
}

#[test]
fn test_key_expiration() {
    let mut manager = EncryptionManager::new().unwrap();
    
    // Set expiration to 1 second ago
    let past = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() - 1;
    
    manager.key_pair.expires_at = Some(past);
    
    assert!(manager.is_key_expired());
}

// Property-based test: encryption is deterministic with same inputs
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_encryption_deterministic(data in prop::collection::vec(any::<u8>(), 1..1000)) {
        let manager = EncryptionManager::new().unwrap();
        
        let encrypted1 = manager.encrypt_message(
            &data,
            "sender",
            "recipient",
            MessageType::Binary
        ).unwrap();
        
        let encrypted2 = manager.encrypt_message(
            &data,
            "sender",
            "recipient",
            MessageType::Binary
        ).unwrap();
        
        // Ciphertexts should differ (due to random nonce)
        // but both should decrypt to same plaintext
        let decrypted1 = manager.decrypt_message(&encrypted1).unwrap();
        let decrypted2 = manager.decrypt_message(&encrypted2).unwrap();
        
        prop_assert_eq!(decrypted1, decrypted2);
        prop_assert_eq!(decrypted1, data);
    }
}
```

#### Step 7.3: Mesh Networking Tests

Create `ghostwire/tests/mesh_test.rs`:

```rust
use ghostwire::core::{Core, mesh::MeshTransport};
use tokio::time::{sleep, Duration};

#[tokio::test]
async fn test_peer_discovery() {
    let node1 = start_test_node("node1", 8001).await;
    let node2 = start_test_node("node2", 8002).await;
    
    // Wait for mDNS discovery
    sleep(Duration::from_secs(2)).await;
    
    let peers = node1.get_discovered_peers().await.unwrap();
    assert!(peers.iter().any(|p| p.id == "node2"));
}

#[tokio::test]
async fn test_message_routing() {
    let node1 = start_test_node("node1", 8001).await;
    let node2 = start_test_node("node2", 8002).await;
    let node3 = start_test_node("node3", 8003).await;
    
    // Connect: node1 <-> node2 <-> node3
    node1.connect_to(&node2.addr).await.unwrap();
    node2.connect_to(&node3.addr).await.unwrap();
    
    // Send from node1 to node3
    let msg = "test message";
    node1.send_message(&node3.id, msg).await.unwrap();
    
    // Verify node3 receives it
    let received = tokio::time::timeout(
        Duration::from_secs(5),
        node3.wait_for_message()
    ).await.unwrap().unwrap();
    
    assert_eq!(received.content, msg);
    assert_eq!(received.sender, node1.id);
}

#[tokio::test]
async fn test_connection_resilience() {
    let node1 = start_test_node("node1", 8001).await;
    let node2 = start_test_node("node2", 8002).await;
    
    node1.connect_to(&node2.addr).await.unwrap();
    assert!(node1.is_connected(&node2.id).await);
    
    // Simulate node2 crash
    node2.shutdown().await.unwrap();
    sleep(Duration::from_secs(2)).await;
    
    assert!(!node1.is_connected(&node2.id).await);
    
    // Restart node2
    let node2_new = start_test_node("node2", 8002).await;
    sleep(Duration::from_secs(2)).await;
    
    // Should reconnect automatically
    assert!(node1.is_connected(&node2_new.id).await);
}

// Helper function
async fn start_test_node(name: &str, port: u16) -> TestNode {
    // ... implementation
}
```

#### Step 7.4: Frontend Tests

Create `webui/src/components/App.test.tsx`:

```typescript
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import '@testing-library/jest-dom';
import App from './App';
import { api } from './services/api';

// Mock API
jest.mock('./services/api');

describe('App Component', () => {
    beforeEach(() => {
        jest.clearAllMocks();
    });
    
    test('renders dashboard by default', () => {
        render(<App />);
        expect(screen.getByText(/dashboard/i)).toBeInTheDocument();
    });
    
    test('scans network when button clicked', async () => {
        const mockPeers = [
            { id: 'peer1', name: 'Peer 1', status: 'online' },
        ];
        
        (api.scanNetwork as jest.Mock).mockResolvedValue(mockPeers);
        
        render(<App />);
        
        const scanButton = screen.getByRole('button', { name: /scan/i });
        fireEvent.click(scanButton);
        
        await waitFor(() => {
            expect(api.scanNetwork).toHaveBeenCalled();
        });
        
        expect(screen.getByText('Peer 1')).toBeInTheDocument();
    });
    
    test('handles API errors gracefully', async () => {
        (api.scanNetwork as jest.Mock).mockRejectedValue(new Error('Network error'));
        
        render(<App />);
        
        const scanButton = screen.getByRole('button', { name: /scan/i });
        fireEvent.click(scanButton);
        
        await waitFor(() => {
            expect(screen.getByText(/error/i)).toBeInTheDocument();
        });
    });
    
    test('displays emergency mode when activated', () => {
        render(<App />);
        
        const emergencyButton = screen.getByRole('button', { name: /emergency/i });
        fireEvent.click(emergencyButton);
        
        expect(screen.getByText(/emergency mode active/i)).toBeInTheDocument();
    });
});
```

Configure Jest:

```json
// webui/package.json
{
  "scripts": {
    "test": "jest",
    "test:watch": "jest --watch",
    "test:coverage": "jest --coverage"
  },
  "jest": {
    "preset": "ts-jest",
    "testEnvironment": "jsdom",
    "setupFilesAfterEnv": ["<rootDir>/src/setupTests.ts"],
    "collectCoverageFrom": [
      "src/**/*.{ts,tsx}",
      "!src/**/*.d.ts",
      "!src/main.tsx"
    ],
    "coverageThresholds": {
      "global": {
        "branches": 50,
        "functions": 50,
        "lines": 50,
        "statements": 50
      }
    }
  },
  "devDependencies": {
    "@testing-library/react": "^14.0.0",
    "@testing-library/jest-dom": "^6.0.0",
    "@types/jest": "^29.0.0",
    "jest": "^29.0.0",
    "jest-environment-jsdom": "^29.0.0",
    "ts-jest": "^29.0.0"
  }
}
```

**Run tests:**
```bash
# Rust tests
cd ghostwire
cargo test
cargo test --all-features
cargo tarpaulin --out Html  # Generate coverage report

# Frontend tests
cd ../webui
npm test
npm run test:coverage
```

**Set coverage requirement in CI:**

```yaml
# .github/workflows/rust-ci.yml
- name: Run tests with coverage
  run: |
    cargo install cargo-tarpaulin
    cargo tarpaulin --out Xml --all-features
    
- name: Upload coverage
  uses: codecov/codecov-action@v3
  with:
    files: ./cobertura.xml
    fail_ci_if_error: true
```

---

## 📋 Verification Checklist

After completing P0 and P1 fixes:

```bash
# Security
- [ ] cargo audit shows 0 vulnerabilities
- [ ] npm audit shows 0 vulnerabilities
- [ ] No hardcoded passwords in code
- [ ] No secrets in git history
- [ ] CORS properly configured
- [ ] API authentication working

# Testing
- [ ] cargo test passes (100%)
- [ ] npm test passes (100%)
- [ ] Test coverage > 50%
- [ ] All critical paths tested

# Code Quality
- [ ] cargo clippy shows 0 warnings
- [ ] cargo fmt applied
- [ ] ESLint passes (frontend)
- [ ] No TODO in security-critical code

# Legal
- [ ] LICENSE file present
- [ ] All dependencies compatible with AGPL-3.0
```

---

## 🎯 Next Steps

After completing P0 and P1, proceed to:
- **Priority 2:** Tor integration, Bluetooth mesh, onion routing (see COMPREHENSIVE_CODE_REVIEW.md)
- **Documentation:** Generate rustdoc, write architecture docs
- **Performance:** Benchmarking, optimization
- **Security Audit:** Third-party penetration testing

---

**Questions?** Open an issue or discussion on GitHub.
