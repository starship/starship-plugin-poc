#[tarpc::service]
pub trait StarshipService {
    async fn prompt() -> String;
}
