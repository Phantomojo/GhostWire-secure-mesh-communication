import React from 'react';
import TestimonialCarousel from '../components/TestimonialCarousel';

const Community = () => (
  <div className="min-h-screen bg-gradient-to-br from-purple-900 via-black to-indigo-900 py-16 px-4">
    <h1 className="text-5xl font-extrabold text-white text-center mb-12 drop-shadow-lg">Community</h1>
    <div className="max-w-2xl mx-auto text-center text-lg text-gray-200 mb-8">
      <p>Join the GhostWire community! Connect with us on Discord, contribute on GitHub, and help build the future of secure, decentralized communication.</p>
      <div className="flex justify-center gap-6 mt-6">
        <a href="https://github.com/phantomojo/ghostwire" className="bg-white/10 hover:bg-white/20 text-white px-6 py-3 rounded-xl font-semibold shadow-lg transition">GitHub</a>
        <a href="#" className="bg-indigo-600 hover:bg-indigo-700 text-white px-6 py-3 rounded-xl font-semibold shadow-lg transition">Discord</a>
      </div>
    </div>
    <TestimonialCarousel />
  </div>
);

export default Community; 