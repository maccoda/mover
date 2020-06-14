# Configuration

The configuration file is written in JSON, below is an example that we will
decompose:

```json
{
  "posts": [
    { "id": 1, "title": "mover", "author": "maccoda" },
    {
      "id": 2,
      "title": "web stuff",
      "author": "bob"
    }
  ],
  "comments": [{ "id": 1, "body": "some comment", "postId": 1 }],
  "profile": { "name": "maccoda" }
}
```

All top level keys will be mapped to the path they are accessible from, for
example to access the `posts` resources above it is accessed at the path
`/posts`.

## Resource Types

### Singular resource

A singular resource can be represented by a basic object. This is shown by the
`profile` resource above.

```json
"profile" : { "name": "maccoda" }
```

### Collection of resources

A collection can be represented by an array. This is shown by then `posts`
resource above.

```json
"posts": [
    { "id": 1, "title": "mover", "author": "maccoda" },
    {
      "id": 2,
      "title": "web stuff",
      "author": "bob"
    }
]
```
