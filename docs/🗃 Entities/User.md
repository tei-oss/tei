A user represents a signed in product user. Users are automatically created after sign in (assuming OIDC is used). User does not contain any authentication data — only external Id from identity provider.

A user consists of:
- `id` — a [[Id|unique identifier]] of a user. ^id
- `alias` — a unique, user selected display name within platform. ^alias
- `external_id` — id of a user in an external identity provider.

Invariants:
- [[🗃 Entities/User#^alias|Alias]] is case-insensitive
- [[🗃 Entities/User#^alias|Alias]] is globally unique
- [[🗃 Entities/User#^alias|Alias]] can be changed by the user

Example workflows:
- Sign in
- View home feed
- Manage groups