# Contributing to GhostWire

Thank you for your interest in contributing to GhostWire! This document provides guidelines and information for contributors.

## 🤝 How to Contribute

### Types of Contributions

We welcome various types of contributions:

- **Code Contributions**: Bug fixes, new features, improvements
- **Documentation**: Improving docs, adding examples, fixing typos
- **Testing**: Adding tests, improving test coverage
- **Security**: Security reviews, vulnerability reports
- **Design**: UI/UX improvements, user experience enhancements
- **Community**: Helping other users, answering questions

### Before You Start

1. **Read the Documentation**: Familiarize yourself with the project
2. **Check Existing Issues**: Look for existing issues or discussions
3. **Join the Community**: Participate in discussions and forums
4. **Understand the Codebase**: Review the architecture and code structure

## 🛠️ Development Setup

### Prerequisites

- **Rust**: Version 1.70 or higher
- **Cargo**: Rust package manager
- **Git**: Version control
- **Code Editor**: VS Code, Vim, Emacs, or your preferred editor

### Getting Started

```bash
# Fork the repository
# Clone your fork
git clone https://github.com/yourusername/ghostwire.git
cd ghostwire

# Add the upstream repository
git remote add upstream https://github.com/original/ghostwire.git

# Create a development branch
git checkout -b feature/your-feature-name

# Install dependencies
cargo build

# Run tests
cargo test
```

### Development Environment

#### Recommended Tools

- **rust-analyzer**: Rust language server for your editor
- **cargo-watch**: Auto-run tests on file changes
- **cargo-audit**: Security vulnerability checking
- **cargo-tarpaulin**: Code coverage analysis

#### VS Code Extensions

```json
{
  "recommendations": [
    "rust-lang.rust-analyzer",
    "serayuzgur.crates",
    "tamasfe.even-better-toml",
    "ms-vscode.vscode-json"
  ]
}
```

## 📝 Code Style and Standards

### Rust Code Style

We follow Rust community standards:

- **Formatting**: Use `cargo fmt` for consistent formatting
- **Linting**: Use `cargo clippy` for code quality checks
- **Documentation**: Document all public APIs with doc comments
- **Error Handling**: Use `anyhow` for error handling
- **Async Code**: Use `async/await` for asynchronous operations

### Code Style Guidelines

#### Naming Conventions
- **Functions**: `snake_case`
- **Variables**: `snake_case`
- **Constants**: `SCREAMING_SNAKE_CASE`
- **Types**: `PascalCase`
- **Modules**: `snake_case`

#### Documentation Standards
```rust
/// Brief description of the function.
///
/// # Arguments
/// * `param1` - Description of parameter 1
/// * `param2` - Description of parameter 2
///
/// # Returns
/// Description of return value
///
/// # Examples
/// ```
/// use ghostwire::core::identity::Identity;
/// let identity = Identity::new()?;
/// ```
///
/// # Errors
/// * `ErrorType` - When something goes wrong
pub fn example_function(param1: String, param2: u32) -> Result<()> {
    // Implementation
}
```

#### Error Handling
```rust
use anyhow::{Context, Result};

pub fn process_data(data: &[u8]) -> Result<ProcessedData> {
    let parsed = parse_data(data)
        .context("Failed to parse input data")?;
    
    let processed = transform_data(parsed)
        .context("Failed to transform data")?;
    
    Ok(processed)
}
```

### Security Guidelines

#### Cryptographic Code
- **Use Established Libraries**: Use `ring`, `aes-gcm`, etc.
- **Avoid Custom Crypto**: Don't implement cryptographic algorithms
- **Secure Random**: Use `getrandom` for random number generation
- **Key Management**: Follow secure key management practices

#### Input Validation
```rust
pub fn validate_input(input: &str) -> Result<()> {
    if input.is_empty() {
        return Err(anyhow::anyhow!("Input cannot be empty"));
    }
    
    if input.len() > MAX_INPUT_LENGTH {
        return Err(anyhow::anyhow!("Input too long"));
    }
    
    // Additional validation
    Ok(())
}
```

## 🧪 Testing

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_function_name() {
        // Arrange
        let input = "test data";
        
        // Act
        let result = function_name(input);
        
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_value);
    }
    
    #[tokio::test]
    async fn test_async_function() {
        // Async test implementation
    }
}
```

### Test Guidelines

- **Unit Tests**: Test individual functions and methods
- **Integration Tests**: Test module interactions
- **Property Tests**: Use `proptest` for property-based testing
- **Mocking**: Use `mockall` for mocking dependencies
- **Coverage**: Aim for >80% code coverage

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture

# Run tests in parallel
cargo test -- --test-threads=4

# Run tests with coverage
cargo tarpaulin --out Html
```

## 🔒 Security Contributions

### Security Review Process

1. **Code Review**: All code changes undergo security review
2. **Static Analysis**: Automated security scanning
3. **Dynamic Testing**: Runtime security testing
4. **Dependency Audit**: Regular dependency vulnerability checks

### Security Testing

```bash
# Check for vulnerabilities
cargo audit

# Run security-focused tests
cargo test security

# Static analysis
cargo clippy -- -D warnings

# Fuzzing (if applicable)
cargo fuzz run
```

## 📚 Documentation

### Documentation Standards

- **API Documentation**: Document all public APIs
- **Examples**: Include usage examples
- **Error Cases**: Document error conditions
- **Security Notes**: Include security considerations

### Documentation Structure

```
docs/
├── api/                    # API documentation
├── guides/                 # User guides
├── security/              # Security documentation
├── development/           # Developer documentation
└── examples/              # Code examples
```

### Building Documentation

```bash
# Build documentation
cargo doc

# Build documentation with private items
cargo doc --document-private-items

# Serve documentation locally
cargo doc --open
```

## 🚀 Submitting Contributions

### Pull Request Process

1. **Create Issue**: Create an issue describing your contribution
2. **Fork Repository**: Fork the repository to your account
3. **Create Branch**: Create a feature branch from `main`
4. **Make Changes**: Implement your changes
5. **Add Tests**: Add tests for new functionality
6. **Update Documentation**: Update relevant documentation
7. **Run Checks**: Ensure all checks pass
8. **Submit PR**: Create a pull request

### Pull Request Guidelines

#### Title Format
```
type(scope): brief description

Examples:
feat(mesh): add peer discovery via mDNS
fix(security): resolve timing attack vulnerability
docs(api): update encryption API documentation
```

#### Description Template
```markdown
## Description
Brief description of the changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing completed

## Security
- [ ] Security implications considered
- [ ] No security vulnerabilities introduced

## Checklist
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] All checks pass
```

### Review Process

1. **Automated Checks**: CI/CD pipeline runs automatically
2. **Code Review**: At least one maintainer reviews the PR
3. **Security Review**: Security implications are assessed
4. **Testing**: Changes are tested in various environments
5. **Approval**: PR is approved and merged

## 🏷️ Issue Labels

We use the following labels for issues:

- **bug**: Something isn't working
- **enhancement**: New feature or request
- **documentation**: Improvements or additions to documentation
- **good first issue**: Good for newcomers
- **help wanted**: Extra attention is needed
- **security**: Security-related issues
- **wontfix**: This will not be worked on

## 🎯 Good First Issues

If you're new to the project, look for issues labeled `good first issue`. These are typically:

- Documentation improvements
- Simple bug fixes
- Test additions
- Code formatting
- Minor feature additions

## 📞 Getting Help

### Communication Channels

- **GitHub Issues**: For bug reports and feature requests
- **GitHub Discussions**: For questions and general discussion
- **Discord**: For real-time chat and community support
- **Email**: For security issues (security@ghostwire.dev)

### Asking Questions

When asking questions, please include:

- **Environment**: OS, Rust version, GhostWire version
- **Error Messages**: Full error messages and stack traces
- **Steps to Reproduce**: Clear steps to reproduce the issue
- **Expected vs Actual**: What you expected vs what happened

## 🏆 Recognition

### Contributors

We recognize contributors in several ways:

- **Contributors List**: All contributors are listed in the repository
- **Release Notes**: Contributors are credited in release notes
- **Blog Posts**: Featured contributors in blog posts
- **Community Awards**: Recognition for significant contributions

### Contribution Levels

- **Bronze**: 1-5 contributions
- **Silver**: 6-20 contributions
- **Gold**: 21-50 contributions
- **Platinum**: 50+ contributions

## 📄 License

By contributing to GhostWire, you agree that your contributions will be licensed under the MIT License.

## 🚨 Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please read our [Code of Conduct](CODE_OF_CONDUCT.md) for details.

---

Thank you for contributing to GhostWire! Your contributions help make secure, private communication accessible to everyone. 