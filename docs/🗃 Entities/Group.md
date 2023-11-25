A group represents a root level of entry organization. 

A group consists of:
- `id` — a [[Id|unique identifier]] of the group.
- `title` — an arbitrary textual name, associated with the group.
- `members` — a list of [[Group#Member|members]] of the group. *Maybe it shouldn't be accessible directly? How can we control members count?* ^members
- `tags` — a list of [[Tag|tags]] associated with the group.
- [[Audit]] information.

Invariants:
- [[Group#^members|Members]] must always contain at least one owner (see [[Group#Role|roles]])

Example workflows:
- Create group
- Manage members
## Member
Auxiliary object describing [[🗃 Entities/User]]'s role within the group.

Consists of:
- `user_id` — [[🗃 Entities/User#^id|Id]] of the user.
- `role` — [[Group#Role|Role]] of the user.
- `added_by` — [[🗃 Entities/User#^id|Id]] of a user who added the member.
- `added_at` — [[Timestamp]] when member was added.

## Role
Represents a set of permissions available to [[Group#^member|member]].

Available variants:
- `viewer` — can only view & download available content
- `uploader` — can add, tag and delete own content
- `admin` — can add, tag, delete all content
- `owner` — can add, tag, delete all content and users