mod lexer {
    struct Lexer {
        input: str,
        position: i32,
        read_position: i32,
        ch: u8,
    }

    impl Lexer {
        pub fn new(input: str) -> Self {
            let mut l = Self {
                input: input,
                position: 0,
                read_position: 0,
                ch: 0,
            };
        }
        pub fn readChar(&mut self) {
            if self.read_position >= self.input.len() {
                self.ch = 0;
            } else {
                self.ch = self.input[self.read_position];
            }
            self.postion = self.read_position;
            self.read_position += 1;
        }
    }
}
