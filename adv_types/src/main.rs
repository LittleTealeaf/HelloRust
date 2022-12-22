use core::fmt;

fn main() {
    {
        type Kilometeres = i32;

        let x: i32 = 5;
        let y: Kilometeres = 5;

        println!("x + y = {}", x + y);
    }
    {
        type Result<T> = std::result::Result<T, std::io::Error>;

        pub trait Write {
            fn write(&mut self, buf: &[u8]) -> Result<usize>;
            fn flush(&mut self) -> Result<()>;
            fn write_all(&mut self, buf: &[u8]) -> Result<()>;
            fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
        }
    }
}
