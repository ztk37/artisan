pub mod init;
pub mod license;
pub mod new;
pub mod todo;

pub fn not_implemented_yet() -> Result<(), (i32, String)> {
    Err((1, String::from("command not implemented yet")))
}