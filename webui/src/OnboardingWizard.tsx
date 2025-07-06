const OnboardingWizard = () => (
  <div className="p-6 max-w-lg mx-auto">
    <h2 className="text-2xl font-bold mb-4">Welcome to GhostWire!</h2>
    <ol className="list-decimal pl-6 space-y-2">
      <li>Set up your profile (avatar, name, status)</li>
      <li>Configure security preferences</li>
      <li>Choose your theme and language</li>
      <li>Invite contacts or join groups</li>
    </ol>
    <button className="mt-6 bg-cyan-500 hover:bg-cyan-600 text-white px-6 py-2 rounded font-semibold">Get Started</button>
  </div>
);

export default OnboardingWizard; 