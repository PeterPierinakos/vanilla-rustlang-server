# VRS Documentation

## Configuring the web server

### Where is the configuration file?

There are many useful "settings" VRS provides in order to customize the web server for your need. All the useful configuration options for VRS can be found in the configuration.rs file inside src.

### Are the default configurations safe?

By default, we have set the configuration to be production-ready so that you do not have to tinker with the settings a lot. Do not be a fraid to leave the configuration as it is, it is secure enough by default. It is recommended to only tinker with the necessary options. Certain configurations such as <code>USE\_SECURITY\_HEADERS</code> should only be turned off if you know what you are doing.

### Configurations in configuration.rs explained

- ABSOLUTE\_STATIC\_CONTENT\_PATH

You should provide this variable the absolute path (/absolute/path/to/static/) which should contain all the static files you went the web server to serve. The web server will take care of serving the files, all you need to know is that you need to put the same path inside the $STATIC variable in setup.sh.

- ABSOLUTE\_LOGS\_PATH

The absolute path to which the server request logs should be saved (/absolute/path/to/logs/). Note that this setting can be ignored if <code>MULTITHREADING</code> is enabled because the server doesn't save logs outside of singlethreaded mode..

- ADDR

The IPv4 the server should use (e.g. 0.0.0.0 or 127.0.0.1). Change it to 127.0.0.1 if you want to only be able to access the web server locally.

- PORT

The port that will be used by the server. The default port for HTTP is 80, and for HTTPS 443.

- SAVE\_LOGS

Boolean for specifying whether you want the server to save request logs or not.

- MULTITHREADING

Specify whether you want the server to use multiple threads (workers) or not. Logs aren't supported when using this functionality.

- NUM\_OF\_THREADS

If MULTITHREADING is enabled, you may enter a valid (unsigned integer) number greater than 0 in order to specify the number of threads the server should cap at. If you specify 0, the server will panic because that is an invalid number of threads.

- HTTP\_PROTOCOL\_VERSION

Enum for specifying whether you want to use HTTP/1.1 or HTTP/2. Note that for API clients such as Postman HTTP/2 is not supported.

- ALLOW\_ALL\_ORIGINS (CORS)

Specify whether you want to allow external web servers (outside your local network) to fetch data from VRS. There shouldn't be any problem with keeping this on.

- ALLOWED\_ORIGINS (CORS)

If you don't want to allow all origins as explained above, you may also just allow specific origins to fetch data from the server. If you want to block all origins, set <code>ALLOW\_ALL\_ORIGINS</code> to false and leave the <code>ALLOWED\_ORIGINS</code> array empty.

- ALLOWED\_METHODS

Use to specify which HTTP methods you wish to allow. Must contain atleast one. Most of the time you only need the GET method.

- USE\_SECURITY\_HEADERS

You should not turn this off. When enabled requests are sent some additional HTTP headers in order to prevent common attacks such as clickjacking.

- ALLOW\_IFRAMES

Enable if you want to allow other web apps to embed your website inside them. May make common attacks possible, not recommended to enable unless necessary.

- APPEND\_EXTRA\_HEADERS

If you wish to apply the <code>EXTRA\_HEADERS</code> to all of the server's responses, set this boolean to true.

- EXTRA\_HEADERS

Additional headers you can specify which will be applied to every server response as long as APPEND\_EXTRA\_HEADERS is set to true.

- EXTRA\_HEADERS\_SIZE

Convenience variable for specifying the size of the <code>EXTRA\_HEADERS</code> 2-dimensional array. You may remove it and hard-code the number if you wish to.

- ALLOW\_DIRECTORY\_LISTING

Boolean value to specify whether you want the server to allow listing the contents of a directory whenever a user tries to access a directory and not particularly a file.

- FORMAT\_DIRECTORY\_LISTING\_AS\_JSON

Boolean value to specify whether the returned directory's contents should be JSON. If it is set to false, it will return HTML. Can be ignored if <code>ALLOW\_DIRECTORY\_LISTING</code> is set to false.

- PRINT\_LICENSE\_INFO\_AT\_START

Boolean value which should be set to false if you wish to not print license information for the software upon starting. Note that keeping it enabled is the least you can do to show respects to the contributors who spend their time on improving this software.

- USE\_TIME\_HEADER

The "Time" header is a special HTTP header used by the back-end in order to tell the client when the HTTP request was processed. Disable if you do not need this extra functionality.

- CACHE\_FILES

If enabled, the web server will cache previously requested files from the clients by storing them into the computer's memory and will display them instead of doing a filesystem operation on every request which is costly. Can double the performance on extremely large files, but in most cases it shouldn't make that much of a difference. **If enabled, once a specific file has been requested its contents will never be updated after being changed as it is stored inside RAM after the request.**
