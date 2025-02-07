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
