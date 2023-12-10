use std::ops::BitAnd;
/*
union Value {
    uint64_t value;
    struct {
        uint32_t mask;
        uint32_t value;
    }
}
*/

#[derive(Debug, Copy, Clone)]
pub struct Value {
    pub mask: u32,
    pub value: u32
}

impl Default for Value {
    fn default() -> Self {
        Value {
            mask: 0xff_ff_ff_ff,
            value: 0
        }
    }
}

impl BitAnd for Value {
    type Output = Value;

    fn bitand(self, rhs: Self) -> Self::Output {
        Value {
            mask: self.mask & rhs.mask,
            value: self.value & rhs.value,
        }
    }
}

// #[repr(C)]
// union Value {
//     value: u64,
//     foo: CompositeValue
// }
