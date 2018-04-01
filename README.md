Request Builder for Hyper

```rs
use hyper_request_builder::RequestBuilder;

let req = RequestBuilder::get("https://www.example.edu")
    .query("key","value")
    .query_from_object(MyQuery) // struct MyQuery { key1:String, key2:u32 }
    .query_from_json(json!({
        "key1":"value1",
        "key2":123,    
    }))
    .header_raw("Content-Length", "123123")
    .accept("application/json")
    .accept_json()
    .build(); // hyper::Request
```

## Goals
- Convenience
- Easy to remember
- As long as you don't use 'raw' methods, you are safe from typos.

```
RequestBuilder::get("https://www.example.com")
    .json()

RequestBuilder::get("https://www.example.com")
    .header_raw("Content-type", "")    
```

## Todo
- Use `Cow`
- Add more helper methods