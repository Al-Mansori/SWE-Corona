use crate::model::*;

pub(crate) trait View {
    fn view(&self);
}

impl View for Catalog {
    fn view(&self) {
        println!("Catalog:");
        self.products().iter().enumerate().for_each(|(i, product)| {
            let idx = i + 1;
            let code = product.code();
            let name = product.name();
            let unit_price = product.unit_price();
            println!("{idx:>3}. [{code}] {name} - {unit_price:.2} EGP")
        });
    }
}

impl View for Cart {
    fn view(&self) {
        println!("There are {} item(s) in the cart:", self.iter().len());
        for item in self.iter() {
            println!("{}x {}", item.quantity(), item.name())
        }

        let total_cost: f64 = self.iter().map(|item| item.total_price()).sum();

        println!("Total cost: {total_cost:.2}");
    }
}

impl View for Order {
    fn view(&self) {
        println!("Order #{}", self.order_id());
        println!("  for user: {}", self.username());
        println!("  deliver to: {}", self.delivery_address());
        println!("  costs: {:.2} EGP", self.total_price());
        println!("  state: {}", self.state());
        if let OrderState::Closed { payment } = self.state() {
            println!("  pay by: {}", payment);
        }
        println!("  items:");
        for item in self.items() {
            println!(
                "  - {}x {} [{}] = {:.2} EGP",
                item.quantity(),
                item.name(),
                item.code(),
                item.total_price(),
            );
        }
    }
}

impl View for OrderManager {
    fn view(&self) {
        self.orders().iter().for_each(View::view);
    }
}
