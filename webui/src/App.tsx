import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { AppStateProvider, useAppState } from './state';
import Sidebar from './Sidebar';
import ChatWindow from './ChatWindow';
import MeshNetwork from './MeshNetwork';
import './App.css';

const MainContent = () => {
  const { user, chats, loading, error } = useAppState();
  // Only show the first chat for demo purposes
  const chat = chats[0];

  if (loading) return <div className="p-8">Loading...</div>;
  if (error) return <div className="p-8 text-red-500">Error: {error}</div>;
  if (!user || !chat) return <div className="p-8">No data available.</div>;

  return (
    <div className="flex h-full">
      <Sidebar />
      <div className="flex-1 flex flex-col">
        <ChatWindow />
      </div>
    </div>
  );
};

const App = () => (
  <AppStateProvider>
    <BrowserRouter>
      <Routes>
        <Route path="/*" element={<MainContent />} />
      </Routes>
    </BrowserRouter>
  </AppStateProvider>
);

export default App; 