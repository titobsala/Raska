use colored::*;

/// Displays a simple progress bar
pub fn display_progress_bar(completed: usize, total: usize) {
    let percentage = if total > 0 { (completed * 100) / total } else { 0 };
    let bar_width = 40;
    let filled = (percentage * bar_width) / 100;
    let empty = bar_width - filled;
    
    let filled_bar = "█".repeat(filled).bright_green();
    let empty_bar = "░".repeat(empty).bright_black();
    
    println!("  Progress: [{}{}] {}% ({}/{})", 
        filled_bar, empty_bar, percentage, completed, total);
}

/// Display motivational messages based on progress
pub fn display_motivational_message(completed: usize, total: usize) {
    if total == 0 {
        println!("  🌟 Ready to start your project!");
        return;
    }
    
    let percentage = (completed * 100) / total;
    let remaining = total - completed;
    
    match percentage {
        0 => println!("  🚀 Ready to start? Complete your first task!"),
        1..=25 => println!("  💪 Keep going! {} tasks remaining.", remaining),
        26..=50 => println!("  🎯 Great progress! You're {} tasks away from halfway.", total/2 - completed),
        51..=75 => println!("  🔥 Over halfway there! {} more to go!", remaining),
        76..=99 => println!("  🏁 Almost done! Just {} tasks left!", remaining),
        100 => println!("  🎉 Congratulations! All tasks completed! 🎊"),
        _ => println!("  📈 Keep up the great work!"),
    }
}