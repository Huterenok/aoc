const std = @import("std");
const fs = std.fs;
pub var allocator = std.heap.page_allocator;

const CmpType = enum {
    Bigger,
    Less,
    AltBigger,
    AltLess,

    pub fn convert(el: u8) CmpType {
        return switch (el) {
            '>' => CmpType.Bigger,
            '<' => CmpType.Less,
            else => unreachable,
        };
    }
};

const Workflow = struct {
    arg: ?u8 = null,
    cmpTo: ?usize = null,
    cmpType: ?CmpType = null,
    to: []const u8,
};

const Comparison = struct { arg: u8, cmpType: CmpType, cmpTo: usize };

const Input = struct {
    x: usize,
    m: usize,
    a: usize,
    s: usize,

    pub fn get(self: Input, elem: u8) usize {
        return switch (elem) {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            else => unreachable,
        };
    }
};

const SystemData = struct {
    inputs: []Input,
    workflows: std.StringHashMap([]Workflow),

    pub fn init(input: []const u8) !SystemData {
        var inputs = std.ArrayList(Input).init(allocator);
        var map = std.StringHashMap([]Workflow).init(allocator);

        var records = std.mem.tokenizeSequence(u8, input, "\n\n");

        const workflow_part = records.next().?;
        const input_part = records.next().?;
        var workflow_records = std.mem.tokenizeScalar(u8, workflow_part, '\n');
        var input_records = std.mem.tokenizeScalar(u8, input_part, '\n');

        while (workflow_records.next()) |line| {
            var splitted = std.mem.tokenizeScalar(u8, line, '{');
            const path = splitted.next().?;
            const workflows_unparsed = splitted.next().?;
            var workflows = std.mem.tokenizeScalar(u8, workflows_unparsed[0 .. workflows_unparsed.len - 1], ',');

            var workflows_finalized = std.ArrayList(Workflow).init(allocator);
            while (workflows.next()) |workflow| {
                if (std.mem.count(u8, workflow, ":") > 0) {
                    var splitted_workflow = std.mem.tokenizeScalar(u8, workflow, ':');

                    const left = splitted_workflow.next().?;

                    const right = splitted_workflow.next().?;
                    const our_element = left[0];
                    const cmp = CmpType.convert(left[1]);
                    try workflows_finalized.append(Workflow{ .to = right, .arg = our_element, .cmpTo = try std.fmt.parseInt(usize, left[2..], 10), .cmpType = cmp });
                } else {
                    try workflows_finalized.append(Workflow{ .to = workflow });
                }
            }

            try map.put(path, try workflows_finalized.toOwnedSlice());
        }

        while (input_records.next()) |line| {
            var splitted = std.mem.tokenizeScalar(u8, line, ',');
            const x = splitted.next().?;
            const m = splitted.next().?;
            const a = splitted.next().?;
            const s = splitted.next().?;

            try inputs.append(Input{
                .x = try std.fmt.parseInt(usize, x[3..], 10),
                .m = try std.fmt.parseInt(usize, m[2..], 10),
                .a = try std.fmt.parseInt(usize, a[2..], 10),
                .s = try std.fmt.parseInt(usize, s[2 .. s.len - 1], 10),
            });
        }

        return SystemData{
            .inputs = try inputs.toOwnedSlice(),
            .workflows = map,
        };
    }
};

pub fn sendDatShitToWorkflows(input: []const u8) !usize {
    const sys = try SystemData.init(input);
    var res: usize = 0;

    for (sys.inputs) |in| {
        var curr = sys.workflows.get("in").?;

        out: while (true) {
            for (curr) |workflow| {
                if (workflow.arg != null) {
                    if (workflow.cmpType.? == CmpType.Less) {
                        if (in.get(workflow.arg.?) >= workflow.cmpTo.?) {
                            continue;
                        }
                    } else {
                        if (in.get(workflow.arg.?) <= workflow.cmpTo.?) {
                            continue;
                        }
                    }
                }

                if (workflow.to[0] == 'R') break :out;
                if (workflow.to[0] == 'A') {
                    res += in.x + in.m + in.a + in.s;
                    break :out;
                }

                curr = sys.workflows.get(workflow.to).?;
                continue :out;
            }
            unreachable;
        }
    }

    return res;
}

pub fn ecmps_listectivlySendDatShitToWorkflows(input: []const u8) !usize {
    var final = std.ArrayList([]Comparison).init(allocator);
    const comparisons = std.ArrayList(Comparison).init(allocator);
    const sys = try SystemData.init(input);

    var stack = std.ArrayList(struct { []Workflow, std.ArrayList(Comparison) }).init(allocator);
    try stack.append(.{ sys.workflows.get("in").?, comparisons });

    out: while (stack.items.len != 0) {
        const t = stack.pop();
        const workflows = t[0];
        var comparisons_exact = t[1];
        for (workflows) |workflow| {
            if (workflow.arg != null) {
                if (workflow.to[0] != 'R') {
                    var temp = try comparisons_exact.clone();
                    try temp.append(Comparison{
                        .arg = workflow.arg.?,
                        .cmpTo = workflow.cmpTo.?,
                        .cmpType = workflow.cmpType.?,
                    });
                    if (workflow.to[0] == 'A') {
                        try final.append(try temp.toOwnedSlice());
                    } else {
                        try stack.append(.{ sys.workflows.get(workflow.to).?, temp });
                    }
                }
                const cmp = if (workflow.cmpType.? == CmpType.Less) CmpType.AltBigger else CmpType.AltLess;
                try comparisons_exact.append(.{
                    .arg = workflow.arg.?,
                    .cmpTo = workflow.cmpTo.?,
                    .cmpType = cmp,
                });
            } else {
                if (workflow.to[0] == 'R') continue :out;
                if (workflow.to[0] == 'A') {
                    try final.append(try comparisons_exact.toOwnedSlice());
                    continue :out;
                }

                try stack.append(.{ sys.workflows.get(workflow.to).?, comparisons_exact });
            }
        }
    }

    const cmps_list = try final.toOwnedSlice();
    var res: usize = 0;
    for (cmps_list) |f| {
        var xmin: usize = 1;
        var xmax: usize = 4000;
        var mmin: usize = 1;
        var mmax: usize = 4000;
        var amin: usize = 1;
        var amax: usize = 4000;
        var smin: usize = 1;
        var smax: usize = 4000;
        for (f) |c| {
            switch (c.arg) {
                'x' => {
                    if (c.cmpType == CmpType.Less) {
                        xmax = c.cmpTo - 1;
                    } else if (c.cmpType == CmpType.Bigger) {
                        xmin = c.cmpTo + 1;
                    } else if (c.cmpType == CmpType.AltBigger) {
                        xmin = c.cmpTo;
                    } else if (c.cmpType == CmpType.AltLess) {
                        xmax = c.cmpTo;
                    } else unreachable;
                },
                'm' => {
                    if (c.cmpType == CmpType.Less) {
                        mmax = c.cmpTo - 1;
                    } else if (c.cmpType == CmpType.Bigger) {
                        mmin = c.cmpTo + 1;
                    } else if (c.cmpType == CmpType.AltLess) {
                        mmax = c.cmpTo;
                    } else if (c.cmpType == CmpType.AltBigger) {
                        mmin = c.cmpTo;
                    } else unreachable;
                },
                'a' => {
                    if (c.cmpType == CmpType.Less) {
                        amax = c.cmpTo - 1;
                    } else if (c.cmpType == CmpType.Bigger) {
                        amin = c.cmpTo + 1;
                    } else if (c.cmpType == CmpType.AltLess) {
                        amax = c.cmpTo;
                    } else if (c.cmpType == CmpType.AltBigger) {
                        amin = c.cmpTo;
                    } else unreachable;
                },
                's' => {
                    if (c.cmpType == CmpType.Less) {
                        smax = c.cmpTo - 1;
                    } else if (c.cmpType == CmpType.Bigger) {
                        smin = c.cmpTo + 1;
                    } else if (c.cmpType == CmpType.AltLess) {
                        smax = c.cmpTo;
                    } else if (c.cmpType == CmpType.AltBigger) {
                        smin = c.cmpTo;
                    } else unreachable;
                },
                else => unreachable,
            }
        }
        res += (xmax - xmin + 1) * (mmax - mmin + 1) * (amax - amin + 1) * (smax - smin + 1);
    }

    return res;
}

const real_input = @embedFile("./input.txt");
const example_input = @embedFile("./example_input.txt");

test "Result 1" {
    try std.testing.expectEqual(sendDatShitToWorkflows(example_input), 19114);
    try std.testing.expectEqual(sendDatShitToWorkflows(real_input), 346230);
}

test "Result 2" {
    try std.testing.expectEqual(ecmps_listectivlySendDatShitToWorkflows(example_input), 167409079868000);
    try std.testing.expectEqual(ecmps_listectivlySendDatShitToWorkflows(real_input), 124693661917133);
}
