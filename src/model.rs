use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct Product {
    code: String,
    name: String,
    #[serde(rename = "price")]
    unit_price: f64,
}

impl Product {
    pub(crate) fn new(code: String, name: String, unit_price: f64) -> Self {
        Self {
            code,
            name,
            unit_price,
        }
    }

    pub(crate) fn code(&self) -> &str {
        self.code.as_ref()
    }

    pub(crate) fn name(&self) -> &str {
        self.name.as_ref()
    }

    pub(crate) fn unit_price(&self) -> f64 {
        self.unit_price
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct OrderItem {
    #[serde(flatten)]
    product: Product,
    quantity: f64,
}

impl OrderItem {
    pub(crate) fn name(&self) -> &str {
        self.product.name()
    }

    pub(crate) fn code(&self) -> &str {
        self.product.code()
    }

    pub(crate) fn quantity(&self) -> f64 {
        self.quantity
    }

    pub(crate) fn unit_price(&self) -> f64 {
        self.product.unit_price()
    }

    pub(crate) fn total_price(&self) -> f64 {
        self.quantity * self.product.unit_price()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum CardStatus {
    Valid,
    Expired,
    Invalid,
    InsufficintFunds,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "payment_method", content = "payment")]
pub(crate) enum OrderPayment {
    Cash,
    CreditCard { card_number: String },
}

impl std::fmt::Display for OrderPayment {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Cash => {
                f.write_str("cash")?;
            }
            Self::CreditCard { card_number, .. } => {
                f.write_str("credit card ")?;
                f.write_str(&card_number)?;
            }
        };
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "order_state", content = "state")]
pub(crate) enum OrderState {
    Open,
    Closed { payment: OrderPayment },
}

impl std::fmt::Display for OrderState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Open => f.write_str("open"),
            Self::Closed { .. } => f.write_str("closed"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Order {
    order_id: u64,
    username: String,
    items: Vec<OrderItem>,
    delivery_address: String,
    state: OrderState,
}

impl Order {
    pub(crate) fn order_id(&self) -> u64 {
        self.order_id
    }

    pub(crate) fn username(&self) -> &str {
        self.username.as_ref()
    }

    pub(crate) fn items(&self) -> &[OrderItem] {
        self.items.as_ref()
    }

    pub(crate) fn delivery_address(&self) -> &str {
        self.delivery_address.as_ref()
    }

    pub(crate) fn state(&self) -> &OrderState {
        &self.state
    }

    pub(crate) fn total_price(&self) -> f64 {
        self.items
            .iter()
            .map(|item| item.product.unit_price * item.quantity)
            .sum()
    }

    pub(crate) fn close(&mut self, payment: OrderPayment) -> bool {
        if let OrderState::Open = self.state {
            self.state = OrderState::Closed { payment };
            true
        } else {
            false
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Cart(Vec<OrderItem>);

impl Cart {
    pub(crate) fn iter(&self) -> std::slice::Iter<'_, OrderItem> {
        self.0.iter()
    }

    pub(crate) fn add_item(&mut self, product: &Product, quantity: f64) {
        if let Some(cart_item) = self
            .0
            .iter_mut()
            .find(|cart_item| cart_item.product.code == product.code)
        {
            cart_item.quantity += quantity;
        } else {
            self.0.push(OrderItem {
                product: product.clone(),
                quantity,
            })
        }
    }

    pub(crate) fn remove_item(&mut self, code: &str) {
        self.0.retain(|item| item.product.code != code);
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct User {
    username: String,
    password_hash: String,
    email: String,

    cart: Cart,
}

impl User {
    pub(crate) fn username(&self) -> &str {
        self.username.as_ref()
    }

    pub(crate) fn email(&self) -> &str {
        self.email.as_ref()
    }

    pub(crate) fn cart(&self) -> &Cart {
        &self.cart
    }

    pub(crate) fn cart_mut(&mut self) -> &mut Cart {
        &mut self.cart
    }

    pub(crate) fn is_admin(&self) -> bool {
        self.username == "admin"
    }
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct UserManager {
    users: Vec<User>,

    #[serde(skip)]
    usernames_taken: std::collections::HashSet<String>,
}

impl UserManager {
    pub fn add_user(&mut self, username: String, password: String, email: String) -> bool {
        if !self.usernames_taken.insert(username.clone()) {
            return false;
        }

        let password_hash = bcrypt::hash(password, 4).unwrap();

        self.users.push(User {
            username,
            password_hash,
            email,

            cart: Default::default(),
        });

        true
    }

    pub(crate) fn user_login_mut(
        &mut self,
        username: String,
        password: String,
    ) -> Option<&mut User> {
        self.users.iter_mut().find(|u| {
            u.username == username && bcrypt::verify(&password, &u.password_hash).unwrap()
        })
    }
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct Catalog {
    products: Vec<Product>,
}

impl Catalog {
    pub(crate) fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    pub(crate) fn remove_product(&mut self, code: &str) {
        self.products.retain(|product| product.code != code);
    }

    pub(crate) fn products(&self) -> &[Product] {
        self.products.as_ref()
    }
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct OrderManager {
    orders: Vec<Order>,
    sequence_id: u64,
}

impl OrderManager {
    pub(crate) fn checkout(&mut self, user: &mut User, delivery_address: String) -> &Order {
        let order_id = self.sequence_id;
        self.sequence_id += 1;

        self.orders.push(Order {
            order_id,
            username: user.username.clone(),
            items: std::mem::take(&mut user.cart.0),
            delivery_address,
            state: OrderState::Open,
        });

        self.orders.last().unwrap()
    }

    pub(crate) fn orders(&self) -> &[Order] {
        &self.orders
    }

    pub(crate) fn orders_mut(&mut self) -> &mut [Order] {
        &mut self.orders
    }
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct CoronaApplication {
    #[serde(flatten)]
    pub user_manager: UserManager,
    #[serde(flatten)]
    pub catalog: Catalog,
    #[serde(flatten)]
    pub order_manager: OrderManager,
}

impl CoronaApplication {
    const PATH: &str = "corona.toml";

    pub fn save(&self) -> Option<()> {
        toml::to_string(self)
            .ok()
            .and_then(|s| std::fs::write(Self::PATH, s).ok())
    }

    pub fn load() -> CoronaApplication {
        std::fs::read_to_string(Self::PATH)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default()
    }
}
