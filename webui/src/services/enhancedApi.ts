// Enhanced API Service - Provides better simulated responses
// This simulates the enhanced backend endpoints we implemented

// Enhanced API Service - Provides better simulated responses
// This simulates the enhanced backend endpoints we implemented

// API Response type
interface ApiResponse {
  success: boolean;
  data: any;
  error: string | null;
}

// Enhanced broadcast response
export const enhancedBroadcastResponse = (message: string): ApiResponse => {
  const messageId = crypto.randomUUID();
  const timestamp = Math.floor(Date.now() / 1000);
  const recipients = Math.floor(Math.random() * 10) + 5;
  
  return {
    success: true,
    data: {
      message_id: messageId,
      recipients,
      delivered_to: Array.from({ length: recipients }, (_, i) => `peer${i + 1}:delivered`),
      encrypted: true,
      encryption_algorithm: "AES-256-GCM",
      timestamp,
      broadcast_type: "mesh_network",
      network_coverage: (recipients / 10.0),
      message_size: message.length,
      delivery_time_ms: Math.floor(Math.random() * 200) + 100,
    },
    error: null,
  };
};

// Enhanced security scan response
export const enhancedSecurityScanResponse = (): ApiResponse => {
  const scanId = crypto.randomUUID();
  const threats = Math.random() > 0.7 ? 1 : 0;
  const vulnerabilities = Math.floor(Math.random() * 3) + 1;
  
  return {
    success: true,
    data: {
      scan_id: scanId,
      scan_type: "Comprehensive Security Scan",
      duration_ms: Math.floor(Math.random() * 1000) + 1500,
      threats_found: threats,
      vulnerabilities_found: vulnerabilities,
      threats: threats > 0 ? ["Suspicious network activity detected"] : [],
      vulnerabilities: [
        "Weak encryption key detected",
        "Outdated security protocol",
        "Missing security patches"
      ].slice(0, vulnerabilities),
      scan_status: "COMPLETED",
      timestamp: new Date().toISOString(),
      security_score: threats > 0 ? 75 : 95,
      recommendations: [
        "Rotate encryption keys",
        "Update security protocols",
        "Enable additional monitoring"
      ],
      scan_areas: [
        "Network Security",
        "Encryption",
        "Authentication",
        "Firewall Rules",
        "Peer Connections"
      ],
    },
    error: null,
  };
};

// Enhanced threat hunt response
export const enhancedThreatHuntResponse = (): ApiResponse => {
  const huntId = crypto.randomUUID();
  const threats = Math.random() > 0.8 ? 2 : 0;
  
  return {
    success: true,
    data: {
      hunt_id: huntId,
      hunt_type: "Proactive Threat Hunting",
      duration_ms: Math.floor(Math.random() * 1000) + 2000,
      threats_found: threats,
      indicators_found: 3,
      suspicious_ips: ["192.168.1.100", "10.0.0.50"],
      threats: threats > 0 ? [
        "Advanced Persistent Threat (APT) detected",
        "Data exfiltration attempt blocked"
      ] : [],
      indicators: [
        "Unusual network traffic patterns",
        "Suspicious file access patterns",
        "Anomalous user behavior detected"
      ],
      hunt_status: "COMPLETED",
      timestamp: new Date().toISOString(),
      threat_level: threats > 0 ? "HIGH" : "LOW",
      confidence_score: threats > 0 ? 0.95 : 0.85,
      recommendations: [
        "Implement additional monitoring",
        "Review network access logs",
        "Update threat intelligence feeds"
      ],
      hunt_techniques: [
        "Behavioral Analysis",
        "Network Traffic Analysis",
        "Endpoint Detection",
        "Threat Intelligence Correlation"
      ],
    },
    error: null,
  };
};

// Enhanced security audit response
export const enhancedSecurityAuditResponse = (): ApiResponse => {
  const auditId = crypto.randomUUID();
  const complianceIssues = Math.random() > 0.6 ? 1 : 0;
  const overallScore = complianceIssues > 0 ? 85 : 95;
  
  return {
    success: true,
    data: {
      audit_id: auditId,
      audit_type: "Comprehensive Security Audit",
      duration_ms: Math.floor(Math.random() * 1000) + 2500,
      overall_score: overallScore,
      grade: overallScore >= 90 ? "A" : overallScore >= 80 ? "B" : "C",
      findings_count: 3,
      recommendations_count: 4,
      compliance_issues_count: complianceIssues,
      findings: [
        "Strong encryption implementation",
        "Proper key management procedures",
        "Network segmentation in place"
      ],
      recommendations: [
        "Implement automated key rotation",
        "Add additional monitoring layers",
        "Conduct regular penetration testing",
        "Update security policies"
      ],
      compliance_issues: complianceIssues > 0 ? ["Key rotation schedule needs updating"] : [],
      audit_status: "COMPLETED",
      timestamp: new Date().toISOString(),
      risk_level: overallScore >= 90 ? "LOW" : overallScore >= 80 ? "MEDIUM" : "HIGH",
      next_audit_date: new Date(Date.now() + 90 * 24 * 60 * 60 * 1000).toISOString(),
      audit_areas: [
        "Network Security",
        "Encryption & Key Management",
        "Access Control",
        "Compliance",
        "Incident Response"
      ],
    },
    error: null,
  };
};

// Enhanced communication analysis response
export const enhancedCommunicationAnalysisResponse = (): ApiResponse => {
  const analysisId = crypto.randomUUID();
  const anomalies = Math.random() > 0.7 ? 1 : 0;
  
  return {
    success: true,
    data: {
      analysis_id: analysisId,
      analysis_type: "Comprehensive Communication Analysis",
      duration_ms: Math.floor(Math.random() * 1000) + 1800,
      patterns_found: 3,
      anomalies_detected: anomalies,
      insights_generated: 4,
      patterns: [
        "Regular peer communication every 5 minutes",
        "Encrypted message volume: 150 messages/hour",
        "Peak communication times: 9AM-11AM, 2PM-4PM"
      ],
      anomalies: anomalies > 0 ? [
        "Unusual communication spike detected",
        "Suspicious message pattern from peer 'alice'"
      ] : [],
      insights: [
        "Network health: Excellent (98% uptime)",
        "Average message delivery time: 45ms",
        "Encryption coverage: 100%",
        "Peer reliability score: 94/100"
      ],
      analysis_status: "COMPLETED",
      timestamp: new Date().toISOString(),
      network_health_score: anomalies > 0 ? 85 : 98,
      communication_volume: "150 messages/hour",
      encryption_coverage: 100.0,
      peer_activity_summary: [
        "alice: 45 messages, 98% uptime",
        "bob: 32 messages, 95% uptime",
        "charlie: 28 messages, 92% uptime"
      ],
    },
    error: null,
  };
};

// Enhanced API service that provides better responses
export const enhancedApiService = {
  // Enhanced broadcast
  broadcast: async (message: string): Promise<ApiResponse> => {
    // Simulate network delay
    await new Promise(resolve => setTimeout(resolve, Math.random() * 500 + 200));
    return enhancedBroadcastResponse(message);
  },

  // Enhanced security scan
  securityScan: async (): Promise<ApiResponse> => {
    // Simulate scan duration
    await new Promise(resolve => setTimeout(resolve, Math.random() * 1000 + 1500));
    return enhancedSecurityScanResponse();
  },

  // Enhanced threat hunt
  threatHunt: async (): Promise<ApiResponse> => {
    // Simulate hunt duration
    await new Promise(resolve => setTimeout(resolve, Math.random() * 1000 + 2000));
    return enhancedThreatHuntResponse();
  },

  // Enhanced security audit
  securityAudit: async (): Promise<ApiResponse> => {
    // Simulate audit duration
    await new Promise(resolve => setTimeout(resolve, Math.random() * 1000 + 2500));
    return enhancedSecurityAuditResponse();
  },

  // Enhanced communication analysis
  analyzeCommunications: async (): Promise<ApiResponse> => {
    // Simulate analysis duration
    await new Promise(resolve => setTimeout(resolve, Math.random() * 1000 + 1800));
    return enhancedCommunicationAnalysisResponse();
  },
}; 