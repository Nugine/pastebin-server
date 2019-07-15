#[macro_export]
macro_rules! or_return {
    ($option:expr, $value:expr) => {
        match $option {
            None => return $value,
            Some(t) => t,
        }
    };
}

#[cfg(test)]
#[test]
fn test_or_return() {
    let fa = || or_return!(None, 1);
    assert_eq!(fa(), 1);

    let fb = || {
        let b = or_return!(Some(2), 0);
        b * b
    };
    assert_eq!(fb(), 4);
}
