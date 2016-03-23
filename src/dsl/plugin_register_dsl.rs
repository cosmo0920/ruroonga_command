use plugin_register::PluginRegisterCommand;

pub fn plugin_register(name: String) -> PluginRegisterCommand {
    PluginRegisterCommand::new(name)
}

#[cfg(test)]
mod test {
    use super::*;
    use plugin_register::PluginRegisterCommand;

    #[test]
    fn test_plugin_register() {
        let syntax = plugin_register("test_plugin".to_string());
        let actual = PluginRegisterCommand::new("test_plugin".to_string());
        assert_eq!(syntax, actual);
    }
}
