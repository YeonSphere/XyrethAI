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
