use std::{
    io::{self, Write},
    str, thread,
    time::Duration,
};

//Basis for a specific item on the menu
#[derive(Clone)]
struct MenuItem {
    name: &'static str,
    price: f32,
    ftype: &'static str,
}

//functions (impl) of the MenuItem
impl MenuItem {
    //new item function
    const fn new(name: &'static str, price: f32, ftype: &'static str) -> Self {
        MenuItem { name, price, ftype }
    }
}

//Basis for a meal object
struct Meal {
    meats: Vec<MenuItem>,
    rice_type: MenuItem,
    toppings: Vec<MenuItem>,
    sauce: MenuItem,
    total_price: f32,
}

//functions (impl) of the Meal object
impl Meal {
    //makes a new Meal object
    fn new(
        meats: Vec<MenuItem>,
        rice_type: MenuItem,
        toppings: Vec<MenuItem>,
        sauce: MenuItem,
        total_price: f32,
    ) -> Self {
        Meal {
            meats,
            rice_type,
            toppings,
            sauce,
            total_price,
        }
    }
}

//builds the Menu as a constant
const MENU: [MenuItem; 12] = [
    MenuItem::new("Chicken", 5.0, "Meat"),
    MenuItem::new("Steak", 6.0, "Meat"),
    MenuItem::new("Shrimp", 7.0, "Meat"),
    MenuItem::new("Fried Rice", 2.0, "Rice"),
    MenuItem::new("Steamed Rice", 1.0, "Rice"),
    MenuItem::new("Zucchini", 0.75, "Topping"),
    MenuItem::new("Onions", 0.75, "Topping"),
    MenuItem::new("Mushrooms", 0.75, "Topping"),
    MenuItem::new("Bean Sprouts", 0.75, "Topping"),
    MenuItem::new("Yum Yum", 0.5, "Sauce"),
    MenuItem::new("Ginger", 0.5, "Sauce"),
    MenuItem::new("Sriracha", 0.25, "Sauce"),
];

//main function
fn main() {
    //initialize order and completion boolean
    let mut done: bool = false;
    let mut order: Vec<Meal> = Vec::new();

    //welcomes customer to Benji Bo and displays the menu
    println!("Welcome to Benji Bo!");
    sleep(2);
    println!("What would you like to order today?");
    sleep(2);
    println!("Here is our menu: ");
    sleep(2);
    show_menu();

    //starts the ordering loop
    loop {
        //opens the main menu and gets user input
        let resp: i32;
        resp = main_menu();

        //checks user input to get to correct option
        if resp == 1 {
            //builds meal and ensures meal is not empty
            let dblchk = build_meal();
            if dblchk.total_price <= 0.0 {
                continue;
            } else {
                order.push(dblchk);
            }
        } else if resp == 2 {
            //prints out order, if not empty, and removes meal if applicable
            let i = show_order(&order, &done);
            if i != 0 {
                let no: i32 = remove_order(i.into());
                if no > 0 {
                    let int: u32 = (no - 1) as u32;
                    let index: usize = int as usize;
                    order.remove(index);
                }
            }
        } else if resp == 3 {
            //dipslays order
            show_order(&order, &done);
        } else if resp == 4 {
            //displays the menu
            show_menu();
        } else if resp == 5 {
            //marks the ordering process complete
            done = true;
        }

        //checks to see if the ordering process is complete to break out of the loop
        if done == true {
            break;
        }
    }

    //prints the receipt
    print_receipt(&order, &done);
}

//function to display the menu
fn show_menu() {
    let mut last_type: &str = "";

    //runs through each item in the menu
    for item in MENU.iter() {
        //checks to see if a new section of the menu begins
        if item.ftype != last_type {
            println!("\n{}s: ", item.ftype);
            last_type = item.ftype;
        }
        println!("{}: ${:.2}", item.name, item.price);
        sleep(250);
    }
}

//simplifies thread sleeping
fn sleep(int: u64) {
    thread::sleep(Duration::from_millis(int))
}

//prints the main menu
fn main_menu() -> i32 {
    //prepares variables for user input
    let sin = io::stdin();
    let mut input = String::new();
    let mut goober: bool = false;
    let inp1: i32;

    //loop until valid response for main menu
    loop {
        sleep(2500);
        println!(
            "\n\n(1) Create new meal\n(2) Remove a meal\n(3) View Current Order\n(4) View Menu\n(5) Checkout"
        );
        print!("   What would you like to do? ");

        //reads user input
        io::stdout().flush().unwrap();
        sin.read_line(&mut input).unwrap();

        //parses user input if possible
        let inp = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "Invalid entry. Please enter a number between 1 and {}.\n",
                    5
                );
                continue;
            }
        };

        //checks if i32 is valid
        if inp == 1 || inp == 2 || inp == 3 || inp == 4 || inp == 5 {
            goober = true;
        } else {
            println!("Invalid entry. Please try again.\n")
        }

        //checks for loop completion
        if goober == true {
            inp1 = inp;
            break;
        }
    }
    return inp1;
}

//builds a meal to put in the order
fn build_meal() -> Meal {
    //initiializes variables for user input and meal creation
    let stin = io::stdin();
    let mut foo: bool = false;
    let mut price: f32 = 0.0;

    let mut meats: Vec<MenuItem> = Vec::new();
    let mut rice_type: MenuItem = MenuItem::new("No Rice", 0.0, "Rice");
    let mut toppings: Vec<MenuItem> = Vec::new();
    let mut sauce: MenuItem = MenuItem::new("No Sauce", 0.0, "Sauce");

    //initialize meat options
    loop {
        println!("\n\nOur meat options are as follows: \n (1) Chicken\n (2) Steak\n (3) Shrimp \n (4) Done with meat selection");
        print!("Which would you like? ");
        let mut input = String::new();

        //read user input
        io::stdout().flush().unwrap();
        stin.read_line(&mut input).unwrap();

        let inp = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "Invalid entry. Please enter a number between 1 and {}.\n",
                    4
                );
                continue;
            }
        };

        //adds the appropriate meat or marks completion based on user input
        if inp == 1 {
            meats.push(MENU[0].clone());
            println!("\nChicken successfully added!");
        } else if inp == 2 {
            meats.push(MENU[1].clone());
            println!("\nSteak successfully added!");
        } else if inp == 3 {
            meats.push(MENU[2].clone());
            println!("\nShrimp successfully added!")
        } else if inp == 4 {
            if meats.is_empty() {
                meats.push(MenuItem::new("No Meats", 0.0, "Meat"));
            }
            foo = true;
        } else {
            println!("Invalid entry. Please try again.");
        }

        //checks for completion of the loop and adds the price of each meat selected
        if foo == true {
            for meat in &meats {
                price = price + meat.price;
            }
            break;
        }
    }

    //initialize rice type
    foo = false;
    loop {
        println!(
            "\n\nOur rice options are as follows: \n (1) Fried Rice\n (2) Steamed Rice\n (3) No Rice"
        );
        print!("Which would you like? ");
        let mut input = String::new();

        //reads user input
        io::stdout().flush().unwrap();
        stin.read_line(&mut input).unwrap();

        let inp = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "Invalid entry. Please enter a number between 1 and {}.\n",
                    3
                );
                continue;
            }
        };

        //selects the appropriate rice type
        if inp == 1 {
            rice_type = MENU[3].clone();
            foo = true;
        } else if inp == 2 {
            rice_type = MENU[4].clone();
            foo = true;
        } else if inp == 3 {
            foo = true;
        } else {
            println!("Invalid entry. Please try again.");
        }

        //checks for loop completion and adds the price of the rice
        if foo == true {
            println!("\n{} successfully added", rice_type.name);
            price = price + rice_type.price;
            break;
        }
    }

    //initialize topping options
    foo = false;
    loop {
        println!("\n\nOur toppings are as follows: \n (1) Zucchini\n (2) Mushrooms\n (3) Onions \n (4) Beansprouts\n (5) Done with toppings selection");
        print!("Which would you like? ");
        let mut input = String::new();
        io::stdout().flush().unwrap();

        //reads user input
        stin.read_line(&mut input).unwrap();

        let inp = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid entry. Please enter a number between 1 and 5.\n");
                continue;
            }
        };

        //adds appropriate toppings or marks completion based on user input
        if inp == 1 {
            toppings.push(MENU[5].clone());
            println!("Zucchini successfully added");
        } else if inp == 2 {
            toppings.push(MENU[6].clone());
            println!("Onions successfully added");
        } else if inp == 3 {
            toppings.push(MENU[7].clone());
            println!("Mushrooms successfully added");
        } else if inp == 4 {
            toppings.push(MENU[8].clone());
            println!("Beansprouts successfully added");
        } else if inp == 5 {
            if toppings.is_empty() {
                toppings.push(MenuItem::new("No Toppings", 0.0, "Meat"));
            }
            foo = true;
        } else {
            println!("Invalid entry. Please try again.");
        }

        //checks completion of loop and adds price of each topping
        if foo == true {
            for topping in &toppings {
                price = price + topping.price;
            }
            break;
        }
    }

    //initialize sauce
    foo = false;
    loop {
        println!("\n\nOur sauce options are as follows: \n (1) Yum Yum Sauce\n (2) Ginger Sauce\n (3) Sriracha Sauce\n (4) No Sauce");
        print!("Which would you like? ");
        let mut input = String::new();
        io::stdout().flush().unwrap();

        //reads user input
        stin.read_line(&mut input).unwrap();

        let inp = match input.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "Invalid entry. Please enter a number between 1 and {}.\n",
                    4
                );
                continue;
            }
        };

        //selects appropriate sauce option based on user input and marks complete
        if inp == 1 {
            sauce = MENU[9].clone();
            foo = true;
        } else if inp == 2 {
            sauce = MENU[10].clone();
            foo = true;
        } else if inp == 3 {
            sauce = MENU[11].clone();
            foo = true;
        } else if inp == 4 {
            sauce = MenuItem::new("No Sauce", 0.0, "Sauce");
            foo = true;
        } else {
            println!("Invalid entry. Please try again.");
        }

        //checks completion and adds price of sauce
        if foo == true {
            println!("{} successfully added", sauce.name);
            price = price + sauce.price;
            break;
        }
    }

    //checks to see if total price is $0 to create failure meal if so or assemble meal if not
    if price == 0.0 {
        println!("\nEmpty Meal\nFailure to create meal.");
        return Meal::new(
            Vec::new(),
            MenuItem::new("", 0.0, ""),
            Vec::new(),
            MenuItem::new("", 0.0, ""),
            price,
        );
    } else {
        let meal: Meal = Meal::new(meats, rice_type, toppings, sauce, price);
        return meal;
    }
}

//shows the order to the user
fn show_order(order: &Vec<Meal>, done: &bool) -> i8 {
    let mut i: i8 = 0;

    //checks to see if the order is empty
    if order.is_empty() {
        if !done {
            //if attempting to display an empty order
            println!("\nYour order is empty! Add some meals to your order! ");
        } else {
            //if receipt is being printed
            println!("          Notes");
            for _i in (1..10).step_by(1) {
                println!("");
            }
        }
    } else {
        let mut no: u8 = 1;
        if !done {
            println!("\nYour order consists of the following meals: ");
        }

        //prints out each meal
        for meal in order.iter() {
            let mut total: f32 = 0.0;

            //prints the meal number if
            if !done {
                println!("\nMeal {}", no);
            }
            sleep(250);

            //prints the meats
            for meat in &meal.meats {
                print!("{} ", meat.name);
                total = total + meat.price;
            }
            println!("Meal");
            sleep(250);

            //prints the rice
            println!("  {}", meal.rice_type.name);
            sleep(250);
            total = total + meal.rice_type.price;

            //prints the toppings
            for topping in &meal.toppings {
                println!("  {}", topping.name);
                sleep(250);
                total = total + topping.price;
            }

            //prints the sauce
            println!("  {}", meal.sauce.name);
            sleep(250);
            total = total + meal.sauce.price;

            //prints the price
            println!("          Meal Price: ${:.2}\n", total);
            sleep(250);
            i += 1;
            no += 1;
        }
    }

    //returns the number of meals
    return i;
}

//removes a meal from the order
fn remove_order(num: i32) -> i32 {
    loop {
        let stin = io::stdin();
        let mut input = String::new();

        println!("Which order would you like to remove? (Type -1 to cancel) ");
        io::stdout().flush().unwrap();

        //read user input
        stin.read_line(&mut input).unwrap();

        let inp = input.trim();

        //gets appropriate bowl number to return to main function
        let int = match inp.parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "Invalid entry. Please enter a number between 1 and {}.\n",
                    num
                );
                continue;
            }
        };
        return int;
    }
}

//prints receipt
fn print_receipt(order: &Vec<Meal>, done: &bool) {
    //prepares variables for prices
    let mut subtotal: f32 = 0.0;
    let tax: f32;
    let total: f32;

    //prints the top of the receipt
    printlns("\n\n----------------------------------\n");
    printlns("|              Welcome           |");
    printlns("|                 to             |");
    printlns("|             Benji Bo!          |\n");
    printlns("----------------------------------\n");
    printlns("            1357 S. 2468 W. ");
    printlns("         Cathelsbury, AK  96105\n");
    printlns("           1 (777) 579-1357");
    printlns("----------------------------------\n");
    printlns("|             Your order         |");
    printlns("----------------------------------\n");

    //prints the meals
    show_order(order, done);

    //adds the prices of each meal to the subtotal
    for meal in order {
        subtotal = subtotal + meal.total_price;
    }

    //calculates the tax and total
    tax = subtotal * 0.06;
    total = subtotal + tax;

    //prints the rest of the receipt
    printlns("\n----------------------------------\n");
    println!("               Subtotal: ${:.2}", subtotal);
    sleep(250);
    println!("                 6% Tax: ${:.2}", tax);
    sleep(250);
    println!("                  Total: ${:.2}", total);
    printlns("\n----------------------------------\n");
    printlns("             Thank you for");
    printlns("           visiting us today!");
    printlns("\n         Have a wonderful day!");
    printlns("\n----------------------------------");
}

//makes the receipt-like printing much easier
fn printlns(string: &str) {
    println!("{}", string);
    thread::sleep(Duration::from_millis(250))
}
