#[derive(Debug)]
struct User {
    username: String,
    email: String,
    uri: String,
    link: Option<String>,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            link: None,
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
    fn from_email(email: String, uri: String) -> Self {
        let username = String::from(email.split_once('@').unwrap().0);
        Self::new(username, email, uri)
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("alfredodeza"),
        String::from("alfreodeza@example.com"),
        String::from("https://alfredodeza.com"),
    );

    let user2 = User::from_email(
        String::from("max@example.com"), 
        String::from("https://max.com"));

    println!("Hello, {:?}!", new_user.username);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);

    println!("{:?}", user2)
}
