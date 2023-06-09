use crate::{model::*, view::*};
use std::{io::Write, str::FromStr};

/// Reads a line from stdin with the given prompt
///
/// This function is similar to the [`input()`](https://docs.python.org/3/library/functions.html#input) function in python.
///
/// The following example asks the use for their name
/// ```rust
/// let line = read_line("Name: ");
/// ```
fn read_line(prompt: &str) -> String {
    print!("{prompt}");
    std::io::stdout().flush().ok();

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).ok();
    line.pop();
    line
}

/// Reads a line from stdin and parses it to type `T` with the given prompt
///
/// If parsing fails the prompt is repeated until a valid value is passed in.
///
/// The following example asks the user for their age
/// ```rust
/// let age: u64 = read_value("Age: ");
/// ```
fn read_value<T: FromStr>(prompt: &str) -> T {
    loop {
        if let Ok(result) = read_line(prompt).parse() {
            break result;
        }
    }
}

/// Display the register user menu.
///
/// Asks the user for their username, password and email and creates the user.
///
/// If creating the user failed an error is printed to stdout.
fn register(user_manager: &mut UserManager) {
    let username = read_line("Username: ");
    let password = read_line("Password: ");
    let email = read_line("Email: ");

    if !user_manager.add_user(username, password, email) {
        println!("Cannot create user.");
    }
}

/// Display the login menu.
///
/// Asks the user for their username and password and searches in the list of usernames and
/// password for a user with these credentials.
///
/// If the user is found, they are logged in and get a new menu,
/// otherwise an error is printed to stdout.
fn login(app: &mut CoronaApplication) {
    let username = read_line("Username : ");
    let password = read_line("Password: ");

    if let Some(user) = app.user_manager.user_login_mut(username, password) {
        logged_in_menu(user, &mut app.catalog, &mut app.order_manager);
    } else {
        println!("Unautherized.");
    }
}

/// Asks user to add item to the catalog.
fn product_add(catalog: &mut Catalog) {
    let code = read_line("Code: ");
    let name = read_line("Name: ");
    let unit_price = read_value("Unit price: ");

    catalog.add_product(Product::new(code, name, unit_price));
}

/// Asks user to remove item from the catalog.
fn product_remove(catalog: &mut Catalog) {
    let code = read_line("Code: ");
    catalog.remove_product(&code);
}

/// Asks user to add item to the cart.
fn cart_add(user: &mut User, catalog: &mut Catalog) {
    let item_index: usize = read_value("Item Index: ");
    if let Some(product) = catalog.products().get(item_index - 1) {
        let quantity = read_value("Quntity: ");
        user.cart_mut().add_item(product, quantity);
        println!("Item added to cart.");
    } else {
        println!("Sorry, there is no item with this index.");
    }
}

/// Asks user to remove item from the cart.
fn cart_remove(user: &mut User) {
    let code = read_line("Code: ");
    user.cart_mut().remove_item(&code);
}

/// Display options to create an order.
fn checkout(user: &mut User, order_manager: &mut OrderManager) {
    let delivery_address = read_line("Delivery address: ");
    order_manager.checkout(user, delivery_address).view();
}

/// Asks user for how to pay and closes order.
fn pay(user: &User, order_manager: &mut OrderManager) {
    let order_id = read_value("Order ID: ");
    if let Some(order) = order_manager
        .orders_mut()
        .iter_mut()
        .find(move |order| order.order_id() == order_id && order.username() == user.username())
    {
        order.view();
        let total_price = order.total_price();
        let payment = match read_line("Payment method: ").as_str() {
            "cash" | "pay on delivery" => {
                let amount: f64 = read_value("Amount: ");
                if amount < total_price {
                    println!("Sorry, not enough money.");
                    return;
                }
                if amount > total_price {
                    println!("Return: {:.2} EGP", amount - total_price);
                }
                OrderPayment::Cash
            }
            "credit" | "credit card" => {
                let card_number = read_line("Card number: ");
                if card_number.len() != 16 {
                    println!("Sorry, card number invalid.");
                    return;
                };

                let amount: f64 = read_value("Amount in card: ");
                if amount < total_price {
                    println!("Sorry, not enough money in card.");
                    return;
                }

                OrderPayment::CreditCard { card_number }
            }
            _ => {
                println!("This payment method is not available. Aborting.");
                return;
            }
        };
        if order.close(payment) {
            println!("Order payed successfully.");
        } else {
            println!("Order already closed.");
        }
    } else {
        println!("Order not found. Aborting.");
    }
}

/// Lists orders for current user.
fn list_orders_for_user(order_manager: &OrderManager, user: &User) {
    order_manager
        .orders()
        .iter()
        .filter(|order| order.username() == user.username())
        .for_each(View::view);
}

/// Menu for logged in users.
fn logged_in_menu(user: &mut User, catalog: &mut Catalog, order_manager: &mut OrderManager) {
    let prompt = format!("({}) >>> ", user.username());
    loop {
        match read_line(&prompt).as_str() {
            "product add" if user.is_admin() => product_add(catalog),
            "product remove" if user.is_admin() => product_remove(catalog),
            "product list" | "product ls" | "catalog" | "products" => catalog.view(),
            "cart add" | "add" => cart_add(user, catalog),
            "cart remove" => cart_remove(user),
            "cart list" | "cart ls" | "cart" => user.cart().view(),
            "order list" | "order ls" | "orders" if user.is_admin() => order_manager.view(),
            "order list" | "order ls" | "orders" => list_orders_for_user(order_manager, user),
            "order" | "checkout" => checkout(user, order_manager),
            "pay" => pay(user, order_manager),
            "q" | "quit" | "exit" | "logout" => break,
            "" => {}
            _ => {
                println!("I don't understand what you are saying!!!");
            }
        }
    }
}

/// Menu for users not logged in.
pub(crate) fn main(app: &mut CoronaApplication) {
    loop {
        match read_line(">>> ").as_str() {
            "register" => register(&mut app.user_manager),
            "login" => login(app),
            "save" => {
                if app.save().is_none() {
                    println!("Failed to save.");
                }
            }
            "q" | "quit" | "exit" => break,
            "" => {}
            _ => {
                println!("I don't understand what you are saying!!!")
            }
        }
    }
}
