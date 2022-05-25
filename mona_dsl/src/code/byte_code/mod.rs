pub use add::ByteCodeAdd;
pub use save_name::ByteCodeSaveName;
pub use load_number::ByteCodeLoadNumber;
pub use load_name::ByteCodeLoadName;
pub use byte_code::{ByteCodeOp, ByteCode};

pub mod add;
pub mod byte_code;
pub mod save_name;
pub mod load_number;
pub mod load_name;
pub mod call;
pub mod neg;
pub mod load_bool;
pub mod access;
pub mod load_string;
pub mod mul;
pub mod div;
pub mod sub;
