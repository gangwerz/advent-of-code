const std = @import("std");

pub fn main() !void {
    var file = @embedFile("./inventory.txt");

    var elves: [8]u64 = undefined;
    var snackpack: [8]u8 = undefined;
    var count: u64 = 0;
    var cursor: u64 = 0;

    for (file) |e| {
        if (e == '\n') {
            if (snackpack.len != 0) {
                const buffer = snackpack[0 .. cursor + 1];
                elves[count] = try std.fmt.parseUnsigned(u64, buffer, 10);
                std.debug.print("Item {}: {}\n", .{ count, elves[count] });
                snackpack = undefined;
                cursor = 0;
            } else {
                count += 1;
            }
        } else {
            snackpack[cursor] = e;
            cursor += 1;
        }
    }
}
