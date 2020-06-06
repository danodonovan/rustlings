// result1.rs
// Make this test pass! Execute `rustlings hint result1` for hints :)

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value > 0 {
            Ok(PositiveNonzeroInteger(value as u64))
        }
        else if value == 0 {
            Err(CreationError::Zero)
        } else {
            Err(CreationError::Negative)
        }
    }
}

#[test]
fn test_creation_1() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
}

#[test]
fn test_creation_2() {
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
}

#[test]
fn test_creation_3() {
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
