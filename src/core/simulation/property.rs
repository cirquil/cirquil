use std::cell::{Cell, RefCell};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

// pub struct PropertyStruct<T: FromStr + ToString> {
//     name: String,
//     value: Cell<T>
// }

pub type PropertyIdx = usize;

pub trait Property: Debug {
    fn get_name(&self) -> String;
    fn get_value(&self) -> String;
    fn parse(&self, string: &str) -> Result<(), Box<dyn Error>>;
}

#[derive(Debug)]
pub struct IntegerProperty {
    pub name: String,
    pub value: Cell<u32>
}
impl Property for IntegerProperty {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_value(&self) -> String {
        self.value.get().to_string()
    }

    fn parse(&self, string: &str) -> Result<(), Box<dyn Error>> {
        Ok(self.value.set(string.parse()?))
    }
}

#[derive(Debug)]
pub struct StringProperty {
    pub name: String,
    pub value: RefCell<String>
}

impl Property for StringProperty {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_value(&self) -> String {
        self.value.borrow().clone()
    }

    fn parse(&self, string: &str) -> Result<(), Box<dyn Error>> {
        *self.value.borrow_mut() = string.to_string();
        Ok(())
    }
}

#[derive(Debug)]
pub struct BoundedIntegerProperty {
    name: String,
    min: u32,
    max: u32,
    value: Cell<u32>
}

#[derive(Debug)]
struct BoundsError;

impl Display for BoundsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Value is out of bounds")
    }
}

impl Error for BoundsError {}

impl Property for BoundedIntegerProperty {
    fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_value(&self) -> String {
        self.value.get().to_string()
    }

    fn parse(&self, string: &str) -> Result<(), Box<dyn Error>> {
        let value = string.parse::<u32>()?;

        if (self.min <= value) && (value < self.max) {
            Ok(self.value.set(string.parse()?))
        } else {
            Err(Box::new(BoundsError))
        }
    }
}
