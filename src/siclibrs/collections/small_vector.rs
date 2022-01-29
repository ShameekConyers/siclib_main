use super::Vector;

pub struct SmallVector<T> {
    buffer: [T; 10],
    extra_buffer: Vector<T>,
}
