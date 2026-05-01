const Feed = () => (
  <div className="p-6 max-w-2xl mx-auto space-y-6">
    <h2 className="text-2xl font-bold mb-4">Feed</h2>
    {/* Placeholder posts */}
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
      <div className="flex items-center gap-3 mb-2">
        <img src="https://randomuser.me/api/portraits/men/11.jpg" alt="User" className="w-8 h-8 rounded-full" />
        <div className="font-semibold text-gray-800 dark:text-gray-100">John Doe</div>
      </div>
      <div className="text-gray-700 dark:text-gray-200">Just joined GhostWire! Excited to connect securely.</div>
    </div>
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
      <div className="flex items-center gap-3 mb-2">
        <img src="https://randomuser.me/api/portraits/women/12.jpg" alt="User" className="w-8 h-8 rounded-full" />
        <div className="font-semibold text-gray-800 dark:text-gray-100">Jane Smith</div>
      </div>
      <div className="text-gray-700 dark:text-gray-200">Shared a file securely with the group.</div>
    </div>
  </div>
);

export default Feed; 