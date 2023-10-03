#[allow(dead_code)]
struct User {
    username: String,
}

// User traits
trait Registered {
    fn is_registered(&self) -> bool;
}

trait SignedIn {
    fn is_signed_in(&self) -> bool;
}

trait Authenticated {
    fn is_authenticated(&self) -> bool;
}

// Trait implementations | User
impl Registered for User {
    fn is_registered(&self) -> bool {
        true
    }
}

impl SignedIn for User {
    fn is_signed_in(&self) -> bool {
        println!(r#"
            Sign In successful!
            Wlecome @{}!

        "#, self.username
        );
        true
    }
}

impl Authenticated for User {
    fn is_authenticated(&self) -> bool {
        println!("> {} authenticated!\n", self.username.trim());
        true
    }
}

trait Authorized: SignedIn + Authenticated {}

impl Authorized for User {}

fn authorize(auth: &dyn Authorized) -> bool {
    auth.is_signed_in();
    auth.is_authenticated();
    true
}

fn main() {
    let user: User = User { username: "johndoe".to_string() };

    if user.is_registered() {
        println!("\n> Registration successful!\n");
    }

    authorize(&user);
}