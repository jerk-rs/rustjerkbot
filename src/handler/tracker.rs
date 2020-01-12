use crate::context::Context;
use carapax::{handler, types::Update};
use tokio_postgres::Error as PostgresError;

#[handler]
pub async fn track_chat_member(context: &Context, update: Update) -> Result<(), PostgresError> {
    let user = update.get_user();
    let chat_id = update.get_chat_id();
    if let (Some(user), Some(chat_id)) = (user, chat_id) {
        if context.config.chat_id == chat_id {
            context
                .pg_client
                .execute(
                    "
                        INSERT INTO users
                            (id, first_name, last_name, username)
                        VALUES
                            ($1, $2, $3, $4)
                        ON CONFLICT (id) DO
                        UPDATE SET
                            first_name = $2,
                            last_name = $3,
                            username = $4
                        WHERE users.id = $1;
                    ",
                    &[&user.id, &user.first_name, &user.last_name, &user.username],
                )
                .await?;
        }
    }
    Ok(())
}
