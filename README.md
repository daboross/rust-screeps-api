rust-screeps-api
================

Provides a wrapper in rust of the https://screeps.com API, using hyper for making requests and serde_json for interpreting results.

The API provided for screeps.com is unofficial, and not officially documented, but the developers of screeps have not been hostile in the past to those who use it. This API, and rust-screeps-api, can also be used to access Screeps Private Servers, provided the `screepsmod-auth` mod has been installed on them to provide non-steam-based authentication.

Documentation for the API calls that rust-screeps-api makes can be found at https://github.com/screepers/python-screeps/blob/master/docs/Endpoints.md.

When this library is relatively stable, I will post the rust docs online and provide some example usages. For now, the library does not do enough to warrant this.

To run tests, provide SCREEPS_API_USERNAME and SCREEPS_API_PASSWORD in .env or as environmental variables. These variables are only ever used when running tests, and are never included in the compiled library.
