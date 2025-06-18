use nadi_core::nadi_plugin::nadi_plugin;

#[nadi_plugin]
mod nadi_futures {
    use nadi_core::nadi_plugin::{env_func, network_func, node_func};
    use nadi_core::prelude::*;
    use std::collections::HashSet;

    /// Concat strings
    #[env_func(sep = "")]
    fn join(values: &[String], sep: &str) -> String {
        values.join(sep)
    }

    /// all attributes from the node
    #[node_func]
    fn ls(node: &NodeInner) -> Vec<String> {
        node.attr_map().keys().map(|k| k.to_string()).collect()
    }

    /// All attribute names present in all the nodes
    #[network_func(ignore=vec![])]
    fn ls(net: &Network, ignore: Vec<String>) -> Vec<String> {
        let mut attrs = HashSet::new();
        for node in net.nodes() {
            node.lock()
                .attr_map()
                .keys()
                .filter(|k| !ignore.contains(&k.to_string()))
                .for_each(|k| {
                    attrs.insert(k.to_string());
                })
        }
        attrs.into_iter().collect()
    }
}
