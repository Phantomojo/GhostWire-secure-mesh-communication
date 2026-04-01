import { useEffect } from 'react';
import { useApp, type ActivePanel } from '../context/AppContext';

const SHORTCUTS: Record<string, ActivePanel> = {
  '1': 'dashboard',
  '2': 'communication',
  '3': 'channels',
  '4': 'network',
  '5': 'ark',
  '6': 'security',
  '7': 'system',
  '8': 'settings',
};

export function useKeyboardShortcuts() {
  const { dispatch } = useApp();

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      // Don't fire when typing in an input
      if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) return;

      if (e.altKey && SHORTCUTS[e.key]) {
        e.preventDefault();
        dispatch({ type: 'SET_PANEL', payload: SHORTCUTS[e.key] });
      }
    };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, [dispatch]);
}
