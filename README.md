# MongoDB to Prisma Schema Generator

![Test Status](https://github.com/ShaunSHamilton/mongodb-prisma-schema/actions/workflows/test.yml/badge.svg)

## How Works

Recurses given collection, and generates a schema based on the data.

## Example Input

```json
[
  {
    "_id": "ObjectId('1234')",
    "name": "A",
    "age": 12,
    "files": [
      {
        "name": "hi",
        "ext": "js"
      },
      {
        "name": "hi",
        "ext": null,
        "size": 123
      }
    ]
  },
  {
    "_id": "ObjectId('1234')",
    "name": "B",
    "files": [
      {
        "name": "hi",
        "ext": "js"
      },
      {
        "name": "hi",
        "ext": "css",
        "size": [1, 2, "3"]
      }
    ]
  }
]
```

## Example Output

```json
{
  "_id": ["ObjectId"],
  "name": ["String"],
  "age": ["Int32"],
  "files": [
    {
      "name": ["String"],
      "ext": ["String", "Null"],
      "size": ["Int32", ["Int32", "String"]]
    }
  ]
}
```

> [!NOTE] `age` field is not present in the second object, but it is still added to the schema.
> All fields should be considered optional!

Current logic is flawed:
If nested values match, they will still be added to the schema:
User A:

```json
{
  "_id": "1234",
  "files": [{ "name": "hi", "ext": "js" }]
}
```

User B:

```json
{
  "_id": 1234,
  "files": [{ "name": "hi", "ext": null }]
}
```

Will result in:

```json
{
  "_id": ["String", "Int32"],
  "files": [
    {
      "name": ["String"],
      "ext": ["String"]
    },
    {
      "name": ["String"],
      "ext": ["Null"]
    }
  ]
}
```

## Usage

Generate schema

```bash
cargo run --release -- --db <db> --collection <collection> --out <out> --url <url>
```

Example:

```bash
cargo run --release -- --db freecodecamp --collection user --out schema.json --url mongodb://localhost:27017
```

Run tests

```bash
cargo test
```
