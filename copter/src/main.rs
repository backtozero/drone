extern crate ads111x;
extern crate configurations;
extern crate debug_server;
extern crate i2cdev;
extern crate i2cdev_bmp180;
extern crate i2cdev_bmp280;
extern crate i2cdev_l3gd20;
extern crate i2cdev_lsm303d;
extern crate i2cdev_lsm9ds0;
extern crate i2csensors;
extern crate logger;
extern crate nalgebra as na;
extern crate pca9685;
#[macro_use]
extern crate rulinalg;
extern crate time;
extern crate typenum;
// extern crate rust_pigpio;
extern crate unbounded_gpsd;
extern crate wifilocation;

use logger::ModuleLogger;

use configurations::Config;

mod hardware;
// mod flight;
// mod networking;

// use flight::{start_flight, FlightMode};
// use hardware::sensors::calibrate_sensors;

// use flight::{FlightMode,start_flight};
// use hardware::motors::MotorManager;

use std::io::stdin;
use std::string::String;
use std::thread;

// use sensor_manager::{start_sensors, calibrate_sensors};

fn main() {
    println!("Enter one of the following options:");
    println!("1: Fly");
    println!("2: Calibrate");

    let mut input = String::new();
    stdin().read_line(&mut input).expect("Error");
    match input.trim() {
        "run_motors" => {}
        "calibrate" => {
            // hardware::motors::calibrate();
        }
        "calibrate sensors" => {
            println!("Place drone on a completely level surface. Then press enter.");
            input = String::new();
            stdin().read_line(&mut input).expect("Error");

            // calibrate_sensors();
        }
        _ => {
            start();
        }
    }
}

fn start() {
    // let logger = ModuleLogger::new("Input", None);
    // let (flight_mode_controller, control_thread_handler) = start_flight();

    // logger.log("Press enter to start the flight.");
    // let mut input = String::new();
    // stdin().read_line(&mut input).expect("Error");

    // flight_mode_controller.send(FlightMode::TakeOff);

    // logger.log("Press enter to stop the flight.");
    // let mut input = String::new();
    // stdin().read_line(&mut input).expect("Error");

    // flight_mode_controller.send(FlightMode::Landing);

    // logger.log("Press enter to shutdown.");
    // let mut input = String::new();
    // stdin().read_line(&mut input).expect("Error");

    // flight_mode_controller.send(FlightMode::Shutdown);
    // control_thread_handler.join();
}
