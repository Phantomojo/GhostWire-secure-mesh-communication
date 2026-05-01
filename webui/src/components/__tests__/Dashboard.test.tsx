import { render, screen, fireEvent } from '@testing-library/react';
import { vi, describe, it, expect } from 'vitest';
import Dashboard from '../Dashboard';

describe('Dashboard', () => {
    const mockProps = {
        systemStatus: {
            cpu: 45,
            memory: 60,
            network: 15,
            encryption: 100,
            uptime: 3600,
            connections: 5,
            messages: 120,
            securityScore: 85,
        },
        peerCount: 12,
        threatCount: 0,
        backendConnected: true,
        onScanNetwork: vi.fn(),
        onBroadcast: vi.fn(),
        onSync: vi.fn(),
        onEmergencyMode: vi.fn(),
        onNavigate: vi.fn(),
        isEmergencyMode: false,
        pqcEnabled: true,
    };

    it('renders "SECURE" status when everything is fine', () => {
        render(<Dashboard {...mockProps} />);
        expect(screen.getByText('SECURE')).toBeInTheDocument();
        expect(screen.getByText('Mesh Operational')).toBeInTheDocument();
    });

    it('renders "WARNING" status when threats are detected', () => {
        render(<Dashboard {...mockProps} threatCount={2} />);
        expect(screen.getByText('WARNING')).toBeInTheDocument();
        expect(screen.getByText('Threats Detected')).toBeInTheDocument();
    });

    it('renders "OFFLINE" status when backend is disconnected', () => {
        render(<Dashboard {...mockProps} backendConnected={false} />);
        expect(screen.getByText('OFFLINE')).toBeInTheDocument();
        expect(screen.getByText('Connection Lost')).toBeInTheDocument();
    });

    it('renders "EMERGENCY" status during lockdown', () => {
        render(<Dashboard {...mockProps} isEmergencyMode={true} />);
        expect(screen.getByText('EMERGENCY')).toBeInTheDocument();
        expect(screen.getAllByText('Protocol Active')).toHaveLength(2);
    });

    it('calls onScanNetwork when Scan button is clicked', () => {
        render(<Dashboard {...mockProps} />);
        const scanButton = screen.getByText(/Scan Sector/i);
        fireEvent.click(scanButton);
        expect(mockProps.onScanNetwork).toHaveBeenCalled();
    });

    it('shows PQC enabled information', () => {
        render(<Dashboard {...mockProps} pqcEnabled={true} />);
        expect(screen.getByText('Kyber-1024')).toBeInTheDocument();
    });

    it('shows system metrics', () => {
        render(<Dashboard {...mockProps} />);
        expect(screen.getByText('45%')).toBeInTheDocument(); // CPU
        expect(screen.getByText('60%')).toBeInTheDocument(); // RAM
        expect(screen.getByText('1.0h')).toBeInTheDocument(); // Uptime (3600s)
    });
});
