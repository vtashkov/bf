use std::{
    num::Wrapping,
    ops::{AddAssign, SubAssign},
};

pub struct Memory<T> {
    cells: Vec<Wrapping<T>>,
    current_idx: usize,
}

impl<T> Memory<T>
where
    T: Default + Clone,
    Wrapping<T>: AddAssign<Wrapping<u8>> + SubAssign<Wrapping<u8>>,
{
    const ONE: Wrapping<u8> = Wrapping(1);

    pub fn new(size: usize) -> Memory<T> {
        Memory {
            cells: vec![Wrapping(T::default()); size],
            current_idx: 0,
        }
    }

    pub fn clear(&mut self) {
        for item in &mut self.cells { *item = Wrapping(T::default()); }
        self.current_idx = 0;
    }

    pub fn read(&self) -> &T {
        &self.cells[self.current_idx].0
    }

    pub fn write(&mut self, value: T) {
        self.cells[self.current_idx] = Wrapping(value);
    }

    pub fn next(&mut self) {
        self.current_idx += 1;
        if self.current_idx == self.cells.len() {
            self.current_idx = 0;
        }
    }

    pub fn previous(&mut self) {
        if self.current_idx == 0 {
            self.current_idx = self.cells.len();
        }
        self.current_idx -= 1;
    }

    pub fn increment(&mut self) {
        self.cells[self.current_idx] += Self::ONE;
    }

    pub fn decrement(&mut self) {
        self.cells[self.current_idx] -= Self::ONE;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_can_be_created() {
        let _memory: Memory<u8> = Memory::new(1);
    }

    #[test]
    fn memory_cell_can_be_accessed() {
        let memory: Memory<u8> = Memory::new(1);
        let _cell_value = memory.read();
    }

    #[test]
    fn memory_cell_is_initialized_to_zero() {
        let memory: Memory<u8> = Memory::new(1);
        let cell_value = memory.read();
        assert_eq!(0, *cell_value);
    }

    #[test]
    fn memory_cell_can_be_written_to() {
        let mut memory: Memory<u8> = Memory::new(1);
        memory.write(1);
    }

    #[test]
    fn memory_cell_preserves_the_written_value() {
        let mut memory: Memory<u8> = Memory::new(1);
        let cell_value = memory.read();
        assert_eq!(0, *cell_value);
        memory.write(1);
        let cell_value = memory.read();
        assert_eq!(1, *cell_value);
    }

    #[test]
    fn memory_can_move_to_next_cell() {
        let mut memory: Memory<u8> = Memory::new(2);
        memory.next();
    }

    #[test]
    fn memory_cells_are_initialized_to_zero() {
        let mut memory: Memory<u8> = Memory::new(2);
        let cell_value = memory.read();
        assert_eq!(0, *cell_value);
        memory.next();
        let cell_value = memory.read();
        assert_eq!(0, *cell_value);
    }

    #[test]
    fn memory_cells_preserve_value_after_next() {
        let mut memory: Memory<u8> = Memory::new(2);
        let cell_value = memory.read();
        assert_eq!(0, *cell_value);
        memory.write(1);
        let cell_value = memory.read();
        assert_eq!(1, *cell_value);
        memory.next();
        let cell_value = memory.read();
        assert_eq!(0, *cell_value);
        memory.write(1);
        let cell_value = memory.read();
        assert_eq!(1, *cell_value);
    }

    #[test]
    fn memory_can_move_to_previous_cell() {
        let mut memory: Memory<u8> = Memory::new(2);
        memory.next();
        memory.previous();
    }

    #[test]
    fn memory_cells_preserve_value_after_previous() {
        let mut memory: Memory<u8> = Memory::new(2);
        let cell_value = memory.read();
        assert_eq!(0, *cell_value);
        memory.write(1);
        let cell_value = memory.read();
        assert_eq!(1, *cell_value);
        memory.next();
        let cell_value = memory.read();
        assert_eq!(0, *cell_value);
        memory.write(2);
        let cell_value = memory.read();
        assert_eq!(2, *cell_value);
        memory.previous();
        let cell_value = memory.read();
        assert_eq!(1, *cell_value);
    }

    #[test]
    fn memory_cells_go_from_the_begining_after_next_on_the_last_cell() {
        let mut memory: Memory<u8> = Memory::new(3);
        memory.next();
        memory.write(1);
        memory.next();
        memory.write(2);
        memory.next();
        let cell_value = memory.read();
        assert_eq!(0, *cell_value);
        memory.next();
        let cell_value = memory.read();
        assert_eq!(1, *cell_value);
        memory.next();
        let cell_value = memory.read();
        assert_eq!(2, *cell_value);
    }

    #[test]
    fn memory_cells_go_to_the_end_after_previous_on_the_first_cell() {
        let mut memory: Memory<u8> = Memory::new(3);
        memory.next();
        memory.write(1);
        memory.next();
        memory.write(2);
        memory.next();
        let cell_value = memory.read();
        assert_eq!(0, *cell_value);
        memory.previous();
        let cell_value = memory.read();
        assert_eq!(2, *cell_value);
        memory.previous();
        let cell_value = memory.read();
        assert_eq!(1, *cell_value);
        memory.previous();
        let cell_value = memory.read();
        assert_eq!(0, *cell_value);
    }

    #[test]
    fn memory_cell_can_be_incremented() {
        let mut memory: Memory<u8> = Memory::new(1);
        memory.increment();
    }

    #[test]
    fn memory_cell_is_one_after_incrementing_the_initial_value() {
        let mut memory: Memory<u8> = Memory::new(1);
        memory.increment();
        let cell_value = memory.read();
        assert_eq!(1, *cell_value);
    }

    #[test]
    fn memory_cell_is_two_after_incrementing_twice_the_initial_value() {
        let mut memory: Memory<u8> = Memory::new(1);
        memory.increment();
        memory.increment();
        let cell_value = memory.read();
        assert_eq!(2, *cell_value);
    }

    #[test]
    fn memory_cell_is_wrapping_after_incrementing_the_max_value() {
        let mut memory: Memory<u8> = Memory::new(1);
        memory.write(u8::max_value());
        memory.increment();
        let cell_value = memory.read();
        assert_eq!(0, *cell_value);
    }

    #[test]
    fn memory_cell_can_be_decremented() {
        let mut memory: Memory<u8> = Memory::new(1);
        memory.write(1);
        memory.decrement();
    }

    #[test]
    fn memory_cell_is_one_after_decrementing_from_two() {
        let mut memory: Memory<u8> = Memory::new(1);
        memory.write(2);
        memory.decrement();
        let cell_value = memory.read();
        assert_eq!(1, *cell_value);
    }

    #[test]
    fn memory_cell_is_zero_after_decrementing_twice_from_two() {
        let mut memory: Memory<u8> = Memory::new(1);
        memory.write(2);
        memory.decrement();
        memory.decrement();
        let cell_value = memory.read();
        assert_eq!(0, *cell_value);
    }

    #[test]
    fn memory_cell_is_wrapping_after_decrementing_zero() {
        let mut memory: Memory<u8> = Memory::new(1);
        let cell_value = memory.read();
        assert_eq!(0, *cell_value);
        memory.decrement();
        let cell_value = memory.read();
        assert_eq!(u8::max_value(), *cell_value);
    }

    #[test]
    fn memory_can_be_cleared() {
        let mut memory: Memory<u8> = Memory::new(3);
        memory.next();
        memory.write(1);
        memory.next();
        memory.write(2);
        memory.clear();
        let cell_value = memory.read();
        assert_eq!(0, *cell_value);
        memory.next();
        let cell_value = memory.read();
        assert_eq!(0, *cell_value);
        memory.next();
        let cell_value = memory.read();
        assert_eq!(0, *cell_value);
    }
}
