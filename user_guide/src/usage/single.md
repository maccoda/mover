# Single element

To represent a single element resource at **path**, add the key `path` to the
database and create the desired value as a JSON object.

For example:

`db.json`
```json
{
    "profile": { "name": "maccoda" }
}
```

With the server started this will be accessible at `/profile`

```shell
> curl http://localhost:5212/profile
{"name":"maccoda"}
```
