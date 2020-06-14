# Collection

To represent a collection of resources at **path**, add the key `path` and
create the desired value as a JSON array.

## Fetch entire collection

It is possible to fetch the entire collection simply by accessing the key of the
collection on the server.

For example:


`db.json`
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
}
```

With the server started this will be accessible at `/profile`

```shell
> curl http://localhost:5212/posts
[{"author":"maccoda","id":1,"title":"mover"},{"author":"bob","id":2,"title":"web stuff"}]
```

## Access by ID

It is also possible to access a single element of a collection by using the `id`
field. To do this is does require that an `id` field is set in the database.

Using the above database:

```shell
> curl http://localhost:5212/posts/2
{"author":"bob","id":2,"title":"web stuff"}
``
