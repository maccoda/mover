# Filtering Resources

It is also a fairly common use case to use query parameters to filter resources
by different properties. **This action is only applicable to collections and not
singluar resources**.

For the below database we could filter on a **title** for example.

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

```shell
> curl "http://localhost:5212/posts?title=mover"
[{"author":"maccoda","id":1,"title":"mover"}]
```

*Please note that this is still returned as an array*.
