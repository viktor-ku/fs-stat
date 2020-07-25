use errno::errno;
use std::ffi::CString;

/// Get information about a file
///
/// Read more about info it returns [www.man7.org/linux/man-pages/man2/stat.2.html](https://www.man7.org/linux/man-pages/man2/stat.2.html)
///
/// It is platform agnostic `libc` binding for the `stat`. In short, as long as the
/// platform for which this program is being compiled to supports `libc`, `stat`
/// will return back `libc::stat` which will have fields appropriate to the platform.
///
/// # Examples
///
/// Here is an example of getting information about a regular file:
///
/// ```
/// use fs_stat::stat;
///
/// let stat = stat("Cargo.toml")?;
/// println!("file is owned by the {} from the group {}", stat.st_uid, stat.st_gid);
///
/// // get uid of the current user
/// let uid = unsafe { libc::getuid() };
///
/// // make sure that the current file belongs to the current user
/// assert_eq!(uid, stat.st_uid);
///
/// # Ok::<(), std::io::Error>(())
/// ```
///
/// Here is an example where we are trying to get info about a symbolic link, which will not
/// go well:
///
/// ```
/// use fs_stat::stat;
///
/// match stat("files/symlink-to-the-question.txt") {
///   Ok(_) => panic!("should not reach here since stat should not work with links"),
///   Err(e) => assert_eq!(e.kind(), std::io::ErrorKind::NotFound),
/// }
/// ```
///
pub fn stat<T: Into<Vec<u8>>>(path: T) -> std::io::Result<libc::stat> {
  let mut stat: libc::stat = unsafe { std::mem::zeroed() };

  let c_path = CString::new(path)?;
  let c_errno = unsafe { libc::stat(c_path.as_ptr(), &mut stat) };

  match c_errno {
    0 => Ok(stat),
    _ => Err(errno().into()),
  }
}

#[cfg(test)]
mod tests {
  use super::stat;
  use pretty_assertions::assert_eq;
  use std::io::ErrorKind;
  use std::io::Result;

  #[test]
  fn should_find_a_file() -> Result<()> {
    let stat = stat("files/the-question.txt")?;

    let uid = unsafe { libc::getuid() };
    assert_eq!(stat.st_uid, uid);

    Ok(())
  }

  #[test]
  fn should_not_find_a_file() {
    let res = stat("hl3.exe");

    let err = res.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::NotFound);
  }

  #[test]
  fn should_not_be_able_to_get_stats_for_the_link() {
    let res = stat("files/symlink-to-the-question.txt");

    let err = res.err().unwrap();
    assert_eq!(err.kind(), ErrorKind::NotFound);
  }
}
