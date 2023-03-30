/// LS exit codes
/// https://www.gnu.org/software/coreutils/manual/html_node/ls-invocation.h

/// The program exited successfully.
// pub static OK: i32 = 0;
/// The program exited with a minor error.
/// (e.g., failure to access a file or directory not
/// specified as a command line argument.  This happens when listing a
/// directory in which entries are actively being removed or renamed.)
// pub static MINOR_PROBLEM: i32 = 1;

/// (e.g., memory exhausted, invalid option, failure
///  to access a file or directory specified as a command line argument
///  or a directory loop)
pub static SERIOUS_TROUBLE: i32 = 2;
