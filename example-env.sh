#!/bin/bash

# Example environment configuration for isolate-sandbox
# This script demonstrates how to configure sandbox resource limits

echo "Setting up isolate-sandbox environment with custom resource limits..."

# Server configuration
export ISOLATE_SANDBOX_PORT=3000
export ISOLATE_SANDBOX_CONFIG_DIR="./config"
export ISOLATE_SANDBOX_BOX_POOL_SIZE=10
export ISOLATE_SANDBOX_API_KEY="your-api-key-here"

# Sandbox resource limits
export ISOLATE_SANDBOX_DEFAULT_CG_MEM=262144      # 256MB
export ISOLATE_SANDBOX_DEFAULT_MEM=256000        # 256MB
export ISOLATE_SANDBOX_DEFAULT_TIME=15           # 15 seconds
export ISOLATE_SANDBOX_DEFAULT_WALL_TIME=30      # 30 seconds
export ISOLATE_SANDBOX_DEFAULT_EXTRA_TIME=5      # 5 seconds
export ISOLATE_SANDBOX_DEFAULT_STACK=64000       # 64KB
export ISOLATE_SANDBOX_DEFAULT_FSIZE=51200       # 50MB
export ISOLATE_SANDBOX_DEFAULT_OPEN_FILES=32     # 32 files
export ISOLATE_SANDBOX_DEFAULT_PROCESSES=0       # Unlimited processes (0 = --processes without value)

echo "Environment variables set:"
echo "- Memory limit: 256MB"
echo "- Time limit: 15 seconds"
echo "- File size limit: 50MB"
echo "- Open files limit: 32"
echo "- Processes limit: unlimited"
echo ""
echo "You can now run the service with: orbctl run sudo RUST_LOG=info ./target/debug/isolate-sandbox"
echo ""
echo "To set any limit to unlimited, set it to 0 (except processes which uses 0 for unlimited with --processes flag)"