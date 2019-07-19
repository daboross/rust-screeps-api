Websocket Protocol Documentation!
=====

Initial Connection Procedure:

- First, obtain an API token via http endpoints.

  Alternatively: obtain an API token via the dedicated API auth page now added

- Connect to the websocket via SockJS or normal WS client.
  - SockJS: connect to `wss://screeps.com/socket/`
  - WS: connect to `wss://screeps.com/socket/<4 integers>/<8 ascii chars a-z0-5>/websocket`
- Perform SockJS initial handshake (I think this just involves receiving an 'open' message)
- Send 'auth API_TOKEN_HERE' message (full format below)
- Receive auth result
- Subscribe to channels
- Receive results from channels and/or subscribe/unsubscribe to channels, etc.

SockJS shim / small details
====

if you don't have a sockjs libary, the format is roughly:
```text
o
    open message, sent once
h
    heartbeat
c[AN_INT, "A_REASON"]
    close
m"THE_INNER_MESSAGE"
    send a single message, THE_INNER_MESSAGE. this is what we actually care about
a["MSG1", "MSG2"]
    like 'm...', but for multiple messages. this can also include only one message
```

Screeps Message Details
=====

Each "THE_INNER_MESSAGE" is a screeps message. It's a string in one of the following formats:

```text
time NOW_TIME               | here's the current time on the server
protocol PROTOCOL_VERSION   | not sure what this represents. protocol version is an integer
package PACKAGE_VESION      | not sure about this either. package version is an integer
auth ok NEW_TOKEN_HERE      | authentication success! here's a new token to use
auth failed                 | authentication failed!
["CHANNEL_NAME", MSG]       | actual message! channel name is a string, MSG is still JSON
```

Examples of each of these:

```text
time 1519081464885885
protocol 13
package 130
auth ok 1143aacawefwa       # (auth token is longer, rest hidden)
auth failed
["user:57874d42d0ae911e3bd15bbc/cpu",{"cpu":0,"memory":754}]
```

Sending messages
====

Before we get to channels, there's one more thing: how to send stuff back.

The three commands you'll need are authentication, subscribe, and unsubscribe.

Each command is a string message, serialized as a JSON array with one element. At least, that's what the official client sends, and it works. One might be able to just send the inner message without the JSON serialization if using a sockjs library rather than just websockets.

Here are the three formats, then:

```text
auth AUTH_TOKEN_HERE
subscribe CHANNEL_NAME
unsubscribe CHANNEL_NAME
```

Some examples, fully wrapped for sending to a raw websocket. Each of these lines would be sent to the raw websocket.

```text
["auth 1143aacb-70e8-42c9-a956-30e07c5575a4"]
["subscribe server-message"]
["subscribe user:57874d42d0ae911e3bd15bbc/cpu"]
["unsubscribe user:57874d42d0ae911e3bd15bbc/money"]
```

Known Channels
===

Channels are how the websocket connection works: you subscribe to a number of them, and then the server sends you updates for each one for each tick. Each channel has a unique name which you use to subscribe, unsubscribe, and identify messages.

Here are known channel names:

```text
server-message                  | server announce messages alert
user:{user id}/cpu              | user CPU / memory usage every tick
user:{user id}/newMessage       | new message sent to user alert
user:{user id}/message:{user2}  | new message sent to user from user id user2
user:{user id}/memory/{p}       | updates on memory path 'p'
user:{user id}/console          | updates with console messages every tick
user:{user id}/set-active-branch| sends an update whenever the active branch changes
roomMap2:{shard}/{room}         | map-level low-detail updates for a room in a shard every tick
roomMap2:{room}                 | ^^ for servers without shard support
room:{shard}/{room}             | full-detail updates on a room every tick, incremental updates
room:{room}                     | ^^ for servers without shard support
```

TODO: I *think* there's a path for memory segment updates too, but haven't searched for it yet.


Update Format
===

After you've subscribed to a channel by sending a subscribe message, you'll receive updates either every tick, or when something happens. Most are every tick.

Here's an example of a channel update with different levels of parsing:

```text
# the raw websocket data:
a["[\"user:57874d42d0ae911e3bd15bbc/cpu\",{\"cpu\":0,\"memory\":754}]"]
# the sockJS 'frame':
["user:57874d42d0ae911e3bd15bbc/cpu",{"cpu":0,"memory":754}]
# the channel name:
user:57874d42d0ae911e3bd15bbc/cpu
# the update:
{"cpu":0,"memory":754}
```

The channel name will always be one you subscribed too, with one exception. The channel `err@room:...` is used instead of `room:...` when a room update is skipped because of rate limiting on the number of connected rooms allowed.

Now: different types!

```text
server-message
    unknown format (never sent/received while watching)
user:{}/cpu
    {
        "cpu": integer, CPU used last tick
        "memory": integer, bytes serialized Memory took up last tick
    }
user:{}/console
    two variations. first:
        {
            "messages": {
                "log": [
                    "a console message",
                    "another console message",
                    ...
                ],
                "results": [
                    "the result of the console command you typed",
                    ...
                ]
                # arrays are present but empty if no messages of the given type occurred last tick.
            },
            "shard": "shard name here" # not present on servers without shards
        }
    second variation:
        {
            "error": "code error which occurred last tick"
            "shard": "shard name here" # not present on servers without shards
        }
    the first variation will be sent every tick, even if no messages happened. the second will be
    sent if the user code exited in an error. It is an additional, separate message from the
    first, and does not replace it.
roomMap2:{}/{}
roomMap2:{}
    {
        "w": [...],     # walls
        "r": [...],     # roads
        "pb": [...],    # power banks or power
        "p": [...],     # portals
        "s": [...],     # sources
        "m": [...],     # minerals
        "c": [...],     # controllers
        "k": [...],     # keeper lairs
        "<user id>": [...]  # structures and creeps belonging to <user id> user
        # 0-inf user id entries allowed
    }
    # each [...] is a list of positions, like `[[0, 2], [40,30]]` to represent the thing being at
    # (x=0, y=2) and (x=40, y=30). The lists are empty if there were no things of that type present
    # last tick. "user" entries only appear if a user is present in the room, and are distinguished
    # from the others by not being one of the 8 known keys.

# TODO: message-type updates

# TODO: room:{}/{} updates

```

room updates
====

This has its own section, as it is a complicated format. Each tick, you'll receive an update on either `room:{shard name}/{room name}` or `err@room:{shard name}/{room name}`. The err variation is for when you *don't* get an update that tick because you're being rate limited.

The actual format is incremental - and built with the assumption that you're a JS client which is just merging data. The initial update will contain everything in the room, and each subsequent update will contain only changed properties of things. Things are set to `null` if they no longer exist.

Example initial message:

```json
{
  "flags": null,
  "info": {
    "mode": "world"
  },
  "objects": {
    "57cd3a30c0551957424a1f38": {
      "H": null,
      "K": null,
      "O": null,
      "X": null,
      "_id": "57cd3a30c0551957424a1f38",
      "energy": 0,
      "energyCapacity": 0,
      "room": "W0S0",
      "type": "terminal",
      "x": 34,
      "y": 35
    }
  },
  "users": {
    "2": {
      "_id": "2",
      "username": "Invader"
    },
    "3": {
      "_id": "3",
      "username": "Source Keeper"
    }
  }
}
```

You'll only get users which exist in the room. Each objects properties are exactly the properties the server stores in MongoDB - even down to ones which are no longer used. You might see some extra properties on structures in the center rooms in `shard0`, for example, because those properties were used before and are now kept as legacy data.

The subsequent updates will be in the exact same format, except with any properties which are the same missing. If, for example, someone added some "H" to that terminal and nothing else happened, the update would be:

```json
{
  "objects": {
    "57cd3a30c0551957424a1f38": {
      "H": 100
    }
  }
}
```

A bunch more examples of updates are available in [websocket-examples.md].
