struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("teste@email"),
        username: String::from("teste"),
        active: true,
        sign_in_count: 1,
    };

    let mut color: Color = Color(0, 0, 0);

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
