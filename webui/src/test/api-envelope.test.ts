import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import { messages, network, peers } from '../services/api';

/**
 * Regression tests for the API envelope contract.
 *
 * The daemon signals some failures with an `error` field in the body. Before
 * `ApiV2Response::error_response` was introduced, several handlers returned
 * HTTP **200** alongside an error envelope, so a client that trusted the status
 * code alone would report a failure as a success. `request()` therefore checks
 * BOTH conditions, and these tests pin that down:
 *
 *   if (!res.ok || envelope.error)
 *
 * A 200-with-error response must still throw. If someone "simplifies" this to
 * `if (!res.ok)`, the first test below fails.
 */

type Envelope = { data?: unknown; error?: { code: string; message: string } };

function mockFetch(
  status: number,
  body: Envelope | string,
  contentType = 'application/json',
) {
  return vi.fn(async () => ({
    ok: status >= 200 && status < 300,
    status,
    statusText: `status ${status}`,
    headers: { get: (h: string) => (h.toLowerCase() === 'content-type' ? contentType : null) },
    json: async () => (typeof body === 'string' ? JSON.parse(body) : body),
    text: async () => (typeof body === 'string' ? body : JSON.stringify(body)),
  }));
}

describe('api envelope contract', () => {
  let fetchMock: ReturnType<typeof vi.fn>;

  beforeEach(() => {
    fetchMock = vi.fn();
    vi.stubGlobal('fetch', fetchMock);
  });

  afterEach(() => {
    vi.unstubAllGlobals();
  });

  it('throws on HTTP 200 that carries an error envelope (the regression)', async () => {
    // This is the exact shape the daemon used to return for a failed send.
    fetchMock.mockImplementation(
      mockFetch(200, {
        error: { code: 'INTERNAL_ERROR', message: 'Transmission failed: Invalid P2P Recipient ID' },
      }),
    );

    await expect(messages.send('deadbeef', 'hi')).rejects.toThrow(
      /Transmission failed/,
    );
  });

  it('attaches the machine-readable code to the thrown error', async () => {
    fetchMock.mockImplementation(
      mockFetch(200, { error: { code: 'INTERNAL_ERROR', message: 'boom' } }),
    );

    const err = await messages.send('peer', 'hi').catch((e) => e);
    expect(err).toBeInstanceOf(Error);
    expect((err as Error & { code?: string }).code).toBe('INTERNAL_ERROR');
  });

  it('returns data on a normal 2xx success', async () => {
    fetchMock.mockImplementation(mockFetch(200, { data: { version: '0.2.0' } }));

    await expect(network.version()).resolves.toEqual({ version: '0.2.0' });
  });

  it('throws on a 4xx even when the body has no error field', async () => {
    fetchMock.mockImplementation(mockFetch(403, { data: null }));

    await expect(network.version()).rejects.toThrow();
  });

  it('rejects a non-JSON response rather than returning garbage', async () => {
    fetchMock.mockImplementation(mockFetch(200, '<html>502 Bad Gateway</html>', 'text/html'));

    await expect(network.version()).rejects.toThrow(/Non-JSON response/);
  });

  it('attaches the auth token to the request when one is stored', async () => {
    sessionStorage.setItem('gw_token', 'test-token-value');
    fetchMock.mockImplementation(mockFetch(200, { data: {} }));

    await network.version();

    const [, init] = fetchMock.mock.calls[0];
    const headers = (init as RequestInit).headers as Record<string, string>;
    expect(JSON.stringify(headers)).toContain('test-token-value');
  });
});
