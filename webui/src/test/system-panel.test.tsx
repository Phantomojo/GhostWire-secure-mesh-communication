import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/react';
import SystemPanel from '../components/panels/System';

/**
 * The System panel used to display CPU and memory percentages that the daemon
 * never reports. With `cpuPct ?? 0` it rendered a confident, fully-styled
 * "0%" — a fabricated measurement, not a missing one. The values are now
 * nullable and render as "n/a".
 *
 * These tests pin that down. Reintroducing `?? 0` makes the second test fail,
 * because "0%" and "n/a" are both rendered but only one of them is honest.
 */

const refreshMesh = vi.fn();

// Mutable so each test can choose what the daemon "reported".
let currentState: Record<string, unknown> = {};

vi.mock('../context/AppContext', () => ({
  useApp: () => ({ state: currentState, refreshMesh }),
}));

describe('SystemPanel telemetry honesty', () => {
  beforeEach(() => {
    refreshMesh.mockClear();
    // The panel reads transport/uptime/latency unconditionally, so a partial
    // mock makes it throw before reaching the CPU/memory markup under test.
    currentState = {
      meshHealth: null,
      cpuPct: null,
      memPct: null,
      latencyMs: null,
      uptimeSecs: 0,
      backendConnected: false,
      transport: { tier: 'offline', internet_reachable: false },
    };
  });

  it('renders "n/a" for CPU when the daemon does not report it', () => {
    render(<SystemPanel />);
    expect(screen.getAllByText('n/a').length).toBeGreaterThan(0);
  });

  it('does NOT render a fabricated 0% when the value is absent', () => {
    render(<SystemPanel />);
    // The exact regression: `state.cpuPct ?? 0` produced this string.
    expect(screen.queryByText('0%')).not.toBeInTheDocument();
  });

  it('renders the real percentages when the daemon supplies them', () => {
    // Proves the n/a branch is not simply dead code.
    currentState = {
      ...currentState,
      cpuPct: 42,
      memPct: 17,
    };
    render(<SystemPanel />);
    expect(screen.getByText('42%')).toBeInTheDocument();
    expect(screen.getByText('17%')).toBeInTheDocument();
    expect(screen.queryByText('0%')).not.toBeInTheDocument();
  });
});
