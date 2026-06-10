pub mod c_maker;
pub mod serial_explorer;

pub mod device_list;

pub trait Run {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>>;
}
