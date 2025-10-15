mod tasks;

const AUTO_GUESS_NUMBER_TASK_AVAILABLE: bool = false;
const MANUAL_GUESS_NUMBER_TASK_AVAILABLE: bool = false;
const CALC_TASK_AVAILABLE: bool = true;

fn main() {
    if AUTO_GUESS_NUMBER_TASK_AVAILABLE {
        println!("[AUTO_GUESS_NUMBER TASK]\n");
        tasks::guess_number_task::run_auto();
        println!("==========================================\n\n\n");
    }

    if MANUAL_GUESS_NUMBER_TASK_AVAILABLE {
        println!("[MANUAL_GUESS_NUMBER TASK]\n");
        tasks::guess_number_task::run_manual();
        println!("==========================================\n\n\n");
    }

    if CALC_TASK_AVAILABLE {
        println!("[CALC TASK]\n");
        tasks::calc_task::run();
        println!("==========================================\n\n\n");
    }
}
