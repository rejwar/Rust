#[rtic::app(device = stm32f4xx_hal::pac)]
mod app {
    #[task]
    fn process_data() {
        println!("Real-Time Task Execution!");
    }
}
