use std::{
    borrow::Cow,
    path::{Component, Path, PathBuf},
};

use etcetera::home_dir;

/// Expands tilde `~` into users home directory if available, otherwise returns the path
/// unchanged.
///
/// The tilde will only be expanded when present as the first component of the path
/// and only slash follows it.
pub fn expand_tilde<'a, P>(path: P) -> Cow<'a, Path>
where
    P: Into<Cow<'a, Path>>,
{
    let path = path.into();
    let mut components = path.components();
    if let Some(Component::Normal(c)) = components.next() {
        if c == "~" {
            if let Ok(mut buf) = home_dir() {
                buf.push(components);
                return Cow::Owned(buf);
            }
        }
    }

    path
}

/// Normalize a path without resolving symlinks.
// Strategy: start from the first component and move up. Canonicalize previous path,
// join component, canonicalize new path, strip prefix and join to the final result.
pub fn normalize(path: impl AsRef<Path>) -> PathBuf {
    let mut components = path.as_ref().components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().copied() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            #[cfg(not(windows))]
            Component::ParentDir => {
                ret.pop();
            }
            #[cfg(windows)]
            Component::ParentDir => {
                if let Some(head) = ret.components().next_back() {
                    match head {
                        Component::Prefix(_) | Component::RootDir => {}
                        Component::CurDir => unreachable!(),
                        // If we left previous component as ".." it means we met a symlink before and we can't pop path.
                        Component::ParentDir => {
                            ret.push("..");
                        }
                        Component::Normal(_) => {
                            if ret.is_symlink() {
                                ret.push("..");
                            } else {
                                ret.pop();
                            }
                        }
                    }
                }
            }
            #[cfg(not(windows))]
            Component::Normal(c) => {
                ret.push(c);
            }
            #[cfg(windows)]
            Component::Normal(c) => 'normal: {
                use std::fs::canonicalize;

                let new_path = ret.join(c);
                if new_path.is_symlink() {
                    ret = new_path;
                    break 'normal;
                }
                let (can_new, can_old) = (canonicalize(&new_path), canonicalize(&ret));
                match (can_new, can_old) {
                    (Ok(can_new), Ok(can_old)) => {
                        let striped = can_new.strip_prefix(can_old);
                        ret.push(striped.unwrap_or_else(|_| c.as_ref()));
                    }
                    _ => ret.push(c),
                }
            }
        }
    }
    dunce::simplified(&ret).to_path_buf()
}
