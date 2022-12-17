/// Creates a [`Vec`] containing the arguments.
/// [`vecu!`] unpacks iterable variables marked with `@`.
///
/// ```
/// use vec_unpack::*;
/// let a = [2, 3];
/// let b = vecu![0, 1, @a, 4, 5];
/// assert_eq!(b, vec![0, 1, 2, 3, 4, 5]);
/// ```
#[macro_export]
macro_rules! vecu {
    () => {
        Vec::new()
    };
    ($($x:expr),+ $(,)?) => {
        vec![$($x),+]
    };
    ($($input:tt)*) => {
        {
            let mut v = Vec::new();
            vecu_inner!(v, $($input)*);
            v
        }
    };
}

/// Used in [`vecu!`].
#[macro_export]
macro_rules! vecu_inner {
    ($v:ident,) => {};
    ($v:ident, $x:expr) => {
        $v.push($x);
    };
    ($v:ident, @$x:expr) => {
        $v.extend($x);
    };
    ($v:ident, $x:expr, $($tail:tt)*) => {
        $v.push($x);
        vecu_inner!($v, $($tail)*)
    };
    ($v:ident, @$x:expr, $($tail:tt)*) => {
        $v.extend($x);
        vecu_inner!($v, $($tail)*)
    };
}

#[cfg(test)]
mod tests {
    use super::vecu;

    #[test]
    fn empty() {
        let vu: Vec<i32> = vecu![];
        let v: Vec<i32> = vec![];
        assert_eq!(vu, v);
    }

    #[test]
    fn with_tail_comma() {
        let a = [2, 3];
        let b = vecu![0, 1, @a, 4, 5,];
        assert_eq!(b, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn without_tail_comma() {
        let a = [2, 3];
        let b = vecu![0, 1, @a, 4, 5];
        assert_eq!(b, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn multi_unpack() {
        let a = [1, 2];
        let b = [3, 4, 5];
        let c = vecu![0, @a, @b, 6, 7];
        assert_eq!(c, vec![0, 1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn without_unpack() {
        assert_eq!(vecu![0, 1, 2], vec![0, 1, 2]);
    }

    #[test]
    fn unpack_only() {
        let a = [1, 2];
        let b = vecu![@a];
        assert_eq!(b, vec![1, 2]);
    }
}
