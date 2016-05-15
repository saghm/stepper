#[macro_export]
macro_rules! step {
    ($start:expr => $end:expr; $step:expr) => { Stepper::new($start, $end, $step).into_iter() }
}
