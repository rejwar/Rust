macro_rules! sum_rows {
    ( $( [ $( $x:expr ),* ] );* ) => {{
        let mut sums = Vec::new();
        $(
            let mut s = 0;
            $(
                s += $x;
            )*
            sums.push(s);
        )*
        sums
    }};
}

// sum_rows!([1,2,3]; [4,5]) => vec![6, 9]
