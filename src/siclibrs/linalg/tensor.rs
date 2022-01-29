use crate::collections::{RawVector, SmallVector};
pub struct Tensor<T> {
    // shared
    data: RawVector<T>,
    size: SmallVector<T>,
}
