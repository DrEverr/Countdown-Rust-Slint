slint::include_modules!();

use slint::Timer;

use chrono::{Local, NaiveDateTime};

fn main() -> Result<(), slint::PlatformError> {
    use slint::Model;
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
                    let ui = ui.as_weak().unwrap();

                    let mut timers: Vec<DeltaTime> = ui.get_timers().iter().collect();
                    for time in timers.iter_mut() {
                        if !time.running {
                            continue;
                        }
                        time.seconds -= 1;
                    }
                    let timers = std::rc::Rc::new(slint::VecModel::from(timers));
                    ui.set_timers(timers.into());
                },
            );
        }
    });

    ui.on_add_timer({
        let ui = ui.as_weak().unwrap();
        ui.set_hint("".into());

        move |text| {
            let ui = ui.as_weak().unwrap();
            let timer = NaiveDateTime::parse_from_str(&text, "%Y-%m-%d %H:%M:%S");
            match timer {
                Ok(timer) => {
                    let mut timers: Vec<DeltaTime> = ui.get_timers().iter().collect();
                    let timer = timer - Local::now().naive_local();
                    let mut new_timer = timers.first().unwrap().clone();
                    new_timer.seconds = (timer.num_seconds() as i32) % 60;
                    new_timer.minutes = (timer.num_minutes() as i32) % 60;
                    new_timer.hours = (timer.num_hours() as i32) % 24;
                    new_timer.days = timer.num_days() as i32;
                    new_timer.running = true;
                    timers.push(new_timer);
                    let timers = std::rc::Rc::new(slint::VecModel::from(timers));
                    ui.set_timers(timers.into());
                }
                Err(_) => {
                    ui.set_hint("Invalid time format. Should be \"%Y-%m-%d %H:%M:%S\"".into());
                }
            }
        }
    });

    ui.run()
}
