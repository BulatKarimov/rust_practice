mod tasks;

struct Task {
    name: &'static str,
    enabled: bool,
    run: fn(),
}

fn main() {
    let tasks = [
        Task {
            name: "AUTO_GUESS_NUMBER_TASK",
            enabled: false,
            run: tasks::guess_number_task::run_auto,
        },
        Task {
            name: "MANUAL_GUESS_NUMBER_TASK",
            enabled: false,
            run: tasks::guess_number_task::run_manual,
        },
        Task {
            name: "CALC_TASK",
            enabled: false,
            run: tasks::calc_task::run,
        },
        Task {
            name: "FIZZ_BUZZ_TASK",
            enabled: true,
            run: tasks::fizz_buzz_task::run,
        },
    ];

    for task in tasks {
        if task.enabled {
            println!("{} TASK\n", task.name);
            (task.run)();
            println!("==========================================\n\n\n");
        }
    }
}
