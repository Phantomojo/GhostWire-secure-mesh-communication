import React from 'react';
import { AppProvider, useApp } from './context/AppContext';
import Sidebar from './components/layout/Sidebar';
import Header from './components/layout/Header';
import Toast from './components/shared/Toast';
import MobileNav from './components/layout/MobileNav';
import { useKeyboardShortcuts } from './hooks/useKeyboardShortcuts';
import LoadingScreen from './components/shared/LoadingScreen';
import Dashboard from './components/panels/Dashboard';
import Channels from './components/panels/Channels';

// Lazy-importable panels (keep bundle lean)
const NetworkPanel    = React.lazy(() => import('./components/panels/Network'));
const CommunicationPanel = React.lazy(() => import('./components/panels/Communication'));
const ArkPanel        = React.lazy(() => import('./components/panels/Ark'));
const SecurityPanel   = React.lazy(() => import('./components/panels/Security'));
const SystemPanel     = React.lazy(() => import('./components/panels/System'));
const SettingsPanel   = React.lazy(() => import('./components/panels/Settings'));
const AboutPanel      = React.lazy(() => import('./components/panels/About'));
const LoginScreen     = React.lazy(() => import('./components/panels/Login'));

function PanelRouter() {
  const { state } = useApp();
  return (
    <React.Suspense fallback={
      <div style={{
        flex: 1, display: 'flex', alignItems: 'center', justifyContent: 'center',
        fontFamily: 'var(--mono)', fontSize: 11, color: 'var(--t3)',
      }}>
        Loading...
      </div>
    }>
      {state.activePanel === 'dashboard'     && <Dashboard />}
      {state.activePanel === 'channels'      && <Channels />}
      {state.activePanel === 'communication' && <CommunicationPanel />}
      {state.activePanel === 'network'       && <NetworkPanel />}
      {state.activePanel === 'ark'           && <ArkPanel />}
      {state.activePanel === 'security'      && <SecurityPanel />}
      {state.activePanel === 'system'        && <SystemPanel />}
      {state.activePanel === 'settings'      && <SettingsPanel />}
      {state.activePanel === 'about'         && <AboutPanel />}
    </React.Suspense>
  );
}

function AppShell() {
  const { state } = useApp();

  if (state.isInitializing) {
    return <LoadingScreen onComplete={() => {}} />;
  }

  if (state.isLocked || !state.identityStatus?.setup_complete) {
    return <LoginScreen />;
  }

  const rootClass = [
    'scanlines',
    state.isEmergencyMode ? 'emergency' : '',
    state.isStealthMode ? 'stealth' : '',
  ].filter(Boolean).join(' ');

  return (
    <div
      className={`${rootClass} gw-app-shell`}
      style={{
        width: '100vw', height: '100vh',
        display: 'flex', overflow: 'hidden',
        background: 'var(--void)',
      }}
    >
      <Sidebar />

      <div className="gw-main-column">
        <Header />
        <main className="gw-content" style={{ overflow: state.activePanel === 'communication' || state.activePanel === 'channels' ? 'hidden' : undefined }}>
          <PanelRouter />
        </main>
      </div>

      <Toast />
      <div className="gw-mobile-dock">
        <MobileNav />
      </div>
    </div>
  );
}

export default function App() {
  return (
    <AppProvider>
      <AppShell />
    </AppProvider>
  );
}
