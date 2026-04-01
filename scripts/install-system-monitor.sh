#!/bin/bash

╔══════════════════════════════════════════════════════════════╗
║     GhostWire System-Wide Progress Monitor Installer         ║
╠══════════════════════════════════════════════════════════════╣
║  This will install and configure:                            ║
║  ✅ Waybar with custom modules                               ║
║  ✅ Pacman progress notifications                            ║
║  ✅ Build process monitoring                                 ║
║  ✅ System task tracker                                      ║
║  ✅ Catppuccin theme integration                             ║
╚══════════════════════════════════════════════════════════════╝

echo ""
read -p "Continue with installation? [y/N] " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]
then
    echo "❌ Installation cancelled"
    exit 1
fi

echo ""
echo "📦 Installing dependencies..."
sudo pacman -S --noconfirm waybar libnotify dunst

echo ""
echo "📁 Creating configuration directories..."
mkdir -p ~/.config/waybar/scripts
mkdir -p ~/.local/bin

echo ""
echo "📝 Creating pacman progress script..."
cat > ~/.config/waybar/scripts/pacman-progress.sh << 'EOF'
#!/bin/bash
if [ -f /var/lib/pacman/db.lck ]; then
  PID=$(pgrep pacman | head -1)
  if [ -n "$PID" ]; then
    echo "{\"text\": \"📦 $PID\", \"tooltip\": \"Pacman running (PID: $PID)\", \"class\": \"warning\"}"
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

echo ""
echo "🔨 Creating build monitor script..."
cat > ~/.config/waybar/scripts/build-progress.sh << 'EOF'
#!/bin/bash
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

echo ""
echo "⚙️  Creating system tasks monitor..."
cat > ~/.config/waybar/scripts/system-tasks.sh << 'EOF'
#!/bin/bash
DOWNLOADS=$(pgrep -c "wget|curl|aria2" 2>/dev/null || echo 0)
BROWSERS=$(pgrep -c "firefox|chrome|chromium" 2>/dev/null || echo 0)
TOTAL=$((DOWNLOADS + BROWSERS))

if [ $TOTAL -gt 0 ]; then
  echo "{\"text\": \"🌐 $TOTAL\", \"tooltip\": \"$DOWNLOADS downloads, $BROWSERS browser tabs\", \"class\": \"info\"}"
else
  echo "{\"text\": \"✓\"}"
fi
EOF
chmod +x ~/.config/waybar/scripts/system-tasks.sh

echo ""
echo "🎨 Creating Catppuccin-themed waybar config..."
cat > ~/.config/waybar/config << 'EOF'
{
  "layer": "top",
  "position": "top",
  "height": 32,
  "modules-left": [
    "hyprland/workspaces",
    "custom/pacman_progress",
    "custom/build_progress"
  ],
  "modules-center": [
    "hyprland/window"
  ],
  "modules-right": [
    "custom/system_tasks",
    "cpu",
    "memory",
    "network",
    "pulseaudio",
    "clock",
    "tray"
  ],
  "custom/pacman_progress": {
    "format": "{}",
    "exec": "~/.config/waybar/scripts/pacman-progress.sh",
    "interval": 5,
    "return-type": "json",
    "on-click": "alacritty -e sudo pacman -Syu"
  },
  "custom/build_progress": {
    "format": "{}",
    "exec": "~/.config/waybar/scripts/build-progress.sh",
    "interval": 3,
    "return-type": "json"
  },
  "custom/system_tasks": {
    "format": "{}",
    "exec": "~/.config/waybar/scripts/system-tasks.sh",
    "interval": 5,
    "return-type": "json"
  },
  "cpu": {
    "format": "🖥️  {usage}%",
    "interval": 5
  },
  "memory": {
    "format": "🧠 {}%",
    "interval": 5
  },
  "network": {
    "format": "{ifname}",
    "format-wifi": "📶 {essid}",
    "format-ethernet": "🔌 {ifname}",
    "interval": 5
  },
  "pulseaudio": {
    "format": "{icon} {volume}%",
    "format-bluetooth": "{icon} {volume}%",
    "format-icons": {
      "headphone": "🎧",
      "hands-free": "📞",
      "headset": "🎧",
      "phone": "📞",
      "portable": "📱",
      "car": "🚗",
      "default": ["🔈", "🔉", "🔊"]
    }
  },
  "clock": {
    "format": "📅 %H:%M",
    "tooltip": "true",
    "tooltip-format": "<big>{:%Y %B}</big>\n<tt><small>{calendar}</small></tt>"
  },
  "tray": {
    "icon-size": 18,
    "spacing": 8
  }
}
EOF

echo ""
echo "🎨 Creating Catppuccin theme..."
cat > ~/.config/waybar/style.css << 'EOF'
* {
  font-family: "JetBrainsMono Nerd Font", monospace;
  font-size: 13px;
  font-weight: bold;
}

window#waybar {
  background: rgba(30, 30, 46, 0.95);
  color: #cdd6f4;
  border-bottom: 2px solid #cba6f7;
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.3);
}

#workspaces button {
  padding: 0 10px;
  color: #cba6f7;
  background: transparent;
}

#workspaces button.active {
  background: #cba6f7;
  color: #1e1e2e;
}

#custom-pacman_progress {
  color: #f9e2af;
  background: rgba(249, 226, 175, 0.15);
  padding: 0 12px;
  margin: 4px 0;
  border-radius: 4px;
}

#custom-pacman_progress.warning {
  background: rgba(249, 226, 175, 0.3);
  color: #f38ba8;
}

#custom-build_progress {
  color: #89b4fa;
  background: rgba(137, 180, 250, 0.15);
  padding: 0 12px;
  margin: 4px 0;
  border-radius: 4px;
}

#custom-build_progress.warning {
  background: rgba(137, 180, 250, 0.3);
  color: #fab387;
}

#custom-system_tasks {
  color: #a6e3a1;
  background: rgba(166, 227, 161, 0.15);
  padding: 0 12px;
  margin: 4px 0;
  border-radius: 4px;
}

#cpu {
  color: #f38ba8;
  padding: 0 8px;
}

#memory {
  color: #fab387;
  padding: 0 8px;
}

#network {
  color: #89dceb;
  padding: 0 8px;
}

#pulseaudio {
  color: #b4befe;
  padding: 0 8px;
}

#clock {
  color: #cdd6f4;
  background: rgba(203, 166, 247, 0.2);
  padding: 0 12px;
  margin: 4px 0;
  border-radius: 4px;
}

#tray {
  padding: 0 8px;
}

#window {
  color: #bac2de;
  padding: 0 8px;
}
EOF

echo ""
echo "🔔 Creating pacman notification hook..."
sudo cat > /usr/share/libalpm/hooks/notify.hook << 'EOF'
[Trigger]
Operation = Install
Operation = Upgrade
Operation = Remove
Type = Package
Target = *

[Action]
Description = Sending notification...
When = PreTransaction
Exec = /usr/bin/notify-send -u low "📦 Pacman Started" "Package operation beginning..."
When = PostTransaction
Exec = /usr/bin/notify-send -u low "✅ Pacman Complete" "Package operation finished!"
EOF

echo ""
echo "📊 Creating system monitor wrapper..."
cat > ~/.local/bin/monitor-all << 'EOF'
#!/bin/bash
echo "╔══════════════════════════════════════════════════════════════╗"
echo "║           System-Wide Progress Monitor Active                ║"
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
echo "🔄 Restarting waybar..."
killall waybar 2>/dev/null
sleep 1
waybar &

echo ""
echo "╔══════════════════════════════════════════════════════════════╗"
║  ✅ Installation Complete!                                       ║
╠══════════════════════════════════════════════════════════════╣
║                                                              ║
║  📊 What's Now Active:                                       ║
║  • Waybar with custom progress modules                       ║
║  • Pacman progress notifications                             ║
║  • Build process monitoring                                  ║
║  • System task tracker                                       ║
║  • Catppuccin theme                                          ║
║                                                              ║
║  🎯 Quick Commands:                                          ║
║  • monitor-all         - Show all system tasks              ║
║  • checkupdates        - Check for updates                  ║
║  • killall waybar      - Restart waybar                     ║
║                                                              ║
║  📁 Config Files:                                            ║
║  • ~/.config/waybar/config      - Waybar configuration      ║
║  • ~/.config/waybar/style.css   - Theme                     ║
║  • ~/.config/waybar/scripts/    - Monitor scripts           ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝

echo ""
echo "🎨 Theme: Catppuccin"
echo "🔄 Waybar will auto-restart in 3 seconds..."
sleep 3
killall waybar && waybar &

echo ""
echo "✨ All done! Check your top bar for the new modules!"
