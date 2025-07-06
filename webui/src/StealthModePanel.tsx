const StealthModePanel = () => (
  <div className="p-6 max-w-lg mx-auto">
    <h2 className="text-2xl font-bold mb-4">Stealth Mode</h2>
    <div className="mb-4">
      <label className="block mb-1 font-semibold">Stealth Mode</label>
      <input type="checkbox" className="mr-2" /> Enable stealth mode
    </div>
    <div className="mb-4">
      <button className="bg-red-500 hover:bg-red-600 text-white px-4 py-2 rounded font-semibold">Panic Wipe</button>
    </div>
    <div className="mb-4">
      <button className="bg-gray-500 hover:bg-gray-600 text-white px-4 py-2 rounded font-semibold">Disguise UI</button>
    </div>
  </div>
);

export default StealthModePanel; 