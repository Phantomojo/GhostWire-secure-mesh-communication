#!/usr/bin/env python3
"""
GhostWire AI Content Generator
===============================
Automatically generates Instagram Reels from GhostWire codebase using AI.

Features:
- Analyzes GitHub repository
- Generates content ideas with AI
- Creates Reels from code snippets
- Auto-writes captions
- Ready for Instagram upload

Usage:
    python ai_instagram_generator.py --repo GhostWire --ideas 5 --render
"""

import os
import sys
import json
import argparse
from pathlib import Path
from datetime import datetime

# Try to import optional dependencies
try:
    from github import Github
    GITHUB_AVAILABLE = True
except ImportError:
    GITHUB_AVAILABLE = False
    print("⚠️  PyGithub not installed. Install with: pip install PyGithub")

try:
    import openai
    OPENAI_AVAILABLE = True
except ImportError:
    OPENAI_AVAILABLE = False
    print("⚠️  OpenAI not installed. Install with: pip install openai")


class GhostWireAIContentGenerator:
    """Generate Instagram content from GhostWire codebase using AI."""
    
    def __init__(self, repo_path=None, github_token=None, openai_key=None):
        self.repo_path = Path(repo_path) if repo_path else Path("/home/ph/GhostWire-secure-mesh-communication")
        self.github_token = github_token or os.getenv("GITHUB_TOKEN")
        self.openai_key = openai_key or os.getenv("OPENAI_API_KEY")
        self.output_dir = self.repo_path / "social-media" / "instagram" / "ai-generated"
        self.output_dir.mkdir(parents=True, exist_ok=True)
        
    def analyze_codebase(self):
        """Analyze GhostWire codebase and extract content-worthy elements."""
        print(f"🔍 Analyzing codebase at {self.repo_path}...")
        
        content_ideas = []
        
        # Key directories to analyze
        key_dirs = [
            "ghostwire/src",
            "docs",
            "README.md",
            "IMPLEMENTATION_STATUS_LIVE.md"
        ]
        
        # Find interesting files
        for key_dir in key_dirs:
            dir_path = self.repo_path / key_dir
            if dir_path.exists():
                if dir_path.is_file():
                    content_ideas.append(self._analyze_file(dir_path))
                else:
                    for file in dir_path.glob("**/*.rs")[:5]:  # Top 5 Rust files
                        content_ideas.append(self._analyze_file(file))
        
        print(f"✅ Found {len(content_ideas)} content-worthy elements")
        return content_ideas
    
    def _analyze_file(self, file_path):
        """Analyze a single file for content potential."""
        try:
            with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                content = f.read(2000)  # First 2000 chars
            
            # Extract key info
            return {
                "file": str(file_path.relative_to(self.repo_path)),
                "size": len(content),
                "preview": content[:200],
                "type": "code" if file_path.suffix == ".rs" else "docs"
            }
        except Exception as e:
            return {"error": str(e)}
    
    def generate_ideas_with_ai(self, content_elements, num_ideas=5):
        """Use AI to generate Instagram content ideas from codebase."""
        
        if not OPENAI_AVAILABLE or not self.openai_key:
            print("⚠️  OpenAI not available. Using template-based generation.")
            return self._generate_ideas_template(content_elements, num_ideas)
        
        print(f"🤖 Generating {num_ideas} content ideas with AI...")
        
        # Prepare context for AI
        context = "GhostWire is a decentralized mesh communication platform built with Rust + libp2p.\n"
        context += "Key features: No servers, no surveillance, no permission.\n\n"
        context += "Codebase elements:\n"
        for elem in content_elements[:5]:
            if "file" in elem:
                context += f"- {elem['file']}: {elem['preview'][:100]}...\n"
        
        # Generate ideas with GPT
        try:
            client = openai.OpenAI(api_key=self.openai_key)
            response = client.chat.completions.create(
                model="gpt-4",
                messages=[
                    {"role": "system", "content": "You are an expert Instagram content creator for tech products. Generate engaging Reel ideas that explain complex tech simply."},
                    {"role": "user", "content": f"Based on this GhostWire codebase, generate {num_ideas} Instagram Reel ideas (15-30 seconds each).\n\n{context}\n\nFor each idea provide:\n1. Title (catchy)\n2. Hook (first line)\n3. Visual description\n4. Script (2-3 sentences)\n5. Hashtags (5-10)\n\nFormat as JSON array."}
                ],
                response_format={"type": "json_object"}
            )
            
            ideas = json.loads(response.choices[0].message.content)
            
            # Save ideas
            ideas_file = self.output_dir / "content_ideas.json"
            with open(ideas_file, 'w') as f:
                json.dump(ideas, f, indent=2)
            
            print(f"✅ Generated {len(ideas)} ideas. Saved to {ideas_file}")
            return ideas
            
        except Exception as e:
            print(f"❌ AI generation failed: {e}")
            return self._generate_ideas_template(content_elements, num_ideas)
    
    def _generate_ideas_template(self, content_elements, num_ideas=5):
        """Generate content ideas using templates (no AI)."""
        
        templates = [
            {
                "title": "Code Spotlight: {filename}",
                "hook": "⚙️ Ever wondered how {feature} works?",
                "visual": "Show code snippet with syntax highlighting",
                "script": "This is {filename} from GhostWire. It handles {feature} using {tech}. No servers, no surveillance, just pure P2P magic.",
                "hashtags": "#Rust #Coding #OpenSource #Privacy #Tech"
            },
            {
                "title": "Architecture Explained",
                "hook": "🌐 6 Layers. Zero Compromise.",
                "visual": "Animated protocol stack diagram",
                "script": "GhostWire's protocol stack: Crypto, Routing, Transport, Mesh, Identity, Ghost Channels. Each layer encrypted. Each node anonymous.",
                "hashtags": "#MeshNetwork #Decentralized #Encryption #TechExplained"
            },
            {
                "title": "Problem vs Solution",
                "hook": "❌ Centralized servers = surveillance\n✅ Mesh = freedom",
                "visual": "Split screen: server farm vs mesh network",
                "script": "Traditional: Your data on their servers. GhostWire: Your data, your network, your control.",
                "hashtags": "#Privacy #Freedom #Tech #Innovation"
            },
            {
                "title": "Build in Public",
                "hook": "🚀 Building GhostWire in public. Day {day}.",
                "visual": "Screen recording of coding + terminal",
                "script": "Today's progress: {progress}. Building the ungovernable internet, one commit at a time.",
                "hashtags": "#BuildInPublic #IndieDev #Rust #OpenSource"
            },
            {
                "title": "Tech Stack Breakdown",
                "hook": "⚡ What's GhostWire built with?",
                "visual": "Animated tech stack with logos",
                "script": "Rust for safety. libp2p for networking. Sphinx for routing. Ed25519 for crypto. All open source. All auditable.",
                "hashtags": "#RustLang #Libp2p #Cryptography #DevCommunity"
            }
        ]
        
        # Fill templates with actual data
        ideas = []
        for i, template in enumerate(templates[:num_ideas]):
            idea = template.copy()
            if i < len(content_elements) and "file" in content_elements[i]:
                filename = content_elements[i]["file"].split("/")[-1]
                idea["title"] = idea["title"].format(filename=filename)
                idea["hook"] = idea["hook"].format(feature=filename.replace(".rs", ""))
            ideas.append(idea)
        
        # Save ideas
        ideas_file = self.output_dir / "content_ideas.json"
        with open(ideas_file, 'w') as f:
            json.dump(ideas, f, indent=2)
        
        print(f"✅ Generated {len(ideas)} ideas (template-based). Saved to {ideas_file}")
        return ideas
    
    def generate_reel_script(self, idea):
        """Generate full Reel script from idea."""
        
        script = f"""
# Instagram Reel Script: {idea.get('title', 'GhostWire')}

## Hook (0-3 seconds)
{idea.get('hook', '🌐 GhostWire: Sovereign mesh communication.')}

## Visual Description
{idea.get('visual', 'Show animated mesh network with connecting nodes.')}

## Script (3-15 seconds)
{idea.get('script', 'No servers. No surveillance. No permission. Built in Rust.')}

## Call-to-Action (15-20 seconds)
Link in bio to join the mesh.

## Hashtags
{idea.get('hashtags', '#GhostWire #MeshNetwork #Privacy #Rust #Decentralized')}

## Caption
{idea.get('hook', '')}

{idea.get('script', '')}

🔗 Link in bio

{idea.get('hashtags', '')}
"""
        return script
    
    def save_scripts(self, ideas):
        """Save all Reel scripts to files."""
        scripts_dir = self.output_dir / "scripts"
        scripts_dir.mkdir(exist_ok=True)
        
        for i, idea in enumerate(ideas):
            script = self.generate_reel_script(idea)
            script_file = scripts_dir / f"reel_{i+1:02d}_{idea.get('title', 'idea')[:20]}.txt"
            
            with open(script_file, 'w') as f:
                f.write(script)
        
        print(f"✅ Saved {len(ideas)} scripts to {scripts_dir}")
        return scripts_dir
    
    def generate_all(self, num_ideas=5, render=False):
        """Run full content generation pipeline."""
        
        print("🎨 GhostWire AI Content Generator")
        print("=" * 50)
        
        # Step 1: Analyze codebase
        content_elements = self.analyze_codebase()
        
        # Step 2: Generate ideas
        ideas = self.generate_ideas_with_ai(content_elements, num_ideas)
        
        # Step 3: Generate scripts
        scripts_dir = self.save_scripts(ideas)
        
        # Step 4: Render (if requested)
        if render:
            print("\n🎬 Rendering videos...")
            # This would integrate with the existing rendering system
            print("⚠️  Video rendering not yet implemented in AI generator.")
            print("   Use: ./render_instagram_content.sh")
        
        print("\n" + "=" * 50)
        print("✅ Content generation complete!")
        print(f"📁 Output: {self.output_dir}")
        print(f"📝 Scripts: {scripts_dir}")
        print("=" * 50)


def main():
    parser = argparse.ArgumentParser(description="GhostWire AI Instagram Content Generator")
    parser.add_argument("--repo", default=None, help="Path to GhostWire repository")
    parser.add_argument("--github-token", default=None, help="GitHub API token")
    parser.add_argument("--openai-key", default=None, help="OpenAI API key")
    parser.add_argument("--ideas", type=int, default=5, help="Number of ideas to generate")
    parser.add_argument("--render", action="store_true", help="Render videos after generating scripts")
    parser.add_argument("--analyze-only", action="store_true", help="Only analyze codebase, don't generate ideas")
    
    args = parser.parse_args()
    
    # Initialize generator
    generator = GhostWireAIContentGenerator(
        repo_path=args.repo,
        github_token=args.github_token,
        openai_key=args.openai_key
    )
    
    # Run generation
    if args.analyze_only:
        elements = generator.analyze_codebase()
        print(f"\nFound {len(elements)} content-worthy elements:")
        for elem in elements:
            if "file" in elem:
                print(f"  - {elem['file']}")
    else:
        generator.generate_all(num_ideas=args.ideas, render=args.render)


if __name__ == "__main__":
    main()
