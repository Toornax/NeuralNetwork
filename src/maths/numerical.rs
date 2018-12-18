use std::ops::{Add, Mul};

pub trait Number: Add<Output=Self>, Div<Output=Self>, Mul<Output=Self>;
