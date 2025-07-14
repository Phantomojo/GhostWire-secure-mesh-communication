import React from 'react';

const milestones = [
  { date: 'Q2 2024', title: 'MVP Launch', desc: 'Core mesh networking, CLI, and web UI released.' },
  { date: 'Q3 2024', title: 'Modular Transports', desc: 'Bluetooth, WiFi, LoRa, WebRTC support.' },
  { date: 'Q4 2024', title: 'Protocol Adapters', desc: 'Briar, Meshtastic, Matrix bridges.' },
  { date: 'Q1 2025', title: 'Advanced Security', desc: 'Cover traffic, Sybil defense, panic wipe.' },
  { date: 'Q2 2025', title: 'Mobile/Desktop Wrappers', desc: 'Tauri and React Native apps.' },
];

const RoadmapTimeline = () => (
  <div className="relative max-w-2xl mx-auto">
    <div className="border-l-4 border-indigo-500 absolute h-full left-6 top-0" />
    <ul className="space-y-12 pl-16">
      {milestones.map((m, i) => (
        <li key={m.title} className="relative">
          <div className="absolute left-[-38px] top-2 w-8 h-8 bg-gradient-to-tr from-indigo-500 via-purple-500 to-pink-500 rounded-full shadow-lg flex items-center justify-center text-white font-bold text-lg">
            {i + 1}
          </div>
          <div className="bg-white/10 backdrop-blur-lg rounded-xl p-6 shadow-xl">
            <div className="text-indigo-300 font-semibold mb-1">{m.date}</div>
            <div className="text-2xl font-bold text-white mb-2">{m.title}</div>
            <div className="text-gray-200">{m.desc}</div>
          </div>
        </li>
      ))}
    </ul>
  </div>
);

export default RoadmapTimeline; 