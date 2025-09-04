#[macro_export]
#[cfg(feature = "metrics")]
macro_rules! with_metric {
    ($counter: expr, $method: ident, $($args: expr),*) => {
        $counter.get().inspect(|c| c.value.$method($($args),*));
    };
}

#[macro_export]
#[cfg(not(feature = "metrics"))]
macro_rules! with_metric {
    ($counter: expr, $method: ident, $($args: expr),*) => {
        ();
    };
}

#[macro_export]
#[cfg(feature = "metrics")]
macro_rules! with_histogram {
    ($counter: expr, $method: ident, $($args: expr),*) => {
        $counter.get().inspect(|c| c.$method($($args),*));
    };
}
#[macro_export]
#[cfg(not(feature = "metrics"))]
macro_rules! with_histogram {
    ($counter: expr, $method: ident, $($args: expr),*) => {
        ();
    };
}
