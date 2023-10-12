# Puan PV API
A simple HTTP-API for the [Puan PV project](https://github.com/ourstudio-se/puan-pv-rust).

# Usage
Stand in root directory and run:
```bash
cargo run
```

Then use by sending a POST request to (default) `http://localhost:8080/evaluate` with the following body:
```json
{
    "evaluables": [
        {
            "prop": {
                "type": "AtMost",
                "variables": [
                    {
                        "type": "And",
                        "variables": [
                            {
                                "type": "Variable",
                                "id": "a"
                            },
                            {
                                "type": "Variable",
                                "id": "b"
                            }
                        ]
                    },
                    {
                        "type": "Variable",
                        "id": "b"
                    }
                ],
                "value": 1
            },
            "key": "abc"
        },
        {
            "prop": {
                "type": "Implies",
                "left": {
                    "type": "AtLeast",
                    "variables": [
                        {
                            "type": "Variable",
                            "id": "a"
                        },
                        {
                            "type": "Variable",
                            "id": "b"
                        }
                    ],
                    "value": 1
                },
                "right": {
                    "type": "Xor",
                    "variables": [
                        {
                            "type": "Variable",
                            "id": "x"
                        },
                        {
                            "type": "Variable",
                            "id": "y"
                        },
                        {
                            "type": "Variable",
                            "id": "z"
                        }
                    ]
                }
            },
            "key": "def"
        }
    ],
    "interpretation": {
        "a": 0,
        "b": 1,
        "x": 1
    }
}
```
If the first proposition is true, given the interpretation, then `abc` will be returned. If the second proposition is true, given the interpretation, then `def` will be returned. If neither is true, then an empty list will be returned.