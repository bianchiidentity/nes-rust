pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }

    // register_aを変更するので、mutにする
    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opscode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opscode {
                0x00 => return,

                // LDA, Load Accumulator https://www.nesdev.org/obelisk-6502-guide/reference.html#LDA
                // アキュムレータに 1 バイトのメモリをロードし、ゼロと負のフラグを適切に設定します。
                0xA9 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;
                    self.lda(param);
                }
                0xE8 => {
                    self.register_x = self.register_x.wrapping_add(1);
                    self.update_zero_and_negative_flags(self.register_x);
                }

                0xAA => self.tax(),
                _ => todo!(),
            }
        }
    }

    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status = self.status | 0b0000_0010; // Zフラグを立てる
        } else {
            self.status = self.status & 0b1111_1101; // Zフラグを下ろす
        }

        if result & 0b1000_0000 != 0 {
            // 最上位ビットが1なら
            self.status = self.status | 0b1000_0000; // Nフラグを立てる
        } else {
            self.status = self.status & 0b0111_1111; // Nフラグを下ろす
        }
    }
}
