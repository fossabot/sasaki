#[macro_export]
macro_rules! letrec {
  ($init:ident = $val:expr, $($lhs:ident = $rhs:expr),*) => {
      let $init = $val;
    $(
      let $lhs = $rhs;
    )*
  };
}
