import React from 'react';

const HeroSection = () => (
  <section className="relative flex flex-col items-center justify-center min-h-[70vh] py-24">
    <div className="absolute inset-0 w-full h-full animate-gradient-x bg-gradient-to-tr from-indigo-500 via-purple-500 to-pink-500 opacity-30 blur-2xl z-0" />
    <div className="relative z-10 max-w-3xl w-full mx-auto px-6">
      <div className="backdrop-blur-xl bg-white/10 rounded-3xl shadow-2xl p-12 flex flex-col items-center">
        <h1 className="text-6xl md:text-7xl font-extrabold text-white text-center mb-6 drop-shadow-xl">
          GhostWire
        </h1>
        <p className="text-2xl md:text-3xl text-indigo-200 text-center mb-8 font-medium">
          Modular, privacy-focused mesh networking for everyone, everywhere.
        </p>
        <a
          href="/docs/getting-started"
          className="inline-block bg-gradient-to-r from-indigo-500 via-purple-500 to-pink-500 text-white text-lg font-bold px-8 py-4 rounded-2xl shadow-lg hover:scale-105 transition-transform duration-200"
        >
          Get Started
        </a>
      </div>
    </div>
  </section>
);

export default HeroSection; 