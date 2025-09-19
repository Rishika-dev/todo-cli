pub enum Command {
    CreateFile(String),
    WriteFile(String, String),
    AppendFile(String, String),
    Invalid,
}
