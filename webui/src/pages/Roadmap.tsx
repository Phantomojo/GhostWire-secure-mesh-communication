import React from 'react';
import RoadmapTimeline from '../components/RoadmapTimeline';

const Roadmap = () => (
  <div className="min-h-screen bg-gradient-to-br from-black via-gray-900 to-indigo-900 py-16 px-4">
    <h1 className="text-5xl font-extrabold text-white text-center mb-12 drop-shadow-lg">Roadmap</h1>
    <RoadmapTimeline />
  </div>
);

export default Roadmap; 