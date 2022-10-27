> As of v1.2.3 the configuration documentation has been moved to the configuration file. Read the comments in `configuration.rs` in order to understand how they work.

# VRS Documentation

## Configuring the web server

### Where is the configuration file?

There are many useful "settings" VRS provides in order to customize the web server for your need. All the useful configuration options for VRS can be found in the configuration.rs file inside src.

### Are the default configurations safe?

By default, we have set the configuration to be production-ready so that you do not have to tinker with the settings a lot. Do not be a fraid to leave the configuration as it is, it is secure enough by default. It is recommended to only tinker with the necessary options. Certain configurations such as <code>USE\_SECURITY\_HEADERS</code> should only be turned off if you know what you are doing.
