use errno::errno;
use std::ffi::CString;

/// Get information about a file or a symbolic link
///
/// Read more about info it returns [www.man7.org/linux/man-pages/man2/stat.2.html](https://www.man7.org/linux/man-pages/man2/stat.2.html)
///
/// It is platform agnostic `libc` binding for the `lstat`. In short, as long as the
/// platform for which this program is being compiled to supports `libc`, `lstat`
/// will return back `libc::stat` which will have fields appropriate to the platform.
///
/// # Example
///
/// Here is an example where we are trying to get info about a symbolic link, which will work
/// just fine:
///
/// ```
/// use fs_stat::lstat;
///
/// // get uid of the current user
/// let uid = unsafe { libc::getuid() };
///
/// match lstat("files/symlink-to-the-question.txt") {
///   Ok(stat) => assert_eq!(stat.st_uid, uid),
///   Err(_) => panic!("should not reach here since lstat should work with links"),
/// }
/// ```
///
pub fn lstat<T: Into<Vec<u8>>>(path: T) -> std::io::Result<libc::stat> {
  let mut stat: libc::stat = unsafe { std::mem::zeroed() };

  let c_path = CString::new(path)?;
  let c_errno = unsafe { libc::lstat(c_path.as_ptr(), &mut stat) };

  match c_errno {
    0 => Ok(stat),
    _ => Err(errno().into()),
  }
}

#[cfg(test)]
mod tests {
  use super::lstat;
  use pretty_assertions::{assert_eq, assert_ne};
  use std::io::ErrorKind;
  use std::io::Result;

  #[test]
  fn should_find_a_file() -> Result<()> {
    let stat = lstat("files/the-question.txt")?;

    let uid = unsafe { libc::getuid() };
    assert_eq!(stat.st_uid, uid);

    Ok(())
  }

  #[test]
  fn should_not_find_a_file() {
    let res = lstat("hl3.exe");

    let err = res.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::NotFound);
  }

  #[test]
  fn should_get_stats_for_the_link() -> Result<()> {
    let link_stat = lstat("files/symlink-to-the-question.txt")?;
    let file_stat = lstat("files/the-question.txt")?;

    assert_ne!(link_stat.st_ctime, file_stat.st_ctime);

    Ok(())
  }
}
