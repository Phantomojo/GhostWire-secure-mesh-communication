import React from 'react';

const outreach = [
  { title: 'Featured on Privacy Weekly', link: '#', desc: 'GhostWire was highlighted as a top privacy project.' },
  { title: 'Partnership with OpenMesh', link: '#', desc: 'Collaborating to expand mesh networking.' },
  { title: 'Interview with MeshCast', link: '#', desc: 'Deep dive into GhostWire’s architecture.' },
];

const Outreach = () => (
  <div className="min-h-screen bg-gradient-to-br from-indigo-900 via-black to-purple-900 py-16 px-4">
    <h1 className="text-5xl font-extrabold text-white text-center mb-12 drop-shadow-lg">Outreach</h1>
    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8 max-w-5xl mx-auto mb-16">
      {outreach.map((item) => (
        <a key={item.title} href={item.link} className="bg-white/10 hover:bg-white/20 rounded-2xl p-8 shadow-xl transition flex flex-col gap-2">
          <h2 className="text-2xl font-bold text-white mb-2">{item.title}</h2>
          <p className="text-gray-200">{item.desc}</p>
        </a>
      ))}
    </div>
    <div className="max-w-xl mx-auto text-center text-gray-200">
      <h2 className="text-2xl font-bold mb-2">Press & Partnerships</h2>
      <p>For media inquiries or partnership opportunities, contact us at <a href="mailto:press@ghostwire.io" className="underline text-indigo-300">press@ghostwire.io</a>.</p>
    </div>
  </div>
);

export default Outreach; 