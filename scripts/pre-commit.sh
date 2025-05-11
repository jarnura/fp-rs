#!/bin/sh
#
# Pre-commit hook to check formatting, lint, and run tests.
# This script is intended to be run from the root of the repository.

echo "Running pre-commit checks..."

# 1. Check formatting
echo "Checking formatting (cargo fmt -- --check)..."
cargo fmt -- --check
if [ $? -ne 0 ]; then
    echo "Formatting check failed. Please run 'cargo fmt' to fix."
    exit 1
fi
echo "Formatting check passed."
echo ""

# 2. Run clippy for lints
echo "Running clippy (cargo clippy --all-features -- -D warnings)..."
cargo clippy --all-features -- -D warnings
if [ $? -ne 0 ]; then
    echo "Clippy check failed. Please fix the lints."
    exit 1
fi
echo "Clippy check passed."
echo ""

# 3. Run tests
echo "Running tests (cargo test --all-features)..."
cargo test --all-features
if [ $? -ne 0 ]; then
    echo "Tests failed. Please fix the tests."
    exit 1
fi
echo "Tests passed."
echo ""

echo "All pre-commit checks passed. Proceeding with commit."
exit 0
