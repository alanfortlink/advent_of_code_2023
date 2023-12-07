const std = @import("std");

pub fn startsWith(line: []const u8, i: usize, needle: []const u8) bool {
    for (0..needle.len) |j| {
        if (j + i >= line.len) return false;
        if (line[j + i] != needle[j]) return false;
    }
    return true;
}

pub fn isDigit(line: []const u8, i: usize) ?(i32) {
    if (startsWith(line, i, "one")) return 1;
    if (startsWith(line, i, "two")) return 2;
    if (startsWith(line, i, "three")) return 3;
    if (startsWith(line, i, "four")) return 4;
    if (startsWith(line, i, "five")) return 5;
    if (startsWith(line, i, "six")) return 6;
    if (startsWith(line, i, "seven")) return 7;
    if (startsWith(line, i, "eight")) return 8;
    if (startsWith(line, i, "nine")) return 9;

    var c = line[i];
    return switch (c) {
        '0'...'9' => c - '0',
        else => null,
    };
}

pub fn get(line: []const u8, s: i32, e: i32, inc: i32) i32 {
    var i: i32 = s;
    while (i != e) {
        var digit = isDigit(line, @intCast(i));
        if (digit) |d| {
            return d;
        }
        i = i + inc;
    }

    return 0;
}

pub fn get_calibration(line: []const u8) i32 {
    var D = get(line, 0, @intCast(line.len), 1);
    var U = get(line, @intCast(line.len - 1), -1, -1);

    return 10 * D + U;
}

pub fn main() !void {
    var args = std.process.args();
    var exe = args.next();

    var path = args.next();
    if (path == null) {
        std.log.err("Usage: ./{s} file_path", .{exe.?});
        std.process.exit(1);
    }

    var file = try std.fs.cwd().openFile(path.?, std.fs.File.OpenFlags{});
    defer file.close();

    var buf_reader = std.io.bufferedReader(file.reader());
    var input_stream = buf_reader.reader();
    var buf: [1024]u8 = undefined;

    var total: i32 = 0;

    while (try input_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
        var lt = get_calibration(line);
        total += lt;
    }

    std.log.info("{d}", .{total});
}
