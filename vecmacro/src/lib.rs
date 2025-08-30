#[macro_export]
macro_rules! avec {
    ($($element:expr),*) => {{
      const COUNT: usize = $crate::count![@COUNT; $($element),*];

       #[allow(unused_mut)]
        let mut vs = Vec::with_capacity(COUNT);
        $(vs.push($element);)*
        vs
    }};
    ($($element:expr,)*) => {{
      $crate::avec![$($element),*]
    }};
    ($element:expr; $count:expr) => {{
      let mut vs = Vec::new();
      vs.resize($count, $element);
      vs
    }}
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element:expr),*) => {
      <[()]>::len(&[$($crate::count![@SUBST; $element]),*])
    };

    (@SUBST; $element:expr) =>{()}
}

#[test]
fn empty_test() {
    let x: Vec<u32> = avec![];

    assert!(x.is_empty())
}

#[test]
fn single_test() {
    let x: Vec<u32> = avec![42];

    assert_eq!(x.len(), 1);
    assert_eq!(x[0], 42);
}

#[test]
fn double_test() {
    let x: Vec<u32> = avec![42, 100];

    assert_eq!(x.len(), 2);
    assert_eq!(x[0], 42);
    assert_eq!(x[1], 100);
}

#[test]
fn trailing_comma_test() {
    let x: Vec<u32> = avec![42, 100, 52, 33, 11, 4, 22, 42, 52,];

    assert_eq!(x.len(), 9);
}

#[test]
fn repetition_test() {
    let mut el = Some(10);
    let x: Vec<u32> = avec![el.take().unwrap(); 5];

    assert_eq!(x.len(), 5);
    assert_eq!(x[0], 10);
    assert_eq!(x[1], 10);
    assert_eq!(x[2], 10);
    assert_eq!(x[3], 10);
    assert_eq!(x[4], 10);
}

/// ```compile_fail
/// let x: Vec<usize> = vecmacro::avec![5; "foo"];
/// ```
#[allow(dead_code)]
pub struct CompileFailTest;
