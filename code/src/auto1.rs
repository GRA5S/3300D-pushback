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
    // Evian PID
// Starting point: (60.63 in, -15.16 in)
_ = robot.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = robot.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = robot.hood.set_high();
_ = robot.midgoal.set_low();
// Point 2
dt.tracking.set_heading(270.00.deg());
// Point 2
basic
    .drive_distance(dt, 15.92)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// Point 3
basic
    .turn_to_heading(dt, 287.68.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 19.03)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
// Point 4
basic
    .turn_to_heading(dt, 203.20.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 7.08)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
_ = robot.matchload.toggle(); //matchload
// Point 5
basic
    .turn_to_heading(dt, 201.50.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 16.49)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
_ = robot.matchload.toggle(); //matchload
sleep(Duration::from_millis(100)).await;
// Point 6
basic
    .turn_to_heading(dt, 114.33.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 37.25)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
_ = robot.matchload.toggle(); //matchload
sleep(Duration::from_millis(100)).await;
// Point 7
basic
    .turn_to_heading(dt, 88.21.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, 14.89)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
sleep(Duration::from_millis(500)).await;
// Point 8
basic
    .turn_to_heading(dt, 88.71.deg())
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
basic
    .drive_distance(dt, -41.40)
    .with_timeout(Duration::from_millis(2000))
    .with_linear_output_limit(1.0)
    .await;
_ = robot.intake1.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = robot.intake2.set_voltage(Motor::V5_MAX_VOLTAGE);
_ = robot.hood.set_low();
_ = robot.midgoal.set_low(); //score
sleep(Duration::from_millis(4000)).await;

}
