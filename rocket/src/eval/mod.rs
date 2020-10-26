
pub mod scope;
pub mod value;

pub use value::Value;
pub use scope::Scope;

pub async fn eval(Vec<>) -> Object {
    let scope = Scope::new();

}
