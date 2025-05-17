pub struct LzCoder {
    vector: Vec<char>,
    index: usize,
    distance: u32,
}


impl LzCoder {
    
    pub fn new() -> LzCoder {
        LzCoder {
            vector: Vec::new(),
            index: 0,
            distance: 0,
        }
    }
}