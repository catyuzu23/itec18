use crate::db::structures;
use crate::db::statics;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn get_user_password_hash(
    session: &scylla::client::session::Session,
    user: structures::UserUsername
) -> Option<String> {
    let query_rows = session
        .query_unpaged(statics::SELECT_USER_PASSWORD_HASH, ((user.username),))
        .await.ok()?
        .into_rows_result().ok()?;
    for row in query_rows.rows::<(Option<&str>,)>().ok()?{
        let (password_hash_str,): (Option<&str>,) = row.ok()?;
        match password_hash_str {
            Some(str) => {return Some(str.to_string());},
            _ => {println!("?");return None;}
        };
    }
    None
}


pub async fn insert_new_user(
    session: &scylla::client::session::Session,
    user: structures::User
) -> Option<Result<()>> {
    let query_rows = session.query_unpaged(statics::SELECT_USER_USERNAME, (user.username.clone(),))
        .await.ok()?
        .into_rows_result().ok()?;
    match query_rows.rows::<(Option<&str>,)>() {
        Ok(row) => {
            println!("db::insert_new_user ok(row)");
            if row.rows_remaining() > 0 { return None; }
            else {
                println!("db::insert_new_user else");
                return Some(session
                    .query_unpaged(statics::INSERT_NEW_USER, (user.username, user.password_hash))
                    .await
                    .map(|_|())
                    .map_err(From::from)); 
            }
        },
        _ => {
            println!("db::insert_new_user _");
            return Some(session
                .query_unpaged(statics::INSERT_NEW_USER, (user.username, user.password_hash))
                .await
                .map(|_|())
                .map_err(From::from));
        }
    }
}
