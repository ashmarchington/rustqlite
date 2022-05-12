const VARCHAR_MAX: u32 = 255;
const VARCHAR_MIN: u32 = 1;

struct VarcharLength {
    value: u32,
}

impl VarcharLength {
    fn new(value: u32) -> VarcharLength {
        VarcharLength { value }
    }

    fn create(length: u32) -> Result<Self, &'static str> {
        match length {
            length if length > VARCHAR_MAX => Err("Value out of Bounded Range: Too High - Max 255"),
            length if length < VARCHAR_MIN => Err("Value out of Bounded Range: Too low - Min 1"),
            _ => Ok(VarcharLength::new(length)),
        }
    }
}

pub struct Varchar {
    length: Result<VarcharLength, &'static str>,
}

impl Varchar {
    pub fn new(length: u32) -> Varchar {
        Varchar {
            length: VarcharLength::create(length),
        }
    }
}

pub struct Row {
    id: i32,
    username: String,
    email: String,
}

impl Row {
    pub fn new(id: i32, username: String, email: String) -> Row {
        Row {
            id,
            username,
            email,
        }
    }
}
