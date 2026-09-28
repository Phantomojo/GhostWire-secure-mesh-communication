import '@testing-library/jest-dom/vitest';

// jsdom does not implement these, and components under test touch them.
if (!window.matchMedia) {
  window.matchMedia = ((query: string) => ({
    matches: false,
    media: query,
    onchange: null,
    addListener: () => {},
    removeListener: () => {},
    addEventListener: () => {},
    removeEventListener: () => {},
    dispatchEvent: () => false,
  })) as unknown as typeof window.matchMedia;
}

// Fail loudly if a test forgets to stub the API, rather than letting a real
// fetch escape to localhost:9097 and hang or silently pass.
beforeEach(() => {
  sessionStorage.clear();
});
