use super::Stepper;

#[test]
fn step_up1() {
    assert_eq!(vec![0, 1, 2, 3, 4], step!(0 => 5; 1).into_iter().collect::<Vec<_>>());
}

#[test]
fn step_up2() {
    assert_eq!(vec![0, 2, 4], step!(0 => 5; 2).into_iter().collect::<Vec<_>>());
}

#[test]
fn step_up3() {
    assert_eq!(vec![0, 4], step!(0 => 8; 4).into_iter().collect::<Vec<_>>());
}

#[test]
fn step_down1() {
    assert_eq!(vec![5, 4, 3, 2, 1], step!(5 => 0; -1).into_iter().collect::<Vec<_>>());
}

#[test]
fn step_down2() {
    assert_eq!(vec![5, 3, 1], step!(5 => 0; -2).into_iter().collect::<Vec<_>>());
}

#[test]
fn step_down3() {
    assert_eq!(vec![8, 4], step!(8 => 0; -4).into_iter().collect::<Vec<_>>());
}
