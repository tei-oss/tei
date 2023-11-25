#todo

An entry represents file within the system.

An entry consists of:
- `id` — a [[Id|unique identifier]] of the entry.
- `name` — optional, arbitrary name of the entry.
- `content_type` — mime type of the entry.
- `size` — size of the entry in bytes.
- `tags` — array of [[Tag|tags]].
- `thumbnail_url` — optional thumbnail url
- `download_url` — url to download original content
- [[Audit]] information

Invariants:
- Content, content_type, size are immutable

Example workflows:
- Upload file
- Search entries
- Create thumbnails for `image/*` or `video/*`
- Get untagged entries
- Download ZIP