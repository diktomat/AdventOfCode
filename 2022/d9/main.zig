const std = @import("std");

const Point = struct {
	x: i32,
	y: i32,
};

pub fn main() !void {
	var file = try std.fs.cwd().openFile("input.txt", .{});
	defer file.close();

	const parts = try rope(file, 2, 10);
	std.debug.print("{}\n", .{parts.p1});
	std.debug.print("{}\n", .{parts.p2});
}

fn rope(file: std.fs.File, n1: u32, n2: u32) !struct { p1: u32, p2: u32 } {
	var gpa = std.heap.GeneralPurposeAllocator(.{}){};
	const allocator = gpa.allocator();
	var buf_reader = std.io.bufferedReader(file.reader());
	var in_stream = buf_reader.reader();
	var buf: [10]u8 = undefined;

	var knots = init: {
		var initial_value: [10]Point = undefined;
		for (initial_value) |*pt| {
			pt.* = Point{
				.x = 0,
				.y = 0,
			};
		}
		break :init initial_value;
	};

	var visited1 = std.AutoHashMap(Point, void).init(allocator);
	try visited1.put(knots[0], {});
	var visited2 = std.AutoHashMap(Point, void).init(allocator);
	try visited2.put(knots[0], {});

	while (try in_stream.readUntilDelimiterOrEof(&buf, '\n')) |line| {
		var len = try std.fmt.parseInt(i32, line[2..], 10);
		const move = switch (line[0]) {
			'R' => Point{ .x = 1, .y = 0 },
			'L' => Point{ .x = -1, .y = 0 },
			'U' => Point{ .x = 0, .y = 1 },
			'D' => Point{ .x = 0, .y = -1 },
			else => Point{ .x = 0, .y = 0 },
		};

		while (len > 0) {
			knots[0].x += move.x;
			knots[0].y += move.y;

			for (knots) |*cur, i| {
				if (i == 0) continue;

				if ((abs(knots[i - 1].x - cur.x) == 2) and (abs(knots[i - 1].y - cur.y) == 2)) { // only in part 2
					cur.x += @divTrunc(knots[i - 1].x - cur.x, 2);
					cur.y += @divTrunc(knots[i - 1].y - cur.y, 2);
				} else if (abs(knots[i - 1].x - cur.x) == 2) {
					cur.x += @divTrunc(knots[i - 1].x - cur.x, 2);
					cur.y += knots[i - 1].y - cur.y;
				} else if (abs(knots[i - 1].y - cur.y) == 2) {
					cur.y += @divTrunc(knots[i - 1].y - cur.y, 2);
					cur.x += knots[i - 1].x - cur.x;
				}
			}

			try visited1.put(knots[n1 - 1], {});
			try visited2.put(knots[n2 - 1], {});
			len -= 1;
		}
	}

	return .{ .p1 = visited1.count(), .p2 = visited2.count() };
}

fn abs(num: i32) i32 {
	return if (num < 0) -num else num;
}
