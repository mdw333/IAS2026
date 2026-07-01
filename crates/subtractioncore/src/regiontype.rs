#[allow(non_camel_case_types)]
#[derive(Copy,Clone)]
#[repr(usize)]
pub enum regiontype {
    sw,
    ss,
    se,
    ne,
    nn,
    nw,
}

impl std::fmt::Debug for regiontype {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       match self {
           regiontype::sw => write!(f, "sw"),
           regiontype::ss => write!(f, "ss"),
           regiontype::se => write!(f, "se"),
           regiontype::ne => write!(f, "ne"),
           regiontype::nn => write!(f, "nn"),
           regiontype::nw => write!(f, "nw"),
       }
    }
}

