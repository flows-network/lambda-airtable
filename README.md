 Config
env:
- `account`: Account that bind to AirTable in flows.network

# API

## Base URL
> <lambda url>

## endpoints

common query:
- `action`: `create` | `update`. The default is `create`
- `base_id`: AirTable's base_id
- `table_name`: AirTable's table_name

common body:
A json represented by `serde_json::Value`. Include only the content in (fields)[https://airtable.com/developers/web/api/create-records#request-fields]

> https://airtable.com/appebI0zAKp3fe9qE/tblx8KEJuGHULAoBX/viwaLgNG4EZbE4NMo?blocks=hide
>                      |----base_id----| |---table_name---|

### create record
query:
- `action`: `create` | null

#### Example
```bash
curl -X POST "https://<lambda url>?base_id={base_id}&table_name={table_name}" \
-H "Content-Type: application/json" \
--data '{
    "Address": "333 Post St",
    "Name": "Union Square",
    "Visited": true
  }'
```

> see also: https://airtable.com/developers/web/api/create-records

### update record
query:
- `action`: `update`
- `record_id`: AirTable's record_id

#### Example
```bash
curl -X POST "https://<lambda url>?base_id={base_id}&table_name={table_name}&record_id={record_id}&action={update}" \
-H "Content-Type: application/json" \
--data '{
    "Address": "333 Post St",
    "Name": "Union Square",
    "Visited": true
  }'
```

> see also: https://airtable.com/developers/web/api/update-record
