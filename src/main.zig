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
