# Plugins

Skycrane is designed to be extensible. It provides a plugin interface that allows you to add new
functionality to the core application. Plugins are loaded from the `plugins` directory in the
Skycrane config directory. The config directory is located at `~/.config/skycrane`.  

## Plugin Interface

The plugins interface is defined in the [schema](../../assets/spec/plugins.json) that is used to
validate the plugin configuration.  
