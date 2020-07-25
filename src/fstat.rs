use errno::errno;

/// Get information about a file, like `stat`, but using file descriptor (`fd`) instead
///
/// Read more about info it returns [www.man7.org/linux/man-pages/man2/stat.2.html](https://www.man7.org/linux/man-pages/man2/stat.2.html)
///
/// It is platform agnostic `libc` binding for the `fstat`. In short, as long as the
/// platform for which this program is being compiled to supports `libc`, `fstat`
/// will return back `libc::stat` which will have fields appropriate to the platform.
///
/// # Examples
///
/// Here is an example of getting information about a regular file:
///
/// ```
/// use fs_stat::fstat;
///
/// // get file descriptor anyhow; here we are cheating a little bit :)
/// use std::fs::File;
/// use std::os::unix::io::AsRawFd;
/// let file = File::open("files/the-question.txt")?;
/// let fd = file.as_raw_fd();
///
/// let stat = fstat(fd)?;
///
/// // get current user id
/// let uid = unsafe { libc::getuid() };
///
/// assert_eq!(uid, stat.st_uid);
///
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn fstat(fd: i32) -> std::io::Result<libc::stat> {
  let mut stat: libc::stat = unsafe { std::mem::zeroed() };
  let c_errno = unsafe { libc::fstat(fd, &mut stat) };

  match c_errno {
    0 => Ok(stat),
    _ => Err(errno().into()),
  }
}
