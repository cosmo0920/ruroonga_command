use plugin_unregister::PluginUnregisterCommand;

pub fn plugin_unregister(name: String) -> PluginUnregisterCommand {
    PluginUnregisterCommand::new(name)
}

#[cfg(test)]
mod test {
    use super::*;
    use plugin_unregister::PluginUnregisterCommand;

    #[test]
    fn test_plugin_unregister() {
        let syntax = plugin_unregister("test_plugin".to_string());
        let actual = PluginUnregisterCommand::new("test_plugin".to_string());
        assert_eq!(syntax, actual);
    }
}
