//！ traits implementations for primitive types and the types defined in std lib

use super::*;
use std::cell::Cell;
use std::rc::Rc;

impl Generate for u8 {
    fn generate(gen: &mut StdRng) -> u8 {
        gen.gen_range(0..127) as u8
    }
}

impl Generate for c_int {
    fn generate(gen: &mut StdRng) -> i32 {
        gen.gen::<c_int>()
    }
}

impl Generate for c_uint {
    fn generate(gen: &mut StdRng) -> u32 {
        gen.gen::<c_uint>()
    }
}

impl Argument for c_uint {
    fn argumentize(&self) -> usize {
        *self as _
    }
}

impl Argument for c_int {
    fn argumentize(&self) -> usize {
        *self as _
    }
}

impl Clean for c_int {
    fn clean(self, _res: std::result::Result<i64, i64>) {
        ()
    }
}

impl Generate for usize {
    fn generate(gen: &mut StdRng) -> usize {
        gen.gen::<usize>()
    }
}

impl Argument for usize {
    fn argumentize(&self) -> usize {
        *self
    }
}

// traits implementations for Box

impl<T> Generate for Box<T>
where
    T: Generate,
{
    fn generate(gen: &mut StdRng) -> Box<T> {
        Box::new(T::generate(gen))
    }
}

impl<T> Argument for Box<T> {
    fn argumentize(&self) -> usize {
        self as *const _ as usize
    }
}

impl<T> Generate for Cell<T>
where
    T: Generate,
{
    fn generate(gen: &mut StdRng) -> Cell<T> {
        Cell::new(T::generate(gen))
    }
}

impl<T> Argument for Cell<T> {
    fn argumentize(&self) -> usize {
        self.as_ptr() as usize
    }
}

impl<T> Generate for Rc<T>
where
    T: Generate,
{
    fn generate(gen: &mut StdRng) -> Rc<T> {
        Rc::new(T::generate(gen))
    }
}

impl<T> Argument for Rc<T> {
    fn argumentize(&self) -> usize {
        Rc::as_ptr(self) as usize
    }
}
