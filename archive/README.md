# 📦 **Archive Directory**

This directory contains files and directories that were part of the GhostWire project but are no longer actively used in the current version.

## 🗂️ **What's Archived**

### **📁 Scripts (`scripts/`)**
- **Old launcher scripts** - Replaced by `launch-ghostwire-working.sh`
- **Platform-specific scripts** - Windows (.bat), PowerShell (.ps1), etc.
- **Development scripts** - Setup, update, and utility scripts
- **Build scripts** - Alternative build configurations

### **📁 Documentation (`docs/`)**
- **Old documentation** - Superseded by current comprehensive docs
- **Project overview** - Moved to main README.md
- **Roadmap** - Integrated into CHANGELOG.md
- **Architecture PDFs** - Security architecture documents

### **📁 Projects (`meshtastic-rust/`, `meshtastic-web/`)**
- **Meshtastic integration** - Separate projects for mesh networking
- **External dependencies** - Not part of core GhostWire functionality

### **📁 Configuration (`package.json`, `package-lock.json`, `.gitmodules`)**
- **Root package files** - Moved to webui directory
- **Git submodules** - No longer needed

## 🎯 **Why Archived?**

### **✅ Clean Codebase**
- **Reduced complexity** - Easier to navigate and maintain
- **Focused functionality** - Only essential files in root
- **Better organization** - Clear separation of active vs archived

### **✅ Current Focus**
- **All 46 buttons functional** - Main achievement
- **Production ready** - Core functionality complete
- **Essential documentation** - Comprehensive guides

### **✅ Future Reference**
- **Historical context** - Development journey preserved
- **Alternative approaches** - Different implementation methods
- **Integration examples** - External project integrations

## 🔄 **How to Restore**

If you need to restore any archived files:

```bash
# Restore a specific file
cp archive/scripts/launch-ghostwire-complete.sh ./

# Restore entire directory
cp -r archive/docs/ ./

# Restore project
cp -r archive/meshtastic-rust/ ./
```

## 📋 **Archive Contents**

### **Scripts**
- `launch-ghostwire-complete.sh` - Old complete launcher
- `launch-ghostwire-dynamic.sh` - Dynamic features launcher
- `launch-ghostwire-final.sh` - Final version launcher
- `start-ghostwire-complete.sh` - Start script
- `launch-ghostwire.ps1` - PowerShell launcher
- `launch-ghostwire.bat` - Windows batch launcher
- `launch-ghostwire.sh` - Alternative shell launcher
- `start-all.js` - Node.js start script
- `start-all.ps1` - PowerShell start script
- `start-ghostwire.bat` - Windows start script
- `update-ghostwire.bat` - Windows update script
- `update_cursor.sh` - Cursor update script
- `setup-dev.sh` - Development setup script
- `install.sh` - Installation script

### **Documentation**
- `DYNAMIC_FEATURES.md` - Dynamic features documentation
- `FINAL_SUMMARY.md` - Final summary document
- `LAUNCHERS.md` - Launcher documentation
- `PROJECT_OVERVIEW.md` - Project overview
- `ROADMAP.md` - Project roadmap
- `Security Architecture.pdf` - Security architecture document

### **Projects**
- `meshtastic-rust/` - Meshtastic Rust integration
- `meshtastic-web/` - Meshtastic Web integration

### **Configuration**
- `package.json` - Root package configuration
- `package-lock.json` - Package lock file
- `.gitmodules` - Git submodules configuration

## 🎉 **Current Status**

**GhostWire is now clean and focused:**

- ✅ **Essential files only** in root directory
- ✅ **Comprehensive documentation** in main docs
- ✅ **Working launcher** (`launch-ghostwire-working.sh`)
- ✅ **All 46 buttons functional**
- ✅ **Production ready**

**The archive preserves the development history while keeping the current codebase clean and maintainable!** 