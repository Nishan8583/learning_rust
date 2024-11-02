/*
The lib.rs file acts as the root of your library. It typically organizes the structure of the entire crate and exposes different modules.


pub mod configuration;: This declares that there is a configuration module, corresponding to configuration.rs. It is publicly accessible to other parts of your program or crate.

pub mod routes;: This declares the routes module (which we discussed above). Since mod.rs inside the routes directory organizes sub-modules, anything in routes will be structured and accessible.

pub mod startup;: Similarly, this exposes the startup module, corresponding to startup.rs.
*/
pub mod configuration;
pub mod routes;
pub mod startup;
pub mod telemetry;
