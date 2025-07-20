pub struct Epoch {
    content: String,
}

pub trait RepresentableEpoch {
    fn repr(&self) -> String;
}

impl Epoch {
    pub fn new() -> Epoch {
        Epoch {
            content: "00000000-0000-0000-0000-000000000000".to_string()
        }
    }

    pub fn from(str: &String) -> Epoch {
        Epoch {
            content: str.clone(),
        }
    }
}

impl RepresentableEpoch for Epoch {
    fn repr(&self) -> String {
        self.content.clone()
    }
}