import React from 'react';
import { FaLock, FaNetworkWired, FaUserShield, FaPlug, FaMobileAlt, FaGlobe } from 'react-icons/fa';

const features = [
  { icon: <FaLock className="text-4xl text-indigo-400" />, title: 'End-to-End Encryption', desc: 'All messages are encrypted with industry-standard cryptography.' },
  { icon: <FaNetworkWired className="text-4xl text-purple-400" />, title: 'Mesh Networking', desc: 'Peer-to-peer mesh with multi-transport support (Bluetooth, WiFi, LoRa, WebRTC, TCP/IP).' },
  { icon: <FaUserShield className="text-4xl text-pink-400" />, title: 'Sybil Defense', desc: 'Advanced trust and reputation systems to prevent Sybil attacks.' },
  { icon: <FaPlug className="text-4xl text-blue-400" />, title: 'Protocol Adapters', desc: 'Bridge to Briar, Meshtastic, Matrix, and more.' },
  { icon: <FaMobileAlt className="text-4xl text-green-400" />, title: 'Mobile & Desktop', desc: 'Runs everywhere: desktop, mobile, and embedded devices.' },
  { icon: <FaGlobe className="text-4xl text-yellow-400" />, title: 'Censorship Resistance', desc: 'Obfuscation, cover traffic, and panic wipe for privacy in hostile environments.' },
];

const FeatureGrid = () => (
  <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-10">
    {features.map((f) => (
      <div key={f.title} className="bg-white/10 backdrop-blur-lg rounded-2xl p-8 flex flex-col items-center shadow-xl hover:scale-105 transition-transform duration-200">
        {f.icon}
        <h3 className="text-2xl font-bold text-white mt-4 mb-2 text-center">{f.title}</h3>
        <p className="text-gray-200 text-center">{f.desc}</p>
      </div>
    ))}
  </div>
);

export default FeatureGrid; 