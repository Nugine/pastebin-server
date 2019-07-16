# pastebin-server

## Deploy

```bash
$ cargo build --release
$ . ./start.sh
```


## JSON API

+ POST `/record`
  
    ```typescript
    interface Request{
        title: string,
        lang: string,
        content: string,
        expiration: number,
    }
    ```

    ```typescript
    interface Response{
        key: string
    }
    ```

+ GET `/record/{key}`

    ```typescript
    interface Response{
        title: string,
        lang: string,
        content: string,
        saving_time: number,
        expiration: number,
        view_count: number
    }
    ```

+ Error

    ```typescript
    interface Error{
        code: number,
        message: string
    }
    ```

## Environment

Prefix: "PASTEBIN_"

| var            | default        | unit        | description                                                 |
| -------------- | -------------- | ----------- | ----------------------------------------------------------- |
| MAX_STORE_SIZE | 104857600      | byte        | An ambiguous size count for controlling server memory usage |
| MAX_POST_SIZE  | 32768          | byte        | Max length of POST request body                             |
| MAX_EXPIRATION | 604800         | second      | Max expiration time                                         |
| CLEAN_DURATION | 5000           | millisecond | GC interval                                                 |
| ADDR           | localhost:8088 |             | Binding address                                             |
| CRYPT_KEY      | magic          |             | Crypto key for short url                                    |

### Example

`.env`

```
RUST_LOG=info
PASTEBIN_ADDR=localhost:8000
PASTEBIN_CRYPT_KEY=MyImportantSecret
```
