#todo : write about external audit

All relevant entities must contain both inline audit data and logged audit data.

## Inline audit data
Inline audit data contains following:
- `created_at` — [[Timestamp|timestamp]] of creation.
- `updated_at` — [[Timestamp|timestamp]] of the last modification. Initially should be null.
- `created_by` — [[🗃 Entities/User#^id|Id]] of the user who created this entity .
- `updated_by` — [[🗃 Entities/User#^id|Id]] of the last user who modified this entity.
- `version` — integer version of given entity. May be used to perform [[CAS Updates|CAS updates]]. ^version

Invariants:
- [[Audit#^version|Version]] should always increase.

## External audit data
TODO