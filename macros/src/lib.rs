/// 多次元の vector を作成します
///
/// Examples
/// ```
/// N*N の vector
/// let mut result = mat![0; N; N];
/// ```
#[macro_export]
macro_rules! mat {
($($e:expr),*) => { Vec::from(vec![$($e),*]) };
($($e:expr,)*) => { Vec::from(vec![$($e),*]) };
($e:expr; $d:expr) => { Vec::from(vec![$e; $d]) };
($e:expr; $d:expr $(; $ds:expr)+) => { Vec::from(vec![mat![$e $(; $ds)*]; $d]) };
}

#[macro_export]
macro_rules! echo { ($($num:expr),*)=> {
    let mut tmp=vec![];
    $ (tmp.push(format!("{}",$num));) *
    println!("{}",tmp.join(" ")); };
}

#[macro_export]
macro_rules! mint {
    ($num:expr) => {
        Mint::new($num)
    };
}

#[macro_export]
macro_rules! YesNo {
    ($num:expr) => {
        if ($num) as i64 == 0 {
            println!("No");
        } else {
            println!("Yes");
        }
    };
}
#[macro_export]
macro_rules! Yes {
    () => {
        println!("Yes");
    };
}
#[macro_export]
macro_rules! No {
    () => {
        println!("No");
    };
}
