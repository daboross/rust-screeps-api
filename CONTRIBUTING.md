Contributing to screeps-api
===========================

This is a fairly barebones document. If you have any questions, please file an
issue in the repository to ask them!

# Some protocol update procedures

## Fixing an error you've found

If you are using this library, and you get either a logged warning or error
about missing fields, you've come to the right place! The libary is designed to
be fairly easy to update.

First, you should get your error message - it'll contain JSON of what was
received, and some note for what struct is wrong.

Second, find the struct in this repository, and look at its structure. Then,
depending on your error, either make some of the fields optional by wrapping the
type in an `Option<>`, or add new fields, or if you believe an update removed a
field, remove it.

Third, compile your changes - run `cargo check --tests`, and see what breaks.
Keep going around fixing compilation errors until it works.

Fourth, run the tests - `cargo test`. If any fail, see if its the fault of your
changes. IF the tests are now wrong because they contain old protocol data, and
the server no longer responds with that data, follow the "Updating an old test"
procedure listed below.

Last, make a new test using the JSON in the warning message. The crate should
now support it, so you should make a new unit test under whatever structure you
edited which validates that it does. Everything should have existing unit tests,
so there should be one you can copy/paste and then stick your data into. The
only thing to know when doing this is that the `json!()` macro sometimes
requires you to suffix your numbers with rust number suffixes - so you'll need
to modify the json `"key": 314` into `"key": 314u64` when copying it in (or
`f64` if a float).

## Updating an old test

When adapting to protocol changes, it's likely that old tests will completely
break. When this happens, I prefer to _completely redo the test_ rather than try
to fix it piecemail. Ideally, every test contains some data the current server
sent you - and if you simply modify bits of the test, then this guarantee will
be broken. It's easy to write tests which work for your code (but don't
actually represent the server), so instead, we should grab data from the server.

Luckily, we have some tools to do just that! This crate comes with a few
examples, and the most important one is `ws-debug`. Once you get the crate
compiling, even if tests fail, you can run `ws-debug` to observe and grab raw
JSON data from any room on the screeps server.

So - say you've just updated `StructureStorage`, and its old test fails. First,
go onto the screeps web interface, and find a room with a storage in it which
exhibits the properties you're looking to test (if it's just the
`parse_storage` test, this is literally any storage).

Note the room number and shard, and then run `ws-debug`:

```
cargo run --example ws-debug -- --room W39N49 --shard shard0 -v
```

This requires authentication. If you haven't already, go to
https://screeps.com/a/#!/account/auth-tokens to create a new authentication
token, and then stick it into a new `.env` file in this repository like so:

```
# this is .env
SCREEPS_API_TOKEN='your-token-here'
```

The `-v` flag here is required to actually print out the raw JSOn. If ommitted,
`ws-debug` will only output warnings/errors, but not successful messages. That
can be useful, for example, if you're updating to include a new feature and want
to see what fails when reading a room with it in it.


Anyways, if that works, you'll connect to the server, and `ws-debug` will start
spitting out room updates for your room.

Find the part of the update representing your structure. For example, for
`StructureStorage`, you'd look for something like this:

```json
  "599ca7f2e48c09254b443791": {
    "_id": "599ca7f2e48c09254b443791",
    "hits": 10000,
    "hitsMax": 10000,
    "notifyWhenAttacked": true,
    "room": "W39N49",
    "store": {
      "energy": 913026
    },
    "storeCapacity": 1000000,
    "type": "storage",
    "user": "5788389e3fd9069e6b546e2d",
    "x": 6,
    "y": 13
  },
```

Finally, copy this data into the failing test, and update the test's assertions
to match the new coordinates/inventory/etc. You'll probably want to reformat the
json data as well, so it looks like what the test did before.
