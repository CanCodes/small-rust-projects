/*
Expense Calculator
*/
extern crate colored;

use colored::*;

struct Expense {
    name: String,
    amount: u32
}
impl Expense {
    fn new(name: String, amount: u32) -> Expense {
        Expense {
            name: name.trim().to_string(),
            amount: amount
        }
    }

    fn list_expenses(expenses: &Vec<Expense>) {
        let mut c = 1;
        for expense in expenses.iter() {
            println!("{}. {} {}", c, expense.name.yellow(), expense.amount.to_string().red());
            c += 1i32
        }
    }

    fn calc_expense(expenses: &Vec<Expense>) -> u32 {
        let mut expense_amount: u32 = 0;
        for expense in expenses.iter() {
            expense_amount += expense.amount
        }
        expense_amount
    }

}



fn main() {
    println!("{}", "Welcome to the Awesome Expense Calculater!".bold().magenta());
    let mut salary = String::new();
    println!("{}", "Please enter your monthly income:".bold());
    std::io::stdin().read_line(&mut salary).unwrap();
    let salary: u32 = salary.trim().parse().unwrap();
    let mut expenses: Vec<Expense> = Vec::new();
    loop {
        println!("Enter one of the following commands:\n    Add\n    Remove\n    List\n    Status\n    Quit");
        let mut command = String::new();
        std::io::stdin().read_line(&mut command).unwrap();
        println!("{}", "========================".bold().magenta());
        match command.trim().to_lowercase().as_str() {
            "add" => {
                let mut name = String::new();
                let mut amount = String::new();
                println!("{}", "Enter a name for your Expense:".bold());
                std::io::stdin().read_line(&mut name).unwrap();
                loop {
                    println!("{}", "Enter the cost of it:".bold());
                    std::io::stdin().read_line(&mut amount).unwrap();
                    let amount = amount.trim().parse::<u32>();
                    match amount {
                        Ok(amount) => {
                            println!("Done! Added {} Expense to your expenses.", name.trim());
                            expenses.push(Expense::new(name, amount));
                            break;
                        }
                        Err(_) => {
                            println!("Error parsing the cost, are you sure you've entered a number?");
                            continue;
                        }
                    }
                }
            }
            "remove" => {
                println!("{}", "Enter the Expense id you'd like to remove from your list:".bold());
                Expense::list_expenses(&expenses);
                let mut id = String::new();
                std::io::stdin().read_line(&mut id).unwrap();
                let id: usize = id.trim().parse().unwrap();
                if expenses.len() < id {
                    return println!("{}", "You've entered a wrong ID!".red().bold());
                }
                let expense = expenses.remove(id - 1);
                println!("Removed the {} from your Expense list!", expense.name.yellow())
            }
            "list" => {
                println!("{}","Here are your expenses:".bold().magenta());
                Expense::list_expenses(&expenses);
            }
            "status" => {
                let total_expense = Expense::calc_expense(&expenses);
                println!("Your Monthly income is: {}", salary.to_string().green());
                println!("Your Monthly Expense is: {}", total_expense.to_string().red());
                if total_expense >= salary {
                    println!("{}", "You are spending way too much money! Try cutting some expenses?".yellow().bold())
                } else {
                    println!("{} {}", "Great! You have some leftovers for this month!".green(), (&salary-&total_expense).to_string().green().bold())
                }
            }
            "quit" => {break}
            _ => continue
        }
        println!("{}", "========================".bold().magenta());
        println!()
    }
}