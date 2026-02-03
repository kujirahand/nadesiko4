/**
 * Nadesiko4 VM code definitions
 */

/// VM code type
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VmcodeKind {
    Nop = 0,
    Print = 1,
    PushString = 2,
}

/// VM code argument
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VmArg {
    I(i64),
    U(u64),
    F(f64),
    P(usize),
}

/// VM code structure
#[derive(Clone, Copy, Debug)]
pub struct Vmcode {
    pub kind: VmcodeKind,
    pub arg: VmArg,
}

/// VM code data structure
#[derive(Clone, Debug)]
pub struct VmcodeData {
    pub data: Vec<u8>,
}

/// VM code list structure
#[derive(Clone, Debug)]
pub struct VmSystem {
    pub codes: Vec<Vmcode>,
    pub str_list: Vec<String>,
    pub num_list: Vec<f64>,
    pub bin_list: Vec<VmcodeData>,
    pub stack: Vec<>,
    pub output: String,
    pub error: String,
}
impl VmSystem {
    pub fn new() -> Self {
        VmSystem {
            codes: Vec::new(),
            str_list: Vec::new(),
            num_list: Vec::new(),
            bin_list: Vec::new(),
            output: String::new(),
            error: String::new(),
        }
    }
}
