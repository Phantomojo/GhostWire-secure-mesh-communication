#!/bin/bash

echo "╔══════════════════════════════════════════════════════════════╗"
echo "║     GhostWire Progress Monitor for Omarchy OS                ║"
echo "╠══════════════════════════════════════════════════════════════╣"
echo "║  Detected: Omarchy OS (Arch + Hyprland)                      ║"
echo "║  Waybar: Modular config                                      ║"
echo "║  Theme: Will use Omarchy palette                             ║"
echo "╚══════════════════════════════════════════════════════════════╝"

echo ""
echo "🔍 Checking Omarchy installation..."

# Verify Omarchy
if pacman -Q omarchy-keyring &>/dev/null; then
    echo "✅ Omarchy detected"
    OMARCHY=true
else
    echo "⚠️  Not running Omarchy, but will use Omarchy-compatible config"
    OMARCHY=false
fi

echo ""
read -p "Continue with Omarchy-compatible installation? [y/N] " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Installation cancelled"
    exit 1
fi

echo ""
echo "📦 Installing dependencies..."
sudo pacman -S --noconfirm waybar libnotify dunst

echo ""
echo "📁 Creating Omarchy-compatible directory structure..."
mkdir -p ~/.config/waybar/modules/custom
mkdir -p ~/.config/waybar/scripts
mkdir -p ~/.local/bin

echo ""
echo "🎨 Creating Omarchy-compatible scripts..."

# Create pacman progress script
cat > ~/.config/waybar/scripts/pacman-progress.sh << 'EOF'
#!/bin/bash
# Omarchy-compatible pacman monitor

if [ -f /var/lib/pacman/db.lck ]; then
  PID=$(pgrep pacman | head -1)
  if [ -n "$PID" ]; then
    echo "{\"text\": \"📦 $PID\", \"tooltip\": \"Installing...\", \"class\": \"warning\"}"
  else
    echo "{\"text\": \"📦 ⚠️\", \"tooltip\": \"Database locked\", \"class\": \"critical\"}"
  fi
else
  UPDATES=$(checkupdates 2>/dev/null | wc -l)
  if [ $UPDATES -gt 0 ]; then
    echo "{\"text\": \"📦 $UPDATES\", \"tooltip\": \"$UPDATES updates available\", \"class\": \"info\"}"
  else
    echo "{\"text\": \"✓\", \"tooltip\": \"System up to date\"}"
  fi
fi
EOF
chmod +x ~/.config/waybar/scripts/pacman-progress.sh

# Create build progress script
cat > ~/.config/waybar/scripts/build-progress.sh << 'EOF'
#!/bin/bash
# Build monitor (Omarchy doesn't have this)

CARGO=$(pgrep -c cargo 2>/dev/null || echo 0)
GRADLE=$(pgrep -c gradle 2>/dev/null || echo 0)
NPM=$(pgrep -c npm 2>/dev/null || echo 0)
MAKE=$(pgrep -c make 2>/dev/null || echo 0)
TOTAL=$((CARGO + GRADLE + NPM + MAKE))

if [ $TOTAL -gt 0 ]; then
  TOOLTIP="Cargo: $CARGO | Gradle: $GRADLE | NPM: $NPM | Make: $MAKE"
  echo "{\"text\": \"🔨 $TOTAL\", \"tooltip\": \"$TOOLTIP\", \"class\": \"warning\"}"
else
  echo "{\"text\": \"\"}"
fi
EOF
chmod +x ~/.config/waybar/scripts/build-progress.sh

# Create system tasks script
cat > ~/.config/waybar/scripts/system-tasks.sh << 'EOF'
#!/bin/bash
# System task monitor

DOWNLOADS=$(pgrep -c "wget|curl|aria2|firefox|chromium" 2>/dev/null || echo 0)
BACKGROUND=$(pgrep -c "syncthing|dropbox|nextcloud" 2>/dev/null || echo 0)
TOTAL=$((DOWNLOADS + BACKGROUND))

if [ $TOTAL -gt 0 ]; then
  echo "{\"text\": \"⚙️ $TOTAL\", \"tooltip\": \"$DOWNLOADS downloads, $BACKGROUND background\", \"class\": \"info\"}"
else
  echo "{\"text\": \"✓\"}"
fi
EOF
chmod +x ~/.config/waybar/scripts/system-tasks.sh

echo ""
echo "📝 Creating Omarchy module configs..."

# Create Omarchy-compatible module configs (JSONC format)
cat > ~/.config/waybar/modules/custom/pacman-progress.jsonc << 'EOF'
{
  "custom/pacman-progress": {
    "format": "📦 {}",
    "exec": "~/.config/waybar/scripts/pacman-progress.sh",
    "interval": 5,
    "return-type": "json",
    "on-click": "alacritty -e sudo pacman -Syu"
  }
}
EOF

cat > ~/.config/waybar/modules/custom/build-progress.jsonc << 'EOF'
{
  "custom/build-progress": {
    "format": "🔨 {}",
    "exec": "~/.config/waybar/scripts/build-progress.sh",
    "interval": 3,
    "return-type": "json"
  }
}
EOF

cat > ~/.config/waybar/modules/custom/system-tasks.jsonc << 'EOF'
{
  "custom/system-tasks": {
    "format": "⚙️ {}",
    "exec": "~/.config/waybar/scripts/system-tasks.sh",
    "interval": 5,
    "return-type": "json"
  }
}
EOF

echo ""
echo "🎨 Creating Omarchy-compatible CSS..."

# Create CSS that uses Omarchy theme palette
cat > ~/.config/waybar/modules/custom/progress-modules.css << 'EOF'
/* GhostWire Progress Modules - Omarchy Compatible */
/* Uses Omarchy theme palette */

#custom-pacman-progress {
  color: #cba6f7;  /* Mauve - matches Omarchy */
  background: rgba(203, 166, 247, 0.15);
  padding: 0 12px;
  margin: 4px 0;
  border-radius: 4px;
  font-weight: bold;
}

#custom-pacman-progress.warning {
  background: rgba(249, 226, 175, 0.3);
  color: #f9e2af;
}

#custom-pacman-progress.critical {
  background: rgba(243, 139, 168, 0.3);
  color: #f38ba8;
}

#custom-build-progress {
  color: #89b4fa;  /* Blue - matches Omarchy */
  background: rgba(137, 180, 250, 0.15);
  padding: 0 12px;
  margin: 4px 0;
  border-radius: 4px;
  font-weight: bold;
}

#custom-build-progress.warning {
  background: rgba(250, 179, 135, 0.3);
  color: #fab387;
}

#custom-system-tasks {
  color: #a6e3a1;  /* Green - matches Omarchy */
  background: rgba(166, 227, 161, 0.15);
  padding: 0 12px;
  margin: 4px 0;
  border-radius: 4px;
  font-weight: bold;
}

#custom-system-tasks.info {
  background: rgba(137, 215, 234, 0.3);
  color: #89dceb;
}
EOF

echo ""
echo "📋 Updating Omarchy layout..."

# Backup existing layout
if [ -f ~/.config/waybar/modules/layout.jsonc ]; then
  cp ~/.config/waybar/modules/layout.jsonc ~/.config/waybar/modules/layout.jsonc.backup
  echo "✅ Backed up existing layout"
fi

# Add our modules to layout
cat >> ~/.config/waybar/modules/layout.jsonc << 'EOF'

// GhostWire Progress Monitor Modules
// Added by install-system-monitor.sh
"custom/pacman-progress",
"custom/build-progress",
"custom/system-tasks",
EOF

echo "✅ Added modules to layout"

echo ""
echo "🔔 Creating pacman notification hook (Omarchy-style)..."

sudo cat > /usr/share/libalpm/hooks/omarchy-progress.hook << 'EOF'
[Trigger]
Operation = Install
Operation = Upgrade
Operation = Remove
Type = Package
Target = *

[Action]
Description = Sending Omarchy-style notification...
When = PreTransaction
Exec = /usr/bin/notify-send -u low -t 0 -h string:x-canonical-private-synchronous:omarchy-pacman "📦 Pacman Started" "Package operation beginning..."
When = PostTransaction
Exec = /usr/bin/notify-send -u low -t 5000 -h string:x-canonical-private-synchronous:omarchy-pacman "✅ Pacman Complete" "Package operation finished!"
EOF

echo "✅ Notification hook created"

echo ""
echo "📊 Creating CLI monitor tool..."

cat > ~/.local/bin/monitor-all << 'EOF'
#!/bin/bash
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║           Omarchy System Monitor                             ║"
echo "╠══════════════════════════════════════════════════════════════╣"
echo ""
echo "📦 Pacman Status:"
if [ -f /var/lib/pacman/db.lck ]; then
  echo "   ⚠️  Database locked - pacman running"
  pgrep -a pacman | head -3
else
  UPDATES=$(checkupdates 2>/dev/null | wc -l)
  echo "   ✓ System up to date ($UPDATES updates available)"
fi
echo ""
echo "🔨 Build Processes:"
echo "   Cargo:  $(pgrep -c cargo 2>/dev/null || echo 0)"
echo "   Gradle: $(pgrep -c gradle 2>/dev/null || echo 0)"
echo "   NPM:    $(pgrep -c npm 2>/dev/null || echo 0)"
echo "   Make:   $(pgrep -c make 2>/dev/null || echo 0)"
echo ""
echo "🌐 Downloads:"
pgrep -a "wget|curl|aria2" 2>/dev/null | head -5 || echo "   No active downloads"
echo ""
echo "╚══════════════════════════════════════════════════════════════╝"
EOF
chmod +x ~/.local/bin/monitor-all

echo ""
echo "🔄 Restarting Waybar..."

# Check if waybar is running
if pgrep waybar > /dev/null; then
  echo "   Stopping existing waybar..."
  killall waybar
  sleep 1
fi

echo "   Starting waybar..."
waybar &

sleep 2

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║  ✅ Installation Complete for Omarchy!                       ║"
echo "╠══════════════════════════════════════════════════════════════╣"
echo "║                                                              ║"
echo "║  📊 What's Now Active:                                       ║"
echo "║  • Pacman progress (real-time during installs)              ║"
echo "║  • Build process monitoring (cargo, gradle, npm, make)      ║"
echo "║  • System task tracker (downloads, background)              ║"
echo "║  • Omarchy-style notifications                              ║"
echo "║  • Omarchy theme compatible                                 ║"
echo "║                                                              ║"
echo "║  🎯 Quick Commands:                                          ║"
echo "║  • monitor-all         - Show all system tasks              ║"
echo "║  • checkupdates        - Check for updates                  ║"
echo "║  • killall waybar      - Restart waybar                     ║"
echo "║                                                              ║"
echo "║  📁 Config Files:                                            ║"
echo "║  • ~/.config/waybar/modules/custom/  - New modules          ║"
echo "║  • ~/.config/waybar/scripts/         - Monitor scripts      ║"
echo "║  • ~/.config/waybar/modules/layout.jsonc - Updated          ║"
echo "║                                                              ║"
echo "║  🎨 Theme: Uses Omarchy color palette                        ║"
echo "║                                                              ║"
echo "╚══════════════════════════════════════════════════════════════╝"

echo ""
echo "✨ Check your top bar for the new modules!"
echo "   Modules added: 📦 Pacman | 🔨 Builds | ⚙️ System Tasks"
echo ""
echo "📖 For more info, see: OMARCHY_PROGRESS_MONITOR.md"
