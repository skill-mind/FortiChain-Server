#!/bin/bash

# FortiChain Server Docker Build Script
# ====================================
# This script builds multi-stage Docker images for the FortiChain Server
# supporting both Debian and Alpine Linux base images with Rust nightly
# for edition 2024 support.
#
# Features:
# - Multi-stage builds for minimal production images
# - Size comparison between Debian vs Alpine variants
# - Comprehensive deployment instructions
# - Docker and docker-compose integration
#
# Usage: ./build.sh
#
# Authors: Ebube Onuora, Abraham Ujah

set -e  # Exit on any error

# Color codes for better output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}$1${NC}"
}

print_success() {
    echo -e "${GREEN}$1${NC}"
}

print_warning() {
    echo -e "${YELLOW}$1${NC}"
}

print_error() {
    echo -e "${RED}$1${NC}"
}

echo "🚀 Building FortiChain Server Docker images..."
echo

# Check if Docker needs sudo
DOCKER_CMD="docker"
if ! docker info >/dev/null 2>&1; then
    print_warning "🔐 Docker requires sudo permissions"
    DOCKER_CMD="sudo docker"
    
    # Verify sudo docker works
    if ! sudo docker info >/dev/null 2>&1; then
        print_error "❌ Docker is not available even with sudo. Please ensure Docker is installed and running."
        exit 1
    fi
fi

print_success "✅ Docker is accessible"
echo

# Build the multi-stage image
print_status "📦 Building multi-stage FortiChain Server image..."
if $DOCKER_CMD build -t fortichain-server:latest . > /dev/null 2>&1; then
    image_size=$($DOCKER_CMD images fortichain-server:latest --format "table {{.Size}}" | tail -n +2)
    print_success "✅ Multi-stage image built successfully"
else
    print_error "❌ Failed to build image"
    exit 1
fi
echo

# Show image details
echo "📊 Image Details:"
echo "================="
echo "🏔️  FortiChain Server: $image_size"
echo

# Show all images
echo "📋 FortiChain Server image:"
$DOCKER_CMD images | grep fortichain-server

echo
echo "🎉 Build complete! Choose your deployment method:"
echo
echo "=================="
echo "🚀 RECOMMENDED: Full Stack with PostgreSQL"
echo "=================="
echo "   docker-compose up                    # Start all services (interactive)"
echo "   docker-compose up -d                # Start in background (detached)"
echo "   docker-compose up --build           # Force rebuild and start"
echo "   docker-compose logs -f              # View real-time logs"
echo "   docker-compose down                 # Stop all services"
echo
echo "✅ This automatically handles:"
echo "   • PostgreSQL database setup"
echo "   • All required environment variables"
echo "   • Service networking and health checks"
echo "   • Data persistence"
echo
echo "=================="
echo "🔧 ADVANCED: Manual Container Run"
echo "=================="
echo "⚠️  Requires external PostgreSQL database"
echo
echo "   # Start PostgreSQL first:"
echo "   $DOCKER_CMD run -d -p 5432:5432 \\"
echo "     -e POSTGRES_PASSWORD=password \\"
echo "     -e POSTGRES_DB=fortichain \\"
echo "     --name postgres postgres:16-alpine"
echo
echo "   # Run FortiChain Server:"
echo "   $DOCKER_CMD run -p 8000:8000 \\"
echo "     -e APP_ENVIRONMENT=local \\"
echo "     -e PORT=8000 \\"
echo "     -e DATABASE_URL=postgresql://postgres:password@host.docker.internal:5432/fortichain \\"
echo "     -e DB_MAX_CONNECTIONS=10 \\"
echo "     fortichain-server:latest"
echo
echo "=================="
echo "🧪 Testing & Verification"
echo "=================="
echo "   curl http://localhost:8000/health_check    # Test health endpoint"
echo "   $DOCKER_CMD logs fortichain-server         # View container logs"
echo "   $DOCKER_CMD ps                             # List running containers"
echo
echo "💡 Quick Start: Just run 'docker-compose up' for the easiest setup!"
