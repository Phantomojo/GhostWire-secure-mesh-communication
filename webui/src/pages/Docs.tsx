import React from 'react';

const docs = [
  { title: 'Getting Started', link: '/docs/getting-started', desc: 'Quickstart guide for new users.' },
  { title: 'API Reference', link: '/docs/api', desc: 'REST and WebSocket API documentation.' },
  { title: 'Security', link: '/docs/security', desc: 'Security model and threat analysis.' },
  { title: 'Roadmap', link: '/docs/roadmap', desc: 'Project roadmap and milestones.' },
  { title: 'Contributing', link: '/docs/contributing', desc: 'How to contribute to GhostWire.' },
];

const Docs = () => (
  <div className="min-h-screen bg-gradient-to-br from-gray-900 via-black to-indigo-900 py-16 px-4">
    <h1 className="text-5xl font-extrabold text-white text-center mb-12 drop-shadow-lg">Documentation</h1>
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 max-w-5xl mx-auto">
      {docs.map((doc) => (
        <a key={doc.title} href={doc.link} className="bg-white/10 hover:bg-white/20 rounded-2xl p-8 shadow-xl transition flex flex-col gap-2">
          <h2 className="text-2xl font-bold text-white mb-2">{doc.title}</h2>
          <p className="text-gray-200">{doc.desc}</p>
        </a>
      ))}
    </div>
  </div>
);

export default Docs; 