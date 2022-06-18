# VRS Documentation

## Configuring the web server

### Where is the configuration file?

There are many useful "settings" VRS provides in order to customize the web server for your need. All the useful configuration options for VRS can be found in the configuration.rs file inside src.

### Are the default configurations safe?

By default, we have set the configuration to be production-ready so that you do not have to tinker with the settings a lot. Do not be a fraid to leave the configuration as it is, it is secure enough by default. Certain configurations such as SECURITY_HEADERS should only be turned off if you know what you are doing.

### Configurations in configuration.rs explained

- ABSOLUTE_STATIC_CONTENT_PATH

You should provide this variable the absolute path (/absolute/path/to/static/) which should contain all the static files you went the web server to serve. The web server will take care of serving the files, all you need to know is that you need to put the same path inside the $STATIC variable in setup.sh.

- ABSOLUTE_LOGS_PATH

The absolute path to which the server request logs should be saved (/absolute/path/to/logs/). Note that this can be empty if MULTITHREADING or SAVE_LOGS is enabled because the server doesn't save logs when either are enabled.

- ADDR

The IPv4 the server should use (e.g. 0.0.0.0 or 127.0.0.1). Change it to 127.0.0.1 if you want to only be able to access the web server locally.

- PORT

The port that will be used by the server. The default port for HTTP is 80, and for HTTPS 443.

- SAVE_LOGS

Boolean for specifying whether you want the server to save request logs or not.

- MULTITHREADING

Specify whether you want the server to use multiple threads (workers) or not. Logs aren't supported when using this functionality.

- HTTP_PROTOCOL_VERSION

Enum for specifying whether you want to use HTTP/1.1 or HTTP/2. Note that for API clients such as Postman HTTP/2 is not supported.

- ALLOW_ALL_ORIGINS (CORS)

Specify whether you want to allow external web servers (outside your local network) to fetch data from VRS. There shouldn't be any problem with keeping this on.

- ALLOWED_ORIGINS (CORS)

If you don't want to allow all origins as explained above, you may also just allow specific origins to fetch data from the server. If you want to block all origins, set ALLOW_ALL_ORIGINS to false and leave the ALLOWED_ORIGINS array empty.

- ALLOWED_METHODS

Use to specify which HTTP methods you wish to allow. Must contain atleast one. Most of the time you only need the GET method.

- SECURITY_HEADERS

You should not turn this off. When enabled requests are sent some additional HTTP headers in order to prevent common attacks such as clickjacking.

- ALLOW_IFRAMES

Enable if you want to allow other web apps to embed your website inside them. May make common attacks possible, not recommended to enable unless necessary.
