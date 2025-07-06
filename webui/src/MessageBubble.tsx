const MessageBubble = ({ fromMe, text, time }: { fromMe: boolean; text: string; time: string }) => (
  <div className={`flex ${fromMe ? 'justify-end' : 'justify-start'}`}>
    <div className={`px-4 py-2 rounded-2xl max-w-xs ${fromMe ? 'bg-cyan-500 text-white' : 'bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-100'}`}>
      <div>{text}</div>
      <div className="text-xs text-gray-400 mt-1 text-right">{time}</div>
    </div>
  </div>
);

export default MessageBubble; 