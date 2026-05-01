const PrivacySettings = () => (
  <div className="p-6 max-w-lg mx-auto">
    <h2 className="text-2xl font-bold mb-4">Privacy Settings</h2>
    <div className="mb-4">
      <label className="block mb-1 font-semibold">Blocked Users</label>
      <input type="text" className="w-full px-3 py-2 rounded border border-gray-300 dark:border-gray-600 bg-gray-100 dark:bg-gray-700" placeholder="Search blocked users..." />
    </div>
    <div className="mb-4">
      <label className="block mb-1 font-semibold">Read Receipts</label>
      <input type="checkbox" className="mr-2" /> Allow read receipts
    </div>
    <div className="mb-4">
      <label className="block mb-1 font-semibold">Last Seen</label>
      <input type="checkbox" className="mr-2" /> Show my last seen
    </div>
    <div className="mb-4">
      <label className="block mb-1 font-semibold">Stealth Mode</label>
      <input type="checkbox" className="mr-2" /> Enable stealth mode
    </div>
  </div>
);

export default PrivacySettings; 