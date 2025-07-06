const NotificationsSettings = () => (
  <div className="p-6 max-w-lg mx-auto">
    <h2 className="text-2xl font-bold mb-4">Notification Settings</h2>
    <div className="mb-4">
      <label className="block mb-1 font-semibold">Sound</label>
      <input type="checkbox" className="mr-2" /> Enable sound notifications
    </div>
    <div className="mb-4">
      <label className="block mb-1 font-semibold">Desktop</label>
      <input type="checkbox" className="mr-2" /> Enable desktop notifications
    </div>
    <div className="mb-4">
      <label className="block mb-1 font-semibold">Push</label>
      <input type="checkbox" className="mr-2" /> Enable push notifications
    </div>
    <div className="mb-4">
      <label className="block mb-1 font-semibold">Mute Chats</label>
      <input type="checkbox" className="mr-2" /> Mute all chats
    </div>
  </div>
);

export default NotificationsSettings; 