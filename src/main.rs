slint::include_modules!();

use slint::Timer;

use chrono::Local;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let timer = Timer::default();
    ui.on_start_timer({
        let ui = ui.as_weak().unwrap();
        move || {
            let ui = ui.as_weak().unwrap();
            timer.start(
                slint::TimerMode::Repeated,
                std::time::Duration::from_secs(1),
                move || {
                    let time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                    ui.set_time(time.into());
                },
            );
        }
    });

    ui.run()
}
