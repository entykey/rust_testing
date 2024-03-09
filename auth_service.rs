use sqlx::{PgPool, query_as};

#[derive(sqlx::FromRow)]
struct ApplicationUser {
    id: String,
    user_name: String,
    full_name: String,
    email: String,
    password_hash: String,
}

async fn fetch_user_data(pool: &PgPool, user_name_or_email_or_phone: &str) -> Result<ApplicationUser, sqlx::Error> {
    let is_email = regex::Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}$")
        .unwrap()
        .is_match(user_name_or_email_or_phone);

    let is_phone_number = user_name_or_email_or_phone.chars().all(|c| c.is_digit(10));

    let mut query = "SELECT Id, UserName, FullName, Email, PasswordHash, EmailConfirmed FROM Users WHERE ".to_string();

    if is_email {
        query.push_str("Email = $1");
    } else if is_phone_number {
        query.push_str("PhoneNumber = $1");
    } else {
        query.push_str("UserName = $1");
    }

    let user: ApplicationUser = sqlx::query_as(&query)
        .bind(user_name_or_email_or_phone)
        .fetch_one(pool)
        .await?;

    Ok(user)
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = sqlx::PgPool::connect("your_database_url").await?;

    let user_name_or_email_or_phone = "user@example.com"; // Replace with the actual input

    let user = fetch_user_data(&pool, user_name_or_email_or_phone).await?;
    
    println!("User: {:?}", user);

    Ok(())
}