## Server-Side Proxy Boilerplate
Looking for a simple example of a Rust server-side proxy with CORS included? Look no further!

This project is built on Actix-web and includes actix-cors with placeholder variables to get you up and running ASAP.

It even removes the typical hassle of OpenSSL (in)compatibility by solely focusing on Rustls!

### Running the Project
A main focus was cross-compatibility, but this boilerplate was initially built to run on the free, private cloud hosting service, https://deta.space

Because of this, includes a Spacefile config to easily launch this on Space.

With the Space CLI installed, clone this repository and type ```space new```.

### Configuration
There are two included domains listed within the CORS policy as placeholders.

Be sure to update them at the bottom of the ```main.rs``` file with your specific ports, ngrok addresses, or domains.

### Example Requests
There are three included endpoints:
- GET request
- GET request w/ query parameters
- POST request w/ expected data

#### GET Request
Assuming the project is running locally, visit:

```http://localhost:8080/hello```

This will respond with a ```hello world``` message.

#### GET Request w/ Query Parameters
Assuming the project is running locally, visit:

```http://localhost:8080/query?content=blahblahblah```

The url expects the parameter ```content``` and will include the value with the result.

#### POST Request w/ Expected Data
Assuming the project is running locally, visit:

```http://localhost:8080/capture```

This will expect a key called ```email```, the value can be anything.

A successful response will echo what was sent.
