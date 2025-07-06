const MessageInput = ({ input, setInput }: { input: string; setInput: (val: string) => void }) => (
  <form className="flex items-center gap-2 px-6 py-4 border-t border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-800" onSubmit={e => { e.preventDefault(); }}>
    <button type="button" className="text-2xl px-2">😊</button>
    <button type="button" className="text-xl px-2">📎</button>
    <input
      type="text"
      className="flex-1 px-4 py-2 rounded-full border border-gray-300 dark:border-gray-600 bg-gray-100 dark:bg-gray-700 text-gray-800 dark:text-gray-100 focus:outline-none"
      placeholder="Type a message..."
      value={input}
      onChange={e => setInput(e.target.value)}
    />
    <button type="submit" className="bg-cyan-500 hover:bg-cyan-600 text-white px-6 py-2 rounded-full font-semibold">Send</button>
  </form>
);

export default MessageInput; 