use std::time::Duration;
use evian::{
    motion::{Basic, Seeking},
    control::loops::{Pid, AngularPid},
    prelude::*,
};
use vexide::prelude::*;
use crate::Robot;

pub async fn run(robot: &mut Robot, basic: &mut Basic<Pid, AngularPid>, seeking: &mut Seeking<Pid, Pid>) {
    let dt = &mut robot.drivetrain;
    basic
        .drive_distance(dt, -9999.0)
        .with_timeout(Duration::from_millis(2000))
        .with_linear_output_limit(0.3)
        .await;
}
