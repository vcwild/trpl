// Declarative macros are defined using the macro_rules! macro.
#[macro_export]
macro_rules! vic {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new(); // first match
            $(
                temp_vec.push($x);
            )* // every match
            temp_vec
        }
    };
}

#[macro_export]
macro_rules! nstr {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = String::new(); // first match
            $(
                temp_vec.push_str($x);
            )* // every match
            temp_vec
        }
    };
}
