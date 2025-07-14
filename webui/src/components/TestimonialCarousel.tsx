import React, { useState, useEffect } from 'react';

const testimonials = [
  {
    name: 'Alex P.',
    quote: 'GhostWire is the most exciting mesh project I’ve seen. The privacy features are next-level.',
  },
  {
    name: 'Samira K.',
    quote: 'Finally, a mesh platform that’s actually easy to use and secure by default.',
  },
  {
    name: 'Jordan L.',
    quote: 'The modular design means I can add my own transport. Love it!',
  },
];

const TestimonialCarousel = () => {
  const [index, setIndex] = useState(0);
  useEffect(() => {
    const timer = setInterval(() => setIndex((i) => (i + 1) % testimonials.length), 5000);
    return () => clearInterval(timer);
  }, []);
  const t = testimonials[index];
  return (
    <div className="max-w-xl mx-auto bg-white/10 backdrop-blur-lg rounded-2xl p-8 shadow-2xl flex flex-col items-center transition-all duration-700">
      <p className="text-xl text-white text-center mb-4 italic">“{t.quote}”</p>
      <div className="text-indigo-300 font-bold text-lg">{t.name}</div>
    </div>
  );
};

export default TestimonialCarousel; 