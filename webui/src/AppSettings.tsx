const AppSettings = () => (
  <div className="p-6 max-w-lg mx-auto">
    <h2 className="text-2xl font-bold mb-4">App Settings</h2>
    <div className="mb-4">
      <label className="block mb-1 font-semibold">Theme</label>
      <select className="w-full px-3 py-2 rounded border border-gray-300 dark:border-gray-600 bg-gray-100 dark:bg-gray-700">
        <option>Light</option>
        <option>Dark</option>
        <option>System</option>
      </select>
    </div>
    <div className="mb-4">
      <label className="block mb-1 font-semibold">Font Size</label>
      <select className="w-full px-3 py-2 rounded border border-gray-300 dark:border-gray-600 bg-gray-100 dark:bg-gray-700">
        <option>Small</option>
        <option>Medium</option>
        <option>Large</option>
      </select>
    </div>
    <div className="mb-4">
      <label className="block mb-1 font-semibold">Language</label>
      <select className="w-full px-3 py-2 rounded border border-gray-300 dark:border-gray-600 bg-gray-100 dark:bg-gray-700">
        <option>English</option>
        <option>Spanish</option>
        <option>French</option>
      </select>
    </div>
    <div className="mb-4">
      <label className="block mb-1 font-semibold">Notifications</label>
      <input type="checkbox" className="mr-2" /> Enable notifications
    </div>
    <div className="mb-4">
      <label className="block mb-1 font-semibold">Account</label>
      <button className="bg-red-500 hover:bg-red-600 text-white px-4 py-2 rounded font-semibold">Logout</button>
    </div>
  </div>
);

export default AppSettings; 