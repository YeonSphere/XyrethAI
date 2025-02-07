#!/usr/bin/env zsh

# Setup script for Seokjin-AI
# Created by: daedaevibin
# Date: 2025-02-07 21:47:12 UTC

set -e # Exit on any error

# Colors for pretty output
typeset -A colors
colors=(
    "info" $'\e[1;34m'    # Blue
    "success" $'\e[1;32m'  # Green
    "warning" $'\e[1;33m'  # Yellow
    "error" $'\e[1;31m'    # Red
    "reset" $'\e[0m'
)

# Function to print colored messages
print_message() {
    local type=$1
    local message=$2
    echo "${colors[$type]}$message${colors[reset]}"
}

# Create project structure
create_structure() {
    print_message "info" "Creating project structure..."
    mkdir -p src/{core,memory,neural,utils} tests docs examples scripts build
    if [[ $? -ne 0 ]]; then
        print_message "error" "Failed to create project directories."
        exit 1
    fi
    
    touch src/core/lib.zig src/main.zig build.zig
    if [[ $? -ne 0 ]]; then
        print_message "error" "Failed to create necessary files."
        exit 1
    fi
}

# Create .gitignore
create_gitignore() {
    print_message "info" "Creating .gitignore..."
    cat > .gitignore << 'EOL'
Zig
zig-cache/
zig-out/
build/
release/
debug/
# Editor files
.vscode/
.idea/
*.swp
*.swo
# OS files
.DS_Store
Thumbs.db
EOL
}

# Create initial documentation
create_docs() {
    print_message "info" "Creating initial documentation..."
    cat > docs/ARCHITECTURE.md << 'EOL'
Seokjin AI - Architecture
Overview

Seokjin AI is designed with the following principles:

    High performance
    Low resource usage
    Maximum flexibility
    Direct hardware access when needed

Core Components

    Memory Management
        Custom allocator
        Memory pool
        Zero-copy operations

    Neural Processing
        Custom neural network implementation
        Hardware acceleration support
        Dynamic topology

    Core AI
        State management
        Resource coordination
        System integration

Performance Considerations

    No garbage collection
    Direct memory management
    Minimal runtime overhead
    Hardware-specific optimizations
EOL
}

# Create main build.zig
create_build_zig() {
    print_message "info" "Creating build.zig..."
    cat > build.zig << 'EOL'
const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    // Main executable
    const exe = b.addExecutable(.{
        .name = "seokjin-ai",
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Core AI library
    const lib = b.addStaticLibrary(.{
        .name = "seokjin-core",
        .root_source_file = .{ .path = "src/core/lib.zig" },
        .target = target,
        .optimize = optimize,
    });

    // Install artifacts
    b.installArtifact(exe);
    b.installArtifact(lib);

    // Add tests
    const main_tests = b.addTest(.{
        .root_source_file = .{ .path = "src/main.zig" },
        .target = target,
        .optimize = optimize,
    });

    const test_step = b.step("test", "Run all tests");
    test_step.dependOn(&main_tests.step);
}
EOL
}

# Create main.zig
create_main_zig() {
    print_message "info" "Creating main.zig..."
    cat > src/main.zig << 'EOL'
const std = @import("std");
const core = @import("core/lib.zig");

pub fn main() !void {
    std.debug.print("Seokjin AI - Initializing...\n", .{});
    
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var ai = try core.AI.init(allocator);
    defer ai.deinit();

    ai.start() catch {
        std.debug.print("Failed to start AI.\n", .{});
        return error.StartFailed;
    }
}

test "basic test" {
    try std.testing.expect(true);
}
EOL
}

# Create core library
create_core_lib() {
    print_message "info" "Creating core library..."
    cat > src/core/lib.zig << 'EOL'
const std = @import("std");

pub const AI = struct {
    allocator: std.mem.Allocator,
    state: State,

    const Self = @This();

    pub const State = enum {
        initialized,
        running,
        stopped,
    };

    pub fn init(allocator: std.mem.Allocator) !*Self {
        var self = try allocator.create(Self);
        self.* = .{
            .allocator = allocator,
            .state = .initialized,
        };
        return self;
    }

    pub fn deinit(self: *Self) void {
        self.allocator.destroy(self);
    }

    pub fn start(self: *Self) !void {
        self.state = .running;
        std.debug.print("AI Core started\n", .{});
    }
};
EOL
}

# Main setup function
main() {
    print_message "info" "Starting Seokjin AI project setup..."

    # Create project structure
    create_structure

    # Create core files
    create_build_zig
    create_main_zig
    create_core_lib
    create_gitignore
    create_docs

    # Initialize git if not already initialized
    if [[ ! -d .git ]]; then
        print_message "info" "Initializing git repository..."
        git init
        git add .
        git commit -m "Initial commit: Project setup"
    fi

    print_message "success" "Setup completed successfully!"
    print_message "info" "To get started, run:"
    echo "  zig build"
    echo "  zig build test"
}

# Run the setup
main
