import React from 'react';
import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom';
import Home from './pages/Home';
import Features from './pages/Features';
import Roadmap from './pages/Roadmap';
import Community from './pages/Community';
import Docs from './pages/Docs';
import Outreach from './pages/Outreach';

const Navbar = () => (
  <nav className="w-full z-20 top-0 left-0 bg-gradient-to-r from-black via-indigo-900 to-purple-900 shadow-xl py-4 px-6 flex items-center justify-between">
    <Link to="/" className="text-3xl font-extrabold text-white tracking-tight drop-shadow-lg">GhostWire</Link>
    <div className="flex gap-6 text-lg font-semibold">
      <Link to="/features" className="text-white hover:text-indigo-300 transition">Features</Link>
      <Link to="/roadmap" className="text-white hover:text-indigo-300 transition">Roadmap</Link>
      <Link to="/community" className="text-white hover:text-indigo-300 transition">Community</Link>
      <Link to="/docs" className="text-white hover:text-indigo-300 transition">Docs</Link>
      <Link to="/outreach" className="text-white hover:text-indigo-300 transition">Outreach</Link>
    </div>
  </nav>
);

const Footer = () => (
  <footer className="w-full bg-gradient-to-r from-black via-indigo-900 to-purple-900 py-8 px-6 text-center text-gray-400 mt-16">
    <div className="mb-2">
      <span className="font-bold text-white">GhostWire</span> &copy; {new Date().getFullYear()} &mdash; Modular, privacy-focused mesh networking.
    </div>
    <div className="text-sm">Built with ❤️ using Rust, React, and TailwindCSS.</div>
  </footer>
);

function App() {
  return (
    <Router>
      <div className="flex flex-col min-h-screen bg-black">
        <Navbar />
        <main className="flex-1">
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/features" element={<Features />} />
            <Route path="/roadmap" element={<Roadmap />} />
            <Route path="/community" element={<Community />} />
            <Route path="/docs" element={<Docs />} />
            <Route path="/outreach" element={<Outreach />} />
          </Routes>
        </main>
        <Footer />
      </div>
    </Router>
  );
}

export default App; 