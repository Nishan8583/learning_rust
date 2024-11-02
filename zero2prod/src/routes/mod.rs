/*
mod health_check;: This declares that there is a sub-module named health_check that corresponds to the health_check.rs file in the same directory.
It allows you to access functions, types, and constants declared in health_check.rs under the routes::health_check namespace.

mod subscriptions;: Similarly, this declares a subscriptions sub-module that corresponds to subscription.rs.

pub use health_check::*;: This line publicly re-exports everything (*) defined in health_check.
This means anything defined inside health_check.rs (like functions or structs) can be accessed directly via routes module, e.g., routes::some_function_from_health_check().

pub use subscriptions::*;: This works the same way as the previous line but for the subscriptions module.
*/
mod health_check;
mod subscriptions;

pub use health_check::*;
pub use subscriptions::*;
