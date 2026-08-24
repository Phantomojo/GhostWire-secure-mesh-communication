# 🤝 **Contributing to GhostWire**

<div align="center">

![Contributing](https://img.shields.io/badge/Contributing-Welcome%20🤝-brightgreen?style=for-the-badge)
![Status](https://img.shields.io/badge/Status-Hardening-orange?style=for-the-badge)
![Community](https://img.shields.io/badge/Community-Awesome%20🌟-blue?style=for-the-badge)

**🎯 Help us make GhostWire even better! Every contribution matters!**

</div>

---

## 🌟 **Why Contribute to GhostWire?**

**GhostWire is an ambitious peer-to-peer communication system currently in an intensive security hardening phase.**

We've built a feature-rich prototype, but the Jan 2026 security audit revealed critical areas for improvement. Every contribution to security, testing, and documentation is now high priority!

### **🎯 Current Focus**
- ✅ **Core Components Built** - Functional prototype across all panels
- 🔄 **Security Hardening** - Patching vulnerabilities and removing mock data
- 🔄 **Test Coverage** - Increasing from <10% to 50%+
- ✅ **Dual Licensing** - AGPL-3.0 + Commercial License for sustainable development

### **🚀 What We're Building Next**
- 🔥 **Enhanced Security Features** - More advanced threat detection
- 🌐 **Mobile Support** - iOS and Android apps
- 🔌 **Plugin System** - Extensible architecture
- 🎨 **UI/UX Improvements** - Better user experience
- 📊 **Analytics Dashboard** - Real-time system monitoring
- 🔗 **Protocol Adapters** - Bridge to other mesh networks

---

## 📜 **Licensing & Legal**

### **Dual Licensing Model**

GhostWire is dual-licensed under:
- **AGPL-3.0-or-later** - For open source and community use
- **Commercial License** - For proprietary deployments

**Your Contributions:**
- By contributing, you agree that your contributions will be dual-licensed under the same terms
- This allows the project to offer both AGPL and commercial licenses
- See [LICENSE_EXPLANATION.md](../LICENSE_EXPLANATION.md) for details

### **Contributor License Agreement (CLA)**

**Coming Soon**: We're implementing a Contributor License Agreement (CLA) to:
- Clarify licensing of your contributions
- Ensure we can dual-license the project
- Protect your rights as a contributor
- Maintain legal clarity for all users

**Until CLA is ready:**
- By submitting a PR, you agree to dual-license your contributions
- You represent that you have the right to license your work
- You agree to the terms in this CONTRIBUTING.md

### **License Compatibility**

When contributing third-party code:
- ✅ Must be compatible with AGPL-3.0
- ✅ Must allow dual licensing
- ❌ Cannot include Commons Clause or similar restrictions
- ❌ Cannot be GPL-2.0 only (must be GPL-3.0 compatible or better)
- ✅ Apache 2.0, MIT, BSD-3-Clause are generally compatible

**Always check with maintainers before adding dependencies!**

## 🚀 **Quick Start - Get Contributing in 5 Minutes**

### **1. Fork & Clone**
```bash
# Fork the repository on GitHub
# Then clone your fork
git clone https://github.com/YOUR_USERNAME/GhostWire-secure-mesh-communication.git
cd GhostWire-secure-mesh-communication

# Add the original repository as upstream
git remote add upstream https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication.git
```

### **2. Setup Development Environment**
```bash
# Make launcher executable
chmod +x launch-ghostwire-working.sh

# Run the system to test it works
./launch-ghostwire-working.sh

# Open your browser and test ALL buttons work
# This ensures you have a working baseline
```

### **3. Create Your Feature Branch**
```bash
# Create and switch to a new branch
git checkout -b feature/amazing-new-feature

# Make your changes
# Test everything works

# Commit your changes
git commit -m "Add amazing new feature"

# Push to your fork
git push origin feature/amazing-new-feature
```

### **4. Submit Pull Request**
- Go to your fork on GitHub
- Click "New Pull Request"
- Select your feature branch
- Write a clear description of your changes
- Submit!

---

## 🎯 **What We're Looking For**

### **🔥 High Priority**
- **Bug Fixes** - Any issues you find
- **Security Improvements** - Enhanced threat detection, encryption
- **Performance Optimizations** - Faster, more efficient code
- **UI/UX Improvements** - Better user experience
- **Documentation** - Better guides, tutorials, examples

### **🌟 Medium Priority**
- **New Features** - Additional functionality
- **Testing** - Unit tests, integration tests
- **Code Quality** - Refactoring, better architecture
- **Cross-Platform Support** - Windows, macOS, mobile

### **💡 Low Priority**
- **Cosmetic Changes** - Minor UI tweaks
- **Documentation Updates** - Typos, clarifications
- **Code Comments** - Better code documentation

---

## 🛠️ **Development Setup**

### **Prerequisites**
- **Node.js** (v16 or higher)
- **npm** (comes with Node.js)
- **Git** (for version control)
- **Rust** (for backend development - optional)

### **Full Development Environment**
```bash
# Clone the repository
git clone https://github.com/YOUR_USERNAME/GhostWire-secure-mesh-communication.git
cd GhostWire-secure-mesh-communication

# Install frontend dependencies
cd webui
npm install
cd ..

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build backend (optional - pre-built binary available)
cd ghostwire
cargo build --bin ghostwire --features web
cd ..

# Run the complete system
./launch-ghostwire-working.sh
```

### **Frontend Development**
```bash
# Start frontend development server
cd webui
npm run dev

# The frontend will be available at http://localhost:5173
# Make sure the backend is running for full functionality
```

### **Backend Development**
```bash
# Start backend development server
cd ghostwire
cargo run --bin ghostwire --features web -- --host 0.0.0.0 --port 3001

# The backend API will be available at http://localhost:3001/api
```

---

## 🧪 **Testing Your Changes**

### **Frontend Testing**
```bash
# Run frontend tests
cd webui
npm test

# Run linting
npm run lint

# Build for production
npm run build
```

### **Backend Testing**
```bash
# Run backend tests
cd ghostwire
cargo test

# Run with specific features
cargo test --features web
```

### **Integration Testing**
```bash
# Run the complete system
./launch-ghostwire-working.sh

# Test ALL buttons work:
# 1. Network Panel - Scan, connect, broadcast
# 2. Security Panel - Rotate keys, scan, lockdown
# 3. Control Panel - Refresh, backup, restart
# 4. Communication Panel - Send messages, sync
# 5. System Panel - Diagnostics, tests
```

### **Cross-PC Testing**
```bash
# Test on multiple PCs in your network
# 1. Run launcher on PC 1
# 2. Note the network URL (e.g., http://192.168.100.242:5175)
# 3. Open URL on PC 2
# 4. Configure backend URL to PC 1's backend
# 5. Test communication between PCs
```

---

## 📝 **Code Style & Standards**

### **Frontend (React/TypeScript)**
- **TypeScript** - Use TypeScript for all new code
- **Functional Components** - Use React hooks and functional components
- **ESLint** - Follow the existing ESLint configuration
- **Prettier** - Use Prettier for code formatting
- **Component Structure** - Follow the existing component patterns

### **Backend (Rust)**
- **Rust 2021 Edition** - Use the latest Rust edition
- **Async/Await** - Use async/await for all I/O operations
- **Error Handling** - Use proper error handling with `Result<T, E>`
- **Documentation** - Add documentation comments for public APIs
- **Tests** - Write tests for new functionality

### **General**
- **Commit Messages** - Use clear, descriptive commit messages
- **Pull Request Titles** - Use descriptive titles
- **Code Comments** - Add comments for complex logic
- **Documentation** - Update documentation for new features

---

## 🎯 **Contribution Guidelines**

### **Before You Start**
1. **Check Issues** - Look for existing issues or create a new one
2. **Discuss Changes** - Comment on issues to discuss your approach
3. **Keep It Small** - Make small, focused changes
4. **Test Everything** - Ensure all buttons still work

### **Pull Request Process**
1. **Fork the Repository** - Create your own fork
2. **Create Feature Branch** - Use descriptive branch names
3. **Make Your Changes** - Follow the code style guidelines
4. **Test Thoroughly** - Ensure everything works
5. **Update Documentation** - Update relevant documentation
6. **Submit Pull Request** - Use the PR template

### **Pull Request Template**
```markdown
## 🎯 **Description**
Brief description of your changes

## 🔧 **Changes Made**
- [ ] Feature A added
- [ ] Bug B fixed
- [ ] Documentation C updated

## 🧪 **Testing**
- [ ] All buttons still work
- [ ] Cross-PC communication tested
- [ ] No new errors introduced

## 📸 **Screenshots** (if applicable)
Add screenshots of UI changes

## 🔗 **Related Issues**
Closes #123
```

---

## 🏆 **Recognition & Rewards**

### **Contributor Levels**
- **🌟 New Contributor** - First contribution
- **🔥 Active Contributor** - 5+ contributions
- **💎 Core Contributor** - 20+ contributions
- **👑 Maintainer** - Significant contributions

### **Recognition**
- **Contributor Hall of Fame** - Listed in README
- **GitHub Stars** - Recognition for great work
- **Community Appreciation** - Thanks from the community
- **Learning Opportunities** - Work on cutting-edge tech

---

## 🐛 **Reporting Bugs**

### **Bug Report Template**
```markdown
## 🐛 **Bug Description**
Clear description of the bug

## 🔄 **Steps to Reproduce**
1. Step 1
2. Step 2
3. Step 3

## 📱 **Expected Behavior**
What should happen

## ❌ **Actual Behavior**
What actually happens

## 🖥️ **Environment**
- OS: [e.g., Linux, Windows, macOS]
- Browser: [e.g., Chrome, Firefox, Safari]
- Node.js Version: [e.g., 16.14.0]
- GhostWire Version: [e.g., latest commit]

## 📸 **Screenshots** (if applicable)
Add screenshots showing the bug

## 🔗 **Additional Context**
Any other relevant information
```

---

## 💡 **Feature Requests**

### **Feature Request Template**
```markdown
## 💡 **Feature Description**
Clear description of the feature

## 🎯 **Problem Statement**
What problem does this solve?

## 💭 **Proposed Solution**
How should this work?

## 🔗 **Alternative Solutions**
Other ways to solve this problem

## 📊 **Impact**
How many users would benefit?

## 🎨 **Mockups** (if applicable)
Add mockups or wireframes
```

---

## 🤝 **Community Guidelines**

### **Be Respectful**
- **Respect Others** - Be kind and respectful to all contributors
- **Constructive Feedback** - Provide helpful, constructive feedback
- **Inclusive Language** - Use inclusive language in all communications

### **Be Helpful**
- **Answer Questions** - Help other contributors with their questions
- **Share Knowledge** - Share your knowledge and experience
- **Mentor Newcomers** - Help new contributors get started

### **Be Professional**
- **Quality Code** - Write high-quality, maintainable code
- **Good Documentation** - Write clear, helpful documentation
- **Follow Standards** - Follow established coding standards

---

## 📞 **Getting Help**

### **Resources**
- **Codeberg issues** - [Report bugs or ask questions](https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication/issues)
- **Codeberg discussions** - [Join the community](https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication/discussions)
- **Documentation** - [Read the docs](https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication/wiki)
- **Code Examples** - [Look at existing code](https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication/tree/main/webui/src)

### **Contact**
- **Maintainers** - @Phantomojo and the core team
- **Community** - Other contributors and users
- **Security Issues** - Report security issues privately

---

## 🎉 **Thank You!**

**Thank you for contributing to GhostWire!** 

Your contributions help make GhostWire the best peer-to-peer communication system in the world. Every button works, every feature matters, and every contributor is valued!

**Together, we're building the future of secure, decentralized communication!** 🚀

---

<div align="center">

**🎯 Ready to contribute? Start by forking the repository and running `./launch-ghostwire-working.sh` to see all the working buttons!**

[![Fork on GitHub](https://img.shields.io/github/forks/Phantomojo/GhostWire-secure-mesh-communication?style=social&label=Fork)](https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication/network)
[![Star on GitHub](https://img.shields.io/github/stars/Phantomojo/GhostWire-secure-mesh-communication?style=social&label=Star)](https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication/stargazers)

</div> 