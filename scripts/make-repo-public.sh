#!/bin/bash
#
# 🚀 GhostWire - Make Repository Public Script
# 
# This script uses GitHub CLI (gh) to make the GhostWire repository public
# with proper settings for controlled access and abuse prevention.
#
# Prerequisites:
# - GitHub CLI (gh) installed and authenticated
# - Owner/admin permissions on the repository
#
# Usage:
#   ./scripts/make-repo-public.sh [--dry-run] [--force]
#

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO_OWNER="Phantomojo"
REPO_NAME="GhostWire-secure-mesh-communication"
REPO_FULL="${REPO_OWNER}/${REPO_NAME}"

# Flags
DRY_RUN=false
FORCE=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --dry-run)
            DRY_RUN=true
            shift
            ;;
        --force)
            FORCE=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [--dry-run] [--force]"
            echo ""
            echo "Options:"
            echo "  --dry-run    Show what would be done without making changes"
            echo "  --force      Skip confirmation prompts"
            echo "  -h, --help   Show this help message"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check if gh is installed
    if ! command -v gh &> /dev/null; then
        log_error "GitHub CLI (gh) is not installed. Please install it first."
        echo "Visit: https://cli.github.com/"
        exit 1
    fi
    
    # Check if authenticated
    if ! gh auth status &> /dev/null; then
        log_error "Not authenticated with GitHub CLI. Please run: gh auth login"
        exit 1
    fi
    
    # Check if repo exists and we have access
    if ! gh repo view "$REPO_FULL" &> /dev/null; then
        log_error "Cannot access repository: $REPO_FULL"
        echo "Make sure you have owner/admin permissions."
        exit 1
    fi
    
    log_success "Prerequisites check passed"
}

get_current_settings() {
    log_info "Fetching current repository settings..."
    
    local visibility
    visibility=$(gh repo view "$REPO_FULL" --json visibility --jq '.visibility')
    
    local is_fork
    is_fork=$(gh repo view "$REPO_FULL" --json isFork --jq '.isFork')
    
    local default_branch
    default_branch=$(gh repo view "$REPO_FULL" --json defaultBranchRef --jq '.defaultBranchRef.name')
    
    echo "  Visibility: $visibility"
    echo "  Is Fork: $is_fork"
    echo "  Default Branch: $default_branch"
    
    if [[ "$visibility" == "PUBLIC" ]]; then
        log_warning "Repository is already public!"
        return 1
    fi
    
    return 0
}

confirm_action() {
    if [[ "$FORCE" == true ]]; then
        return 0
    fi
    
    echo ""
    log_warning "You are about to make this repository PUBLIC."
    echo ""
    echo "This means:"
    echo "  ✅ Anyone can view the code"
    echo "  ✅ Anyone can fork the repository"
    echo "  ✅ Anyone can create issues"
    echo "  ⚠️  All code will be visible to the public"
    echo "  ⚠️  License terms (AGPL-3.0 + Commercial) will apply"
    echo ""
    read -p "Are you sure you want to continue? (yes/no): " confirm
    
    if [[ "$confirm" != "yes" ]]; then
        log_info "Aborted by user"
        exit 0
    fi
}

make_repo_public() {
    log_info "Making repository public..."
    
    if [[ "$DRY_RUN" == true ]]; then
        log_warning "[DRY RUN] Would execute: gh repo edit $REPO_FULL --accept-visibility-change-consequences --visibility public"
        return 0
    fi
    
    # Make repository public
    if gh repo edit "$REPO_FULL" --accept-visibility-change-consequences --visibility public; then
        log_success "Repository is now PUBLIC"
    else
        log_error "Failed to make repository public"
        exit 1
    fi
}

configure_settings() {
    log_info "Configuring repository settings..."
    
    if [[ "$DRY_RUN" == true ]]; then
        log_warning "[DRY RUN] Would configure repository settings"
        return 0
    fi
    
    # Enable issues
    log_info "Enabling issues..."
    gh repo edit "$REPO_FULL" --enable-issues true || log_warning "Failed to enable issues"
    
    # Enable projects (if available)
    log_info "Enabling projects..."
    gh repo edit "$REPO_FULL" --enable-projects true || log_warning "Failed to enable projects"
    
    # Enable wiki
    log_info "Enabling wiki..."
    gh repo edit "$REPO_FULL" --enable-wiki true || log_warning "Failed to enable wiki"
    
    # Set homepage
    log_info "Setting homepage..."
    gh repo edit "$REPO_FULL" --homepage "https://ghostwire.com" || log_warning "Failed to set homepage"
    
    log_success "Repository settings configured"
}

verify_visibility() {
    log_info "Verifying repository visibility..."
    
    local visibility
    visibility=$(gh repo view "$REPO_FULL" --json visibility --jq '.visibility')
    
    if [[ "$visibility" == "PUBLIC" ]]; then
        log_success "Repository visibility confirmed: PUBLIC"
        return 0
    else
        log_error "Repository visibility is still: $visibility"
        return 1
    fi
}

show_post_setup_tasks() {
    echo ""
    log_success "Repository is now PUBLIC!"
    echo ""
    echo "📋 Next Steps:"
    echo ""
    echo "1. ✅ Verify license files are in place:"
    echo "     - LICENSE (AGPL-3.0 + Commercial notice)"
    echo "     - COMMERCIAL_LICENSE.md"
    echo "     - LICENSE_EXPLANATION.md"
    echo ""
    echo "2. ✅ Verify documentation:"
    echo "     - README.md (with license badges)"
    echo "     - CODE_OF_CONDUCT.md"
    echo "     - SUPPORT.md"
    echo "     - SECURITY.md"
    echo "     - CONTRIBUTING.md"
    echo ""
    echo "3. ⚙️  Configure GitHub Settings (via Web UI):"
    echo "     - Enable branch protection on main branch"
    echo "     - Configure required status checks"
    echo "     - Set up issue templates"
    echo "     - Set up pull request templates"
    echo "     - Configure GitHub Actions permissions"
    echo ""
    echo "4. 🛡️  Security Settings:"
    echo "     - Enable Dependabot alerts"
    echo "     - Enable Dependabot security updates"
    echo "     - Consider enabling secret scanning"
    echo "     - Review and revoke unnecessary tokens"
    echo ""
    echo "5. 📢 Announce:"
    echo "     - Update social media"
    echo "     - Notify community"
    echo "     - Consider a release announcement"
    echo ""
    echo "📖 Documentation:"
    echo "   - LICENSE_EXPLANATION.md - Full licensing strategy"
    echo "   - COMMERCIAL_LICENSE.md - Commercial licensing terms"
    echo ""
    echo "🔗 Repository URL:"
    echo "   https://github.com/${REPO_FULL}"
    echo ""
}

# Main execution
main() {
    echo ""
    echo "========================================"
    echo "🚀 GhostWire - Make Repository Public"
    echo "========================================"
    echo ""
    
    check_prerequisites
    
    if ! get_current_settings; then
        log_info "No changes needed"
        exit 0
    fi
    
    confirm_action
    
    make_repo_public
    
    configure_settings
    
    if verify_visibility; then
        show_post_setup_tasks
    else
        log_error "Verification failed. Please check manually."
        exit 1
    fi
    
    log_success "Done! Repository is now public."
}

# Run main function
main
