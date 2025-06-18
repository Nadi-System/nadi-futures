
use nadi_core::nadi_plugin::nadi_plugin;

#[nadi_plugin]
mod nadi_futures {
    use nadi_core::prelude::*;

    /// The macros imported from nadi_plugin read the rust function you
    /// write and use that as a base to write more core internally that
    /// will be compiled into the shared libraries. This means it'll
    /// automatically get the argument types, documentation, mutability,
    /// etc. For more details on what they can do, refer to nadi book.
    use nadi_core::nadi_plugin::{node_func, network_func, env_func};

    /// Example Environment function for the plugin
    ///
    /// You can use markdown format to write detailed documentation for the
    /// function you write. This will be availble from nadi-help.
    #[env_func(pre = "Message: ")]
    fn echo(message: String, pre: String) -> String {
         format!("{}{}", pre, message)
    }

    /// Example Node function for the plugin
    #[node_func]
    fn node_name(node: &NodeInner) -> String {
         node.name().to_string()
    }

    /// Example Network function for the plugin
    ///
    /// You can also write docstrings for the arguments, this syntax is not
    /// a valid rust syntax, but our macro will read those docstrings, saves
    /// it and then removes it so that rust does not get confused. This means
    /// You do not have to write separate documentation for functions.
    #[network_func]
    fn node_first_with_attr(
        net: &Network,
        /// Name of the attribute to search
        attrname: String,
    ) -> Option<String> {
         for node in net.nodes() {
             let node = node.lock();
             if node.attr_dot(&attrname).is_ok() {
                 return Some(node.name().to_string())
             }
        }
        None
    }
}
