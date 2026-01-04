import requests
import time
import json

def test_v2_connectivity():
    NODES = {
        "node_alpha": {
            "name": "Alpha",
            "url": "http://localhost:3001",
            "token": "VqJTPFA8Q3qGVp1OGSxsXRHCL07q9f6G"
        },
        "node_beta": {
            "name": "Beta",
            "url": "http://localhost:3002",
            "token": "cYE0xoEpcM7KbFyjRjnkeesI5OV0ww7B"
        }
    }
    
    # Wait for nodes to be ready
    print("⏳ Waiting for nodes to initialize...")
    time.sleep(2)
    
    for node_id, node in NODES.items():
        print(f"\n--- Testing Node {node['name']} ({node['url']}) ---")
        headers = {"X-Ghost-Token": node['token']}
        
        # 1. Profile V2
        try:
            profile_url = f"{node['url']}/api/v2/profile"
            print(f"📡 Requesting: {profile_url}")
            profile_res = requests.get(profile_url, headers=headers)
            if profile_res.status_code != 200:
                print(f"❌ Profile V2 failed with status {profile_res.status_code}")
                print(f"   Response: {profile_res.text}")
            else:
                profile = profile_res.json()
                print(f"✅ Profile V2: {profile['data']['username']} (Score: {profile['data']['security_score']})")
                if "links" in profile:
                    print(f"🔗 HATEOAS: {profile['links']['self']}")
        except Exception as e:
            print(f"❌ Profile V2 failed: {e}")

        # 2. Security Dashboard V2
        try:
            security_url = f"{node['url']}/api/v2/security/dashboard"
            print(f"📡 Requesting: {security_url}")
            security_res = requests.get(security_url, headers=headers)
            if security_res.status_code != 200:
                print(f"❌ Security Dashboard V2 failed with status {security_res.status_code}")
                print(f"   Response: {security_res.text}")
            else:
                security = security_res.json()
                print(f"✅ Security Dashboard V2: Score={security['data']['score']}")
        except Exception as e:
            print(f"❌ Security Dashboard V2 failed: {e}")

        # 3. Peers V2 (Multi-node discovery)
        try:
            peers_url = f"{node['url']}/api/v2/peers"
            print(f"📡 Requesting: {peers_url}")
            peers_res = requests.get(peers_url, headers=headers)
            if peers_res.status_code != 200:
                print(f"❌ Peers V2 failed with status {peers_res.status_code}")
                print(f"   Response: {peers_res.text}")
            else:
                peers = peers_res.json()
                peer_list = peers['data']
                print(f"✅ Peers V2: Found {len(peer_list)} peers")
                for p in peer_list:
                    print(f"  - Node: {p['username']} ({p['status']}) Trust: {p['trust_score']}")
        except Exception as e:
            print(f"❌ Peers V2 failed: {e}")

if __name__ == "__main__":
    test_v2_connectivity()
