import React, { useState, useEffect } from 'react';
import { api } from './services/api';

interface MeshStats {
  total_nodes: number;
  online_nodes: number;
  local_node_id: string;
  routes_count: number;
}

interface MeshNode {
  id: string;
  peer_id: string;
  address: string;
  username: string;
  public_key: string;
  last_seen: number;
  connection_quality: number;
  is_online: boolean;
}

interface MeshTopology {
  nodes: { [key: string]: MeshNode };
  routes: { [key: string]: string[] };
  local_node_id: string;
}

const MeshNetwork: React.FC = () => {
  const [isInitialized, setIsInitialized] = useState(false);
  const [isStarted, setIsStarted] = useState(false);
  const [meshStats, setMeshStats] = useState<MeshStats | null>(null);
  const [meshNodes, setMeshNodes] = useState<MeshNode[]>([]);
  const [meshTopology, setMeshTopology] = useState<MeshTopology | null>(null);
  const [listenAddress, setListenAddress] = useState('/ip4/0.0.0.0/tcp/4001');
  const [meshtasticAddress, setMeshtasticAddress] = useState('localhost:4403');
  const [meshMessage, setMeshMessage] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Initialize mesh networking
  const initializeMesh = async () => {
    setIsLoading(true);
    setError(null);
    try {
      const response = await api.post('/api/mesh/init');
      if (response.data.success) {
        setIsInitialized(true);
        console.log('Mesh networking initialized');
      } else {
        setError(response.data.error || 'Failed to initialize mesh');
      }
    } catch (err) {
      setError('Failed to initialize mesh networking');
      console.error('Mesh init error:', err);
    } finally {
      setIsLoading(false);
    }
  };

  // Start mesh network
  const startMesh = async () => {
    setIsLoading(true);
    setError(null);
    try {
      const response = await api.post('/api/mesh/start', {
        address: listenAddress
      });
      if (response.data.success) {
        setIsStarted(true);
        console.log('Mesh network started on', listenAddress);
      } else {
        setError(response.data.error || 'Failed to start mesh');
      }
    } catch (err) {
      setError('Failed to start mesh network');
      console.error('Mesh start error:', err);
    } finally {
      setIsLoading(false);
    }
  };

  // Connect to Meshtastic device
  const connectMeshtastic = async () => {
    setIsLoading(true);
    setError(null);
    try {
      const response = await api.post('/api/mesh/connect_meshtastic', {
        address: meshtasticAddress
      });
      if (response.data.success) {
        console.log('Connected to Meshtastic device');
      } else {
        setError(response.data.error || 'Failed to connect to Meshtastic');
      }
    } catch (err) {
      setError('Failed to connect to Meshtastic device');
      console.error('Meshtastic connection error:', err);
    } finally {
      setIsLoading(false);
    }
  };

  // Send message through mesh
  const sendMeshMessage = async () => {
    if (!meshMessage.trim()) return;
    
    setIsLoading(true);
    setError(null);
    try {
      const response = await api.post('/api/mesh/send_message', {
        content: meshMessage
      });
      if (response.data.success) {
        setMeshMessage('');
        console.log('Message sent through mesh');
      } else {
        setError(response.data.error || 'Failed to send mesh message');
      }
    } catch (err) {
      setError('Failed to send mesh message');
      console.error('Mesh message error:', err);
    } finally {
      setIsLoading(false);
    }
  };

  // Get mesh statistics
  const getMeshStats = async () => {
    try {
      const response = await api.get('/api/mesh/stats');
      if (response.data.success) {
        setMeshStats(response.data.data.stats);
      }
    } catch (err) {
      console.error('Failed to get mesh stats:', err);
    }
  };

  // Get mesh nodes
  const getMeshNodes = async () => {
    try {
      const response = await api.get('/api/mesh/nodes');
      if (response.data.success) {
        setMeshNodes(response.data.data.nodes);
      }
    } catch (err) {
      console.error('Failed to get mesh nodes:', err);
    }
  };

  // Get mesh topology
  const getMeshTopology = async () => {
    try {
      const response = await api.get('/api/mesh/topology');
      if (response.data.success) {
        setMeshTopology(response.data.data.topology);
      }
    } catch (err) {
      console.error('Failed to get mesh topology:', err);
    }
  };

  // Refresh mesh data
  const refreshMeshData = () => {
    if (isInitialized) {
      getMeshStats();
      getMeshNodes();
      getMeshTopology();
    }
  };

  // Auto-refresh every 10 seconds
  useEffect(() => {
    if (isInitialized) {
      refreshMeshData();
      const interval = setInterval(refreshMeshData, 10000);
      return () => clearInterval(interval);
    }
  }, [isInitialized]);

  return (
    <div className="p-6 bg-gray-50 min-h-screen">
      <div className="max-w-6xl mx-auto">
        <h1 className="text-3xl font-bold text-gray-900 mb-8">Mesh Network</h1>
        
        {error && (
          <div className="mb-6 p-4 bg-red-100 border border-red-400 text-red-700 rounded">
            {error}
          </div>
        )}

        {/* Mesh Initialization */}
        <div className="bg-white rounded-lg shadow-md p-6 mb-6">
          <h2 className="text-xl font-semibold mb-4">Mesh Network Setup</h2>
          
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
            <div>
              <h3 className="text-lg font-medium mb-3">Initialize Mesh</h3>
              <button
                onClick={initializeMesh}
                disabled={isLoading || isInitialized}
                className="w-full bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700 disabled:bg-gray-400"
              >
                {isLoading ? 'Initializing...' : isInitialized ? 'Initialized' : 'Initialize Mesh'}
              </button>
            </div>

            <div>
              <h3 className="text-lg font-medium mb-3">Start Mesh Network</h3>
              <div className="space-y-3">
                <input
                  type="text"
                  value={listenAddress}
                  onChange={(e) => setListenAddress(e.target.value)}
                  placeholder="Listen address (e.g., /ip4/0.0.0.0/tcp/4001)"
                  className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
                <button
                  onClick={startMesh}
                  disabled={isLoading || !isInitialized || isStarted}
                  className="w-full bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700 disabled:bg-gray-400"
                >
                  {isLoading ? 'Starting...' : isStarted ? 'Started' : 'Start Mesh'}
                </button>
              </div>
            </div>
          </div>
        </div>

        {/* Meshtastic Connection */}
        <div className="bg-white rounded-lg shadow-md p-6 mb-6">
          <h2 className="text-xl font-semibold mb-4">Meshtastic Integration</h2>
          <div className="space-y-3">
            <input
              type="text"
              value={meshtasticAddress}
              onChange={(e) => setMeshtasticAddress(e.target.value)}
              placeholder="Meshtastic device address (e.g., localhost:4403)"
              className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500"
            />
            <button
              onClick={connectMeshtastic}
              disabled={isLoading}
              className="w-full bg-purple-600 text-white px-4 py-2 rounded hover:bg-purple-700 disabled:bg-gray-400"
            >
              {isLoading ? 'Connecting...' : 'Connect to Meshtastic'}
            </button>
          </div>
        </div>

        {/* Mesh Messaging */}
        <div className="bg-white rounded-lg shadow-md p-6 mb-6">
          <h2 className="text-xl font-semibold mb-4">Mesh Messaging</h2>
          <div className="space-y-3">
            <textarea
              value={meshMessage}
              onChange={(e) => setMeshMessage(e.target.value)}
              placeholder="Enter message to send through mesh network..."
              className="w-full px-3 py-2 border border-gray-300 rounded focus:outline-none focus:ring-2 focus:ring-blue-500 h-24"
            />
            <button
              onClick={sendMeshMessage}
              disabled={isLoading || !meshMessage.trim()}
              className="w-full bg-indigo-600 text-white px-4 py-2 rounded hover:bg-indigo-700 disabled:bg-gray-400"
            >
              {isLoading ? 'Sending...' : 'Send Mesh Message'}
            </button>
          </div>
        </div>

        {/* Mesh Statistics */}
        {meshStats && (
          <div className="bg-white rounded-lg shadow-md p-6 mb-6">
            <h2 className="text-xl font-semibold mb-4">Mesh Statistics</h2>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <div className="text-center">
                <div className="text-2xl font-bold text-blue-600">{meshStats.total_nodes}</div>
                <div className="text-sm text-gray-600">Total Nodes</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-green-600">{meshStats.online_nodes}</div>
                <div className="text-sm text-gray-600">Online Nodes</div>
              </div>
              <div className="text-center">
                <div className="text-2xl font-bold text-purple-600">{meshStats.routes_count}</div>
                <div className="text-sm text-gray-600">Routes</div>
              </div>
              <div className="text-center">
                <div className="text-xs font-mono text-gray-600 break-all">
                  {meshStats.local_node_id}
                </div>
                <div className="text-sm text-gray-600">Local Node ID</div>
              </div>
            </div>
          </div>
        )}

        {/* Mesh Nodes */}
        {meshNodes.length > 0 && (
          <div className="bg-white rounded-lg shadow-md p-6 mb-6">
            <h2 className="text-xl font-semibold mb-4">Mesh Nodes</h2>
            <div className="overflow-x-auto">
              <table className="min-w-full divide-y divide-gray-200">
                <thead className="bg-gray-50">
                  <tr>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Node
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Username
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Status
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Quality
                    </th>
                    <th className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                      Last Seen
                    </th>
                  </tr>
                </thead>
                <tbody className="bg-white divide-y divide-gray-200">
                  {meshNodes.map((node) => (
                    <tr key={node.id}>
                      <td className="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
                        {node.id}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                        {node.username}
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap">
                        <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${
                          node.is_online 
                            ? 'bg-green-100 text-green-800' 
                            : 'bg-red-100 text-red-800'
                        }`}>
                          {node.is_online ? 'Online' : 'Offline'}
                        </span>
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                        {(node.connection_quality * 100).toFixed(1)}%
                      </td>
                      <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                        {new Date(node.last_seen * 1000).toLocaleString()}
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          </div>
        )}

        {/* Mesh Topology */}
        {meshTopology && (
          <div className="bg-white rounded-lg shadow-md p-6">
            <h2 className="text-xl font-semibold mb-4">Mesh Topology</h2>
            <div className="space-y-4">
              <div>
                <h3 className="text-lg font-medium mb-2">Routes</h3>
                {Object.entries(meshTopology.routes).map(([target, route]) => (
                  <div key={target} className="text-sm text-gray-600 mb-1">
                    <span className="font-medium">{target}:</span> {route.join(' → ')}
                  </div>
                ))}
              </div>
            </div>
          </div>
        )}

        {/* Refresh Button */}
        <div className="mt-6 text-center">
          <button
            onClick={refreshMeshData}
            disabled={!isInitialized}
            className="bg-gray-600 text-white px-6 py-2 rounded hover:bg-gray-700 disabled:bg-gray-400"
          >
            Refresh Mesh Data
          </button>
        </div>
      </div>
    </div>
  );
};

export default MeshNetwork; 