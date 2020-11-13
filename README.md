<p align="center"><img src="static/images/logo.svg" height="200px"></p>
<h1 align="center">Raw Poll</h1>

<p align="center">Raw poll makes it easy to create new polls in seconds and free and open source!</p>

## Api

### Poll

1. Create
    ```
    POST /api/poll
    ```

    Format: `application/json`

    Params:    
    | Name          | Type   | Notes     |
    |---------------|--------|-----------|
    | `question`    | String | Required. |
    | `description` | String | Required. |
    | `options`     | Array  | Required. |

    Response ( If the poll was successfully created ):
    ```json
    {
        "id": "poll_id",
        "status": "success"
    }
    ```

2. Read
    ```
    GET /api/poll/<poll_id>
    ```

    If the poll was found:
    ```json
    {
        "question": "poll_question",
        "description": "poll_description",
        "options": {
            "poll_option": 1,
            "poll_option-2": 1,
            ...
        },
    }
    ```

    If the poll was not found:
    ```json
    {
        "error": "Cannot find the poll specified!",
        "status": "failure"
    }
    ```