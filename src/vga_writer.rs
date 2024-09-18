const VIDEO_MEMORY_ADDRESS_TEXT_MODE: *mut u8 = 0xb8000 as *mut u8;
const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[derive(Clone, Copy)]
struct Char {
    code: u8,
    colour: u8,
}

impl Char {
    fn new(code: u8, colour: u8) -> Self {
        Self { code, colour }
    }
}

pub(crate) struct Writer {
    i: usize,
    j: usize,
    buffer: [[Char; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Writer {
    pub(crate) fn new() -> Self {
        Self {
            i: 0,
            j: 0,
            buffer: [[Char::new(0, 0); BUFFER_WIDTH]; BUFFER_HEIGHT],
        }
    }

    pub(crate) fn write_byte(&mut self, byte: u8) {
        if self.i == BUFFER_HEIGHT - 1 && self.j == BUFFER_WIDTH {
            self.clear_buffer();
        }
        match byte {
            b'\n' => {
                self.i += 1;
                self.j = 0;
            }
            _ => {
                self.buffer[self.i][self.j] = Char::new(byte, 0xc);
                self.j += 1;
                if self.j == BUFFER_WIDTH {
                    self.j = 0;
                    self.i += 1;
                }
            }
        }
        self.write_buffer();
    }

    fn clear_buffer(&mut self) {
        self.i = 0;
        self.j = 0;
        self.buffer = [[Char::new(0, 0); BUFFER_WIDTH]; BUFFER_HEIGHT];
    }

    fn write_buffer(&self) {
        for i in 0..BUFFER_HEIGHT {
            for j in 0..BUFFER_WIDTH {
                let index = i * BUFFER_WIDTH + j;
                unsafe {
                    *VIDEO_MEMORY_ADDRESS_TEXT_MODE.offset(index as isize * 2) =
                        self.buffer[i][j].code; // Chraracter byte
                    *VIDEO_MEMORY_ADDRESS_TEXT_MODE.offset(index as isize * 2 + 1) =
                        self.buffer[i][j].colour; // Chraracter byte
                }
            }
        }
    }
}
