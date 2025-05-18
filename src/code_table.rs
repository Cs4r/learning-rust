struct Range {
    left: usize,
    right: usize,
}

struct Row {
    code: i32,
    n_bits: usize,
    range: Range,
}