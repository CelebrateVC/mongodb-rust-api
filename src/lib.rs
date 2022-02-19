pub mod server;
pub mod api;
pub const APPLICATION_JSON: &str = "application/json";
pub mod connect;
pub use connect::mongo_connection;
pub use mongodb;
#[cfg(test)]
mod tests;