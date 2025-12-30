import { useState, useEffect } from 'react';
import axios from 'axios';

interface SettingsPageProps {
    backendConnected: boolean;
    onBack: () => void;
}

interface SettingsState {
    // Identity
    username: string;
    displayName: string;
    bio: string;

    // Network
    listenPort: number;
    enableMdns: boolean;
    enableDht: boolean;
    maxPeers: number;

    // Security
    encryptionLevel: 'standard' | 'enhanced' | 'quantum';
    autoLockMinutes: number;
    keyRotationDays: number;

    // Privacy
    stealthMode: boolean;
    trafficMorphing: boolean;
    stripMetadata: boolean;

    // Notifications
    soundEnabled: boolean;
    desktopNotifications: boolean;
    messagePreview: boolean;

    // Advanced
    debugMode: boolean;
    logLevel: 'info' | 'debug' | 'trace';
}

const defaultSettings: SettingsState = {
    username: '',
    displayName: '',
    bio: '',
    listenPort: 3001,
    enableMdns: true,
    enableDht: true,
    maxPeers: 50,
    encryptionLevel: 'enhanced',
    autoLockMinutes: 15,
    keyRotationDays: 30,
    stealthMode: false,
    trafficMorphing: false,
    stripMetadata: true,
    soundEnabled: true,
    desktopNotifications: true,
    messagePreview: true,
    debugMode: false,
    logLevel: 'info',
};

export default function SettingsPage({ backendConnected, onBack }: SettingsPageProps) {
    const [settings, setSettings] = useState<SettingsState>(defaultSettings);
    const [activeSection, setActiveSection] = useState<string>('identity');
    const [saving, setSaving] = useState(false);
    const [saved, setSaved] = useState(false);

    // Load settings on mount
    useEffect(() => {
        if (backendConnected) {
            loadSettings();
        }
    }, [backendConnected]);

    const loadSettings = async () => {
        try {
            const response = await axios.get('http://localhost:3001/api/settings');
            if (response.data.success) {
                setSettings(prev => ({ ...prev, ...response.data.data }));
            }
        } catch (error) {
            console.error('Failed to load settings:', error);
        }
    };

    const saveSettings = async () => {
        setSaving(true);
        try {
            await axios.put('http://localhost:3001/api/settings', settings);
            setSaved(true);
            setTimeout(() => setSaved(false), 2000);
        } catch (error) {
            console.error('Failed to save settings:', error);
        } finally {
            setSaving(false);
        }
    };

    const updateSetting = <K extends keyof SettingsState>(key: K, value: SettingsState[K]) => {
        setSettings(prev => ({ ...prev, [key]: value }));
    };

    const sections = [
        { id: 'identity', label: 'Identity', icon: '👤' },
        { id: 'network', label: 'Network', icon: '🌐' },
        { id: 'security', label: 'Security', icon: '🔐' },
        { id: 'privacy', label: 'Privacy', icon: '👁️' },
        { id: 'notifications', label: 'Notifications', icon: '🔔' },
        { id: 'advanced', label: 'Advanced', icon: '⚙️' },
    ];

    return (
        <div className="space-y-6 animate-fade-in">
            {/* Header */}
            <div className="flex items-center justify-between">
                <div className="flex items-center gap-4">
                    <button
                        onClick={onBack}
                        className="p-2 rounded-xl bg-slate-800 hover:bg-slate-700 text-white transition-colors"
                    >
                        ← Back
                    </button>
                    <h1 className="text-2xl font-bold text-white">Settings</h1>
                </div>
                <button
                    onClick={saveSettings}
                    disabled={saving}
                    className={`px-6 py-2 rounded-xl font-semibold transition-all ${saved
                            ? 'bg-emerald-500 text-white'
                            : 'bg-indigo-600 hover:bg-indigo-500 text-white'
                        }`}
                >
                    {saving ? 'Saving...' : saved ? '✓ Saved' : 'Save Changes'}
                </button>
            </div>

            <div className="grid grid-cols-4 gap-6">
                {/* Sidebar Navigation */}
                <div className="col-span-1 space-y-2">
                    {sections.map(section => (
                        <button
                            key={section.id}
                            onClick={() => setActiveSection(section.id)}
                            className={`w-full flex items-center gap-3 px-4 py-3 rounded-xl text-left transition-all ${activeSection === section.id
                                    ? 'bg-indigo-600/20 text-white border border-indigo-500/30'
                                    : 'text-slate-400 hover:bg-white/5 hover:text-white'
                                }`}
                        >
                            <span className="text-xl">{section.icon}</span>
                            <span className="font-medium">{section.label}</span>
                        </button>
                    ))}
                </div>

                {/* Settings Content */}
                <div className="col-span-3 bg-slate-900/50 backdrop-blur-sm border border-white/10 rounded-2xl p-6">
                    {/* Identity Section */}
                    {activeSection === 'identity' && (
                        <div className="space-y-6">
                            <h2 className="text-xl font-bold text-white flex items-center gap-2">
                                👤 Identity Settings
                            </h2>

                            <div className="space-y-4">
                                <div>
                                    <label className="block text-sm font-medium text-white/60 mb-2">Username</label>
                                    <input
                                        type="text"
                                        value={settings.username}
                                        onChange={(e) => updateSetting('username', e.target.value)}
                                        className="w-full bg-slate-800 border border-white/10 rounded-xl px-4 py-3 text-white focus:border-indigo-500 focus:outline-none"
                                        placeholder="your_username"
                                    />
                                </div>

                                <div>
                                    <label className="block text-sm font-medium text-white/60 mb-2">Display Name</label>
                                    <input
                                        type="text"
                                        value={settings.displayName}
                                        onChange={(e) => updateSetting('displayName', e.target.value)}
                                        className="w-full bg-slate-800 border border-white/10 rounded-xl px-4 py-3 text-white focus:border-indigo-500 focus:outline-none"
                                        placeholder="Your Display Name"
                                    />
                                </div>

                                <div>
                                    <label className="block text-sm font-medium text-white/60 mb-2">Bio</label>
                                    <textarea
                                        value={settings.bio}
                                        onChange={(e) => updateSetting('bio', e.target.value)}
                                        className="w-full bg-slate-800 border border-white/10 rounded-xl px-4 py-3 text-white focus:border-indigo-500 focus:outline-none resize-none h-24"
                                        placeholder="A short description about yourself..."
                                    />
                                </div>
                            </div>
                        </div>
                    )}

                    {/* Network Section */}
                    {activeSection === 'network' && (
                        <div className="space-y-6">
                            <h2 className="text-xl font-bold text-white flex items-center gap-2">
                                🌐 Network Settings
                            </h2>

                            <div className="space-y-4">
                                <div>
                                    <label className="block text-sm font-medium text-white/60 mb-2">Listen Port</label>
                                    <input
                                        type="number"
                                        value={settings.listenPort}
                                        onChange={(e) => updateSetting('listenPort', parseInt(e.target.value))}
                                        className="w-full bg-slate-800 border border-white/10 rounded-xl px-4 py-3 text-white focus:border-indigo-500 focus:outline-none"
                                    />
                                </div>

                                <div>
                                    <label className="block text-sm font-medium text-white/60 mb-2">Max Peers</label>
                                    <input
                                        type="number"
                                        value={settings.maxPeers}
                                        onChange={(e) => updateSetting('maxPeers', parseInt(e.target.value))}
                                        className="w-full bg-slate-800 border border-white/10 rounded-xl px-4 py-3 text-white focus:border-indigo-500 focus:outline-none"
                                    />
                                </div>

                                <div className="flex items-center justify-between p-4 bg-slate-800/50 rounded-xl">
                                    <div>
                                        <div className="text-white font-medium">mDNS Discovery</div>
                                        <div className="text-sm text-white/50">Auto-discover peers on local network</div>
                                    </div>
                                    <button
                                        onClick={() => updateSetting('enableMdns', !settings.enableMdns)}
                                        className={`w-14 h-8 rounded-full transition-colors ${settings.enableMdns ? 'bg-emerald-500' : 'bg-slate-700'
                                            }`}
                                    >
                                        <div className={`w-6 h-6 bg-white rounded-full transition-transform ${settings.enableMdns ? 'translate-x-7' : 'translate-x-1'
                                            }`} />
                                    </button>
                                </div>

                                <div className="flex items-center justify-between p-4 bg-slate-800/50 rounded-xl">
                                    <div>
                                        <div className="text-white font-medium">DHT Network</div>
                                        <div className="text-sm text-white/50">Distributed hash table for peer discovery</div>
                                    </div>
                                    <button
                                        onClick={() => updateSetting('enableDht', !settings.enableDht)}
                                        className={`w-14 h-8 rounded-full transition-colors ${settings.enableDht ? 'bg-emerald-500' : 'bg-slate-700'
                                            }`}
                                    >
                                        <div className={`w-6 h-6 bg-white rounded-full transition-transform ${settings.enableDht ? 'translate-x-7' : 'translate-x-1'
                                            }`} />
                                    </button>
                                </div>
                            </div>
                        </div>
                    )}

                    {/* Security Section */}
                    {activeSection === 'security' && (
                        <div className="space-y-6">
                            <h2 className="text-xl font-bold text-white flex items-center gap-2">
                                🔐 Security Settings
                            </h2>

                            <div className="space-y-4">
                                <div>
                                    <label className="block text-sm font-medium text-white/60 mb-2">Encryption Level</label>
                                    <select
                                        value={settings.encryptionLevel}
                                        onChange={(e) => updateSetting('encryptionLevel', e.target.value as any)}
                                        className="w-full bg-slate-800 border border-white/10 rounded-xl px-4 py-3 text-white focus:border-indigo-500 focus:outline-none"
                                    >
                                        <option value="standard">Standard (AES-256-GCM)</option>
                                        <option value="enhanced">Enhanced (ChaCha20-Poly1305)</option>
                                        <option value="quantum">Quantum-Resistant (Kyber)</option>
                                    </select>
                                </div>

                                <div>
                                    <label className="block text-sm font-medium text-white/60 mb-2">Auto-Lock (minutes)</label>
                                    <input
                                        type="number"
                                        value={settings.autoLockMinutes}
                                        onChange={(e) => updateSetting('autoLockMinutes', parseInt(e.target.value))}
                                        className="w-full bg-slate-800 border border-white/10 rounded-xl px-4 py-3 text-white focus:border-indigo-500 focus:outline-none"
                                    />
                                </div>

                                <div>
                                    <label className="block text-sm font-medium text-white/60 mb-2">Key Rotation (days)</label>
                                    <input
                                        type="number"
                                        value={settings.keyRotationDays}
                                        onChange={(e) => updateSetting('keyRotationDays', parseInt(e.target.value))}
                                        className="w-full bg-slate-800 border border-white/10 rounded-xl px-4 py-3 text-white focus:border-indigo-500 focus:outline-none"
                                    />
                                </div>
                            </div>
                        </div>
                    )}

                    {/* Privacy Section */}
                    {activeSection === 'privacy' && (
                        <div className="space-y-6">
                            <h2 className="text-xl font-bold text-white flex items-center gap-2">
                                👁️ Privacy Settings
                            </h2>

                            <div className="space-y-4">
                                <div className="flex items-center justify-between p-4 bg-slate-800/50 rounded-xl">
                                    <div>
                                        <div className="text-white font-medium">Stealth Mode</div>
                                        <div className="text-sm text-white/50">Hide presence from network scans</div>
                                    </div>
                                    <button
                                        onClick={() => updateSetting('stealthMode', !settings.stealthMode)}
                                        className={`w-14 h-8 rounded-full transition-colors ${settings.stealthMode ? 'bg-emerald-500' : 'bg-slate-700'
                                            }`}
                                    >
                                        <div className={`w-6 h-6 bg-white rounded-full transition-transform ${settings.stealthMode ? 'translate-x-7' : 'translate-x-1'
                                            }`} />
                                    </button>
                                </div>

                                <div className="flex items-center justify-between p-4 bg-slate-800/50 rounded-xl">
                                    <div>
                                        <div className="text-white font-medium">Traffic Morphing</div>
                                        <div className="text-sm text-white/50">Disguise traffic patterns</div>
                                    </div>
                                    <button
                                        onClick={() => updateSetting('trafficMorphing', !settings.trafficMorphing)}
                                        className={`w-14 h-8 rounded-full transition-colors ${settings.trafficMorphing ? 'bg-emerald-500' : 'bg-slate-700'
                                            }`}
                                    >
                                        <div className={`w-6 h-6 bg-white rounded-full transition-transform ${settings.trafficMorphing ? 'translate-x-7' : 'translate-x-1'
                                            }`} />
                                    </button>
                                </div>

                                <div className="flex items-center justify-between p-4 bg-slate-800/50 rounded-xl">
                                    <div>
                                        <div className="text-white font-medium">Strip Metadata</div>
                                        <div className="text-sm text-white/50">Remove identifying info from messages</div>
                                    </div>
                                    <button
                                        onClick={() => updateSetting('stripMetadata', !settings.stripMetadata)}
                                        className={`w-14 h-8 rounded-full transition-colors ${settings.stripMetadata ? 'bg-emerald-500' : 'bg-slate-700'
                                            }`}
                                    >
                                        <div className={`w-6 h-6 bg-white rounded-full transition-transform ${settings.stripMetadata ? 'translate-x-7' : 'translate-x-1'
                                            }`} />
                                    </button>
                                </div>
                            </div>
                        </div>
                    )}

                    {/* Notifications Section */}
                    {activeSection === 'notifications' && (
                        <div className="space-y-6">
                            <h2 className="text-xl font-bold text-white flex items-center gap-2">
                                🔔 Notification Settings
                            </h2>

                            <div className="space-y-4">
                                <div className="flex items-center justify-between p-4 bg-slate-800/50 rounded-xl">
                                    <div>
                                        <div className="text-white font-medium">Sound Effects</div>
                                        <div className="text-sm text-white/50">Play sounds for events</div>
                                    </div>
                                    <button
                                        onClick={() => updateSetting('soundEnabled', !settings.soundEnabled)}
                                        className={`w-14 h-8 rounded-full transition-colors ${settings.soundEnabled ? 'bg-emerald-500' : 'bg-slate-700'
                                            }`}
                                    >
                                        <div className={`w-6 h-6 bg-white rounded-full transition-transform ${settings.soundEnabled ? 'translate-x-7' : 'translate-x-1'
                                            }`} />
                                    </button>
                                </div>

                                <div className="flex items-center justify-between p-4 bg-slate-800/50 rounded-xl">
                                    <div>
                                        <div className="text-white font-medium">Desktop Notifications</div>
                                        <div className="text-sm text-white/50">Show system notifications</div>
                                    </div>
                                    <button
                                        onClick={() => updateSetting('desktopNotifications', !settings.desktopNotifications)}
                                        className={`w-14 h-8 rounded-full transition-colors ${settings.desktopNotifications ? 'bg-emerald-500' : 'bg-slate-700'
                                            }`}
                                    >
                                        <div className={`w-6 h-6 bg-white rounded-full transition-transform ${settings.desktopNotifications ? 'translate-x-7' : 'translate-x-1'
                                            }`} />
                                    </button>
                                </div>

                                <div className="flex items-center justify-between p-4 bg-slate-800/50 rounded-xl">
                                    <div>
                                        <div className="text-white font-medium">Message Preview</div>
                                        <div className="text-sm text-white/50">Show message content in notifications</div>
                                    </div>
                                    <button
                                        onClick={() => updateSetting('messagePreview', !settings.messagePreview)}
                                        className={`w-14 h-8 rounded-full transition-colors ${settings.messagePreview ? 'bg-emerald-500' : 'bg-slate-700'
                                            }`}
                                    >
                                        <div className={`w-6 h-6 bg-white rounded-full transition-transform ${settings.messagePreview ? 'translate-x-7' : 'translate-x-1'
                                            }`} />
                                    </button>
                                </div>
                            </div>
                        </div>
                    )}

                    {/* Advanced Section */}
                    {activeSection === 'advanced' && (
                        <div className="space-y-6">
                            <h2 className="text-xl font-bold text-white flex items-center gap-2">
                                ⚙️ Advanced Settings
                            </h2>

                            <div className="space-y-4">
                                <div className="flex items-center justify-between p-4 bg-slate-800/50 rounded-xl">
                                    <div>
                                        <div className="text-white font-medium">Debug Mode</div>
                                        <div className="text-sm text-white/50">Enable verbose logging</div>
                                    </div>
                                    <button
                                        onClick={() => updateSetting('debugMode', !settings.debugMode)}
                                        className={`w-14 h-8 rounded-full transition-colors ${settings.debugMode ? 'bg-yellow-500' : 'bg-slate-700'
                                            }`}
                                    >
                                        <div className={`w-6 h-6 bg-white rounded-full transition-transform ${settings.debugMode ? 'translate-x-7' : 'translate-x-1'
                                            }`} />
                                    </button>
                                </div>

                                <div>
                                    <label className="block text-sm font-medium text-white/60 mb-2">Log Level</label>
                                    <select
                                        value={settings.logLevel}
                                        onChange={(e) => updateSetting('logLevel', e.target.value as any)}
                                        className="w-full bg-slate-800 border border-white/10 rounded-xl px-4 py-3 text-white focus:border-indigo-500 focus:outline-none"
                                    >
                                        <option value="info">Info</option>
                                        <option value="debug">Debug</option>
                                        <option value="trace">Trace</option>
                                    </select>
                                </div>

                                <div className="p-4 bg-red-500/10 border border-red-500/30 rounded-xl">
                                    <div className="text-red-400 font-medium mb-2">Danger Zone</div>
                                    <div className="flex gap-3">
                                        <button className="px-4 py-2 bg-red-500/20 border border-red-500/30 rounded-lg text-red-400 hover:bg-red-500/30 transition-colors text-sm">
                                            Export Data
                                        </button>
                                        <button className="px-4 py-2 bg-red-500/20 border border-red-500/30 rounded-lg text-red-400 hover:bg-red-500/30 transition-colors text-sm">
                                            Clear All Data
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
}
