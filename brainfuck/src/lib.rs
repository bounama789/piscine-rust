pub struct Interpreter {
    source: Vec<char>,
    pointer: u8,
    bytes: Vec<u8>,
}

impl Interpreter {
    fn new(s: &str) -> Self {
        Self {
            source: s.chars().collect(),
            pointer: 0,
            bytes: [0; 256].to_vec(),
        }
    }

    fn current_byte(&self) -> u8 {
        self.bytes[self.pointer as usize]
    }

    fn next_byte(&mut self) {
        self.pointer = self.pointer.wrapping_add(1);
    }

    fn prev_byte(&mut self) {
        self.pointer = self.pointer.wrapping_sub(1);
    }

    fn get_char_at(&self) -> char {
        char::from_u32(self.current_byte() as u32).unwrap()
    }

    fn increase_byte(&mut self) {
        let b = self.bytes.get_mut(self.pointer as usize).unwrap();
        *b = b.wrapping_add(1);
    }

    fn decrease_byte(&mut self) {
        let b = self.bytes.get_mut(self.pointer as usize).unwrap();
        *b = b.wrapping_sub(1);
    }

    fn exec(&mut self, c: &char) {
        match c {
            '<' => self.prev_byte(),
            '>' => self.next_byte(),
            '+' => self.increase_byte(),
            '-' => self.decrease_byte(),
            '.' => print!("{}", self.get_char_at()),
            _ => {}
        }
    }

    fn get_loop(&self, idx: usize) -> Vec<char> {
        let mut occurence = 1;
        let mut i = idx + 1;

        let s = self.source.clone();
        while occurence > 0 {
            if s[i] == '[' {
                occurence += 1;
            }
            if s[i] == ']' {
                occurence -= 1;
            }

            i += 1
        }

        s.get(idx..i).unwrap().to_vec()
    }

    fn exec_loop(&mut self, s: Vec<char>) {
        while self.current_byte() > 1 {
            for (i, c) in s.iter().enumerate() {
                if i != 0 {
                    match c {
                        '[' => {
                            let l = self.get_loop(i);
                            self.exec_loop(l)
                        }
                        _ => self.exec(c),
                    }
                }
            }
        }
    }
}

pub fn brain_fuck(s: &str) {
    let mut interpreter = Interpreter::new(s);
    let source = interpreter.source.clone();
    for (i, c) in source.iter().enumerate() {
        match c {
            '[' => {
                if interpreter.current_byte() > 0 {
                    let l = interpreter.get_loop(i);
                    interpreter.exec_loop(l)
                }
            }
            _ => interpreter.exec(c),
        }
    }
}
