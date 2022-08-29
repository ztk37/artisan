pub mod init;
pub mod new;
pub mod release;

pub fn not_implemented_yet() -> Result<(), (i32, String)> {
    Err((1, String::from("command not implemented yet")))
}