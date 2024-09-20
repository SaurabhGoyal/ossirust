use volatile::Volatile;

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
    buffer: [[Volatile<Char>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Writer {
    pub(crate) fn new() -> Self {
        Self {
            i: 0,
            j: 0,
            buffer: core::array::from_fn(|_i| {
                core::array::from_fn(|_i| Volatile::new(Char::new(0, 0)))
            }),
        }
    }

    pub(crate) fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => {
                self.i += 1;
                self.j = 0;
            }
            _ => {
                self.buffer[self.i][self.j].write(Char::new(byte, 0xc));
                self.j += 1;
                if self.j == BUFFER_WIDTH {
                    self.i += 1;
                    self.j = 0;
                }
            }
        }
        if self.i == BUFFER_HEIGHT {
            self.shift_buffer_up();
            self.i -= 1;
        }
        self.write_buffer();
    }

    fn shift_buffer_up(&mut self) {
        for i in 0..(BUFFER_HEIGHT - 1) {
            self.buffer[i] = self.buffer[i + 1].clone();
        }
        self.buffer[BUFFER_HEIGHT - 1] = core::array::from_fn(|_i| Volatile::new(Char::new(0, 0)));
    }

    fn reset_buffer(&mut self) {
        self.i = 0;
        self.j = 0;
        self.buffer =
            core::array::from_fn(|_i| core::array::from_fn(|_i| Volatile::new(Char::new(0, 0))))
    }

    fn write_buffer(&self) {
        for i in 0..BUFFER_HEIGHT {
            for j in 0..BUFFER_WIDTH {
                let index = i * BUFFER_WIDTH + j;
                unsafe {
                    *VIDEO_MEMORY_ADDRESS_TEXT_MODE.offset(index as isize * 2) =
                        self.buffer[i][j].read().code; // Chraracter byte
                    *VIDEO_MEMORY_ADDRESS_TEXT_MODE.offset(index as isize * 2 + 1) =
                        self.buffer[i][j].read().colour; // Chraracter byte
                }
            }
        }
    }
}
