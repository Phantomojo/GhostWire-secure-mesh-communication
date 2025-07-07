use std::{io, net::SocketAddr, sync::{Arc, Mutex}, time::{Duration, Instant}};
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
use tokio::time::timeout;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{info, warn, error, debug};
use crate::core::security::{SecurityManager, ThreatLevel, SecurityEvent, EntityType};
use serde::{Serialize, Deserialize};

// Enhanced stealth handshake with multiple layers
const MAGIC_BYTES: &[u8] = b"GWSTH"; // GhostWire Stealth Handshake
const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(1);
const MAX_CONN_PER_MIN: usize = 10;

// Advanced stealth features
const STEALTH_VERSION: u8 = 0x01;
const MAX_HANDSHAKE_ATTEMPTS: usize = 3;
const CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);

/// Enhanced StealthTCP provider with advanced security features
#[derive(Clone)]
pub struct StealthTCPProvider {
    rate_limiter: Arc<Mutex<HashMap<SocketAddr, (usize, Instant)>>>,
    allowlist: Option<Vec<std::net::IpAddr>>,
    security_manager: Arc<SecurityManager>,
    handshake_secret: Vec<u8>,
    enable_stealth_mode: bool,
    connection_history: Arc<Mutex<HashMap<SocketAddr, ConnectionInfo>>>,
}

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub first_seen: Instant,
    pub last_seen: Instant,
    pub total_connections: usize,
    pub successful_connections: usize,
    pub failed_attempts: usize,
    pub average_latency: Duration,
    pub threat_score: f32,
}

impl StealthTCPProvider {
    pub fn new(
        allowlist: Option<Vec<std::net::IpAddr>>,
        security_manager: Arc<SecurityManager>,
        handshake_secret: Option<Vec<u8>>,
        enable_stealth_mode: bool,
    ) -> Self {
        let secret = handshake_secret.unwrap_or_else(|| {
            // Generate a cryptographically secure random secret
            use ring::rand::{SecureRandom, SystemRandom};
            let rng = SystemRandom::new();
            let mut secret = vec![0u8; 32];
            rng.fill(&mut secret).unwrap();
            secret
        });

        Self {
            rate_limiter: Arc::new(Mutex::new(HashMap::new())),
            allowlist,
            security_manager,
            handshake_secret: secret,
            enable_stealth_mode,
            connection_history: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn check_rate_limit(&self, addr: &SocketAddr) -> bool {
        let mut rl = self.rate_limiter.lock().unwrap();
        let now = Instant::now();
        let entry = rl.entry(*addr).or_insert((0, now));
        
        if now.duration_since(entry.1) > Duration::from_secs(60) {
            entry.0 = 1;
            entry.1 = now;
            true
        } else if entry.0 < MAX_CONN_PER_MIN {
            entry.0 += 1;
            true
        } else {
            false
        }
    }

    fn check_allowlist(&self, addr: &SocketAddr) -> bool {
        match &self.allowlist {
            Some(list) => list.contains(&addr.ip()),
            None => true,
        }
    }

    fn update_connection_history(&self, addr: SocketAddr, success: bool, latency: Duration) {
        let mut history = self.connection_history.lock().unwrap();
        let now = Instant::now();
        
        let entry = history.entry(addr).or_insert(ConnectionInfo {
            first_seen: now,
            last_seen: now,
            total_connections: 0,
            successful_connections: 0,
            failed_attempts: 0,
            average_latency: Duration::ZERO,
            threat_score: 0.0,
        });

        entry.last_seen = now;
        entry.total_connections += 1;
        
        if success {
            entry.successful_connections += 1;
            // Update average latency
            let total_latency = entry.average_latency * (entry.successful_connections - 1) as u32 + latency;
            entry.average_latency = total_latency / entry.successful_connections as u32;
        } else {
            entry.failed_attempts += 1;
        }

        // Calculate threat score based on connection patterns
        entry.threat_score = self.calculate_threat_score(entry);
    }

    fn calculate_threat_score(&self, info: &ConnectionInfo) -> f32 {
        let failure_rate = if info.total_connections > 0 {
            info.failed_attempts as f32 / info.total_connections as f32
        } else {
            0.0
        };

        let connection_frequency = info.total_connections as f32 / 
            info.first_seen.elapsed().as_secs().max(1) as f32;

        let latency_anomaly = if info.average_latency > Duration::from_millis(1000) {
            1.0
        } else {
            0.0
        };

        // Weighted threat score calculation
        failure_rate * 0.4 + connection_frequency * 0.3 + latency_anomaly * 0.3
    }

    /// Enhanced stealth handshake with multiple verification layers
    async fn perform_stealth_handshake(&self, stream: &mut TcpStream) -> io::Result<bool> {
        let start_time = Instant::now();
        
        // Layer 1: Version check
        let mut version_buf = [0u8; 1];
        if timeout(HANDSHAKE_TIMEOUT, stream.read_exact(&mut version_buf)).await.is_err() {
            return Ok(false);
        }
        
        if version_buf[0] != STEALTH_VERSION {
            return Ok(false);
        }

        // Layer 2: Magic bytes
        let mut magic_buf = [0u8; MAGIC_BYTES.len()];
        if timeout(HANDSHAKE_TIMEOUT, stream.read_exact(&mut magic_buf)).await.is_err() {
            return Ok(false);
        }
        
        if &magic_buf != MAGIC_BYTES {
            return Ok(false);
        }

        // Layer 3: Challenge-response (if stealth mode enabled)
        if self.enable_stealth_mode {
            if !self.perform_challenge_response(stream).await? {
                return Ok(false);
            }
        }

        let handshake_time = start_time.elapsed();
        debug!("Stealth handshake completed in {:?}", handshake_time);
        
        Ok(true)
    }

    /// Challenge-response authentication for enhanced security
    async fn perform_challenge_response(&self, stream: &mut TcpStream) -> io::Result<bool> {
        // Generate a random challenge
        use ring::rand::{SecureRandom, SystemRandom};
        let rng = SystemRandom::new();
        let mut challenge = [0u8; 16];
        rng.fill(&mut challenge).map_err(|_| {
            io::Error::new(io::ErrorKind::Other, "Failed to generate challenge")
        })?;

        // Send challenge
        stream.write_all(&challenge).await?;

        // Receive response
        let mut response = [0u8; 32];
        if timeout(HANDSHAKE_TIMEOUT, stream.read_exact(&mut response)).await.is_err() {
            return Ok(false);
        }

        // Verify response (HMAC of challenge with secret)
        let expected_response = self.calculate_challenge_response(&challenge);
        Ok(response == expected_response)
    }

    fn calculate_challenge_response(&self, challenge: &[u8]) -> [u8; 32] {
        use ring::hmac::{Key, HMAC_SHA256};
        let key = Key::new(HMAC_SHA256, &self.handshake_secret);
        let signature = ring::hmac::sign(&key, challenge);
        let mut response = [0u8; 32];
        response.copy_from_slice(signature.as_ref());
        response
    }

    pub async fn accept(&self, listener: &TcpListener) -> io::Result<TcpStream> {
        let mut attempts = 0;
        
        loop {
            attempts += 1;
            if attempts > MAX_HANDSHAKE_ATTEMPTS {
                error!("StealthTCP: Too many failed handshake attempts");
                return Err(io::Error::new(io::ErrorKind::ConnectionRefused, "Too many failed attempts"));
            }

            let (mut stream, addr) = listener.accept().await?;
            let connection_start = Instant::now();

            // Security checks
            if !self.security_manager.is_ip_allowed(&addr.ip()) {
                self.security_manager.record_connection_attempt(
                    addr.ip(),
                    false,
                    "IP not allowed".to_string(),
                );
                continue;
            }

            if !self.check_allowlist(&addr) {
                warn!("StealthTCP: Connection from {} rejected (not in allowlist)", addr);
                self.security_manager.record_connection_attempt(
                    addr.ip(),
                    false,
                    "Not in allowlist".to_string(),
                );
                continue;
            }

            if !self.check_rate_limit(&addr) {
                warn!("StealthTCP: Connection from {} rate-limited", addr);
                self.security_manager.record_connection_attempt(
                    addr.ip(),
                    false,
                    "Rate limited".to_string(),
                );
                continue;
            }

            // Perform stealth handshake
            match self.perform_stealth_handshake(&mut stream).await {
                Ok(true) => {
                    let latency = connection_start.elapsed();
                    self.update_connection_history(addr, true, latency);
                    
                    info!("StealthTCP: Accepted stealth connection from {} (latency: {:?})", addr, latency);
                    self.security_manager.record_connection_attempt(
                        addr.ip(),
                        true,
                        "Stealth handshake successful".to_string(),
                    );
                    return Ok(stream);
                }
                Ok(false) => {
                    let latency = connection_start.elapsed();
                    self.update_connection_history(addr, false, latency);
                    
                    warn!("StealthTCP: Connection from {} failed stealth handshake", addr);
                    self.security_manager.record_connection_attempt(
                        addr.ip(),
                        false,
                        "Stealth handshake failed".to_string(),
                    );
                    continue;
                }
                Err(e) => {
                    let latency = connection_start.elapsed();
                    self.update_connection_history(addr, false, latency);
                    
                    error!("StealthTCP: Handshake error from {}: {}", addr, e);
                    self.security_manager.record_connection_attempt(
                        addr.ip(),
                        false,
                        format!("Handshake error: {}", e),
                    );
                    continue;
                }
            }
        }
    }

    pub async fn connect(&self, addr: &SocketAddr) -> io::Result<TcpStream> {
        let connection_start = Instant::now();
        
        // Security check for outbound connections
        if !self.security_manager.is_ip_allowed(&addr.ip()) {
            return Err(io::Error::new(io::ErrorKind::ConnectionRefused, "IP not allowed"));
        }

        let mut stream = TcpStream::connect(addr).await?;
        
        // Perform outbound stealth handshake
        if !self.perform_outbound_handshake(&mut stream).await? {
            return Err(io::Error::new(io::ErrorKind::ConnectionRefused, "Stealth handshake failed"));
        }

        let latency = connection_start.elapsed();
        self.update_connection_history(*addr, true, latency);
        
        info!("StealthTCP: Connected to {} (latency: {:?})", addr, latency);
        Ok(stream)
    }

    async fn perform_outbound_handshake(&self, stream: &mut TcpStream) -> io::Result<bool> {
        // Send version
        stream.write_all(&[STEALTH_VERSION]).await?;

        // Send magic bytes
        stream.write_all(MAGIC_BYTES).await?;

        // Handle challenge-response if stealth mode enabled
        if self.enable_stealth_mode {
            if !self.handle_outbound_challenge_response(stream).await? {
                return Ok(false);
            }
        }

        Ok(true)
    }

    async fn handle_outbound_challenge_response(&self, stream: &mut TcpStream) -> io::Result<bool> {
        // Receive challenge
        let mut challenge = [0u8; 16];
        if timeout(HANDSHAKE_TIMEOUT, stream.read_exact(&mut challenge)).await.is_err() {
            return Ok(false);
        }

        // Calculate and send response
        let response = self.calculate_challenge_response(&challenge);
        stream.write_all(&response).await?;

        Ok(true)
    }

    /// Get connection statistics
    pub fn get_connection_stats(&self) -> ConnectionStats {
        let history = self.connection_history.lock().unwrap();
        let total_connections = history.values().map(|info| info.total_connections as u64).sum();
        let active_connections = history.values().filter(|info| info.successful_connections > 0).count() as u64;
        let failed_connections = history.values().map(|info| info.failed_attempts as u64).sum();
        let blocked_connections = history.values().filter(|info| info.threat_score > 0.7).count() as u64;
        
        let avg_connection_time = if total_connections > 0 {
            let total_time: f64 = history.values()
                .map(|info| info.average_latency.as_millis() as f64)
                .sum();
            total_time / total_connections as f64
        } else {
            0.0
        };
        
        let last_connection_time = history.values()
            .map(|info| info.last_seen.duration_since(Instant::now()).as_secs() as u64)
            .max()
            .unwrap_or(0);
        
        ConnectionStats {
            total_connections,
            active_connections,
            failed_connections,
            blocked_connections,
            avg_connection_time,
            last_connection_time,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStats {
    pub total_connections: u64,
    pub active_connections: u64,
    pub failed_connections: u64,
    pub blocked_connections: u64,
    pub avg_connection_time: f64,
    pub last_connection_time: u64,
}

impl Default for ConnectionStats {
    fn default() -> Self {
        Self {
            total_connections: 0,
            active_connections: 0,
            failed_connections: 0,
            blocked_connections: 0,
            avg_connection_time: 0.0,
            last_connection_time: 0,
        }
    }
} 