#![warn(missing_docs)]
//! A Rust reimplementation sio2jail - a tool for supervising the execution of programs submitted in algorithmic competitions

/// Utilities for creating and managing libsio2jail processes
pub mod process;
/// Utilities for setting Linux up for libsio2jail use
pub mod setup;

mod listener;
mod util;

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::os::fd::AsFd;
    use std::time::Duration;

    use crate::process::execution_result::ExitReason::Exited;
    use crate::process::executor::Feature::PERF;
    use crate::process::executor::Sio2jailExecutor;

    #[test]
    fn time_measurement_test() {
        //TODO: COMPREHENSIVE UNIT TESTING SYSTEM
        
        let input_file = File::open("tests/bud.in").unwrap();
        let output_file = File::create("tests/test_output.out").unwrap();

        let child = Sio2jailExecutor::new("tests/bud")
            .stdin(input_file.as_fd())
            .stdout(output_file.as_fd())
            //.current_dir(PathBuf::from("asfd"))
            .features(PERF)
            .measured_time_limit(Duration::from_millis(500))
            .spawn()
            .unwrap();
        let result = child.run().unwrap();

        println!("Exit result: {:?}", result);
        let Exited {
            exit_status: status,
        } = result.exit_reason
        else {
            panic!("not supposed to happen")
        };
        println!(
            "Exit status: {}, measured time: {}",
            status,
            result.measured_time.unwrap().as_millis()
        );

        assert_eq!(result.measured_time.unwrap().as_millis(), 458);
    }
}
