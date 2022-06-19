# MELCloud

## Work In Progress


### Notes

- `Origin not allowed` behind nginx reverse proxy

Add the following line above the `proxy_pass`
```
proxy_set_header Host $http_host;
```