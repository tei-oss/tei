A tag represents a special keyword assigned to an [[Entry - WIP|entry]] to allow it to be found by browsing or searching. Tags are configured within a [[Group|group]] and UI should provide a streamlined experience to create them (when working with entry within a group) or copy (when trying to access one from another group).

A tag consists of:
- `id` — a [[Id|unique identifier]] of the tag.
- `label` — a short textual representation of the tag. Usually displayed alongside [[Entry - WIP|entries]]. ^label
- `color` — color, which should be used by frontend application. It is not yet clear whether it should contain actual hex value (values for bright/dark themes?) or predefined value from a theme.
- `description` — optional textual description to help other users to understand whether this tag aligns with their use case or not.
- `icon` — optional icon from predefined set, used in the UI. 
- `group_id` — a [[Id|unique identifier]] of the owning [[Group|group]].
- [[Audit]] information.

Invariants:
- Tags are only searchable within [[Group|groups]] the [[🗃 Entities/User|user]] is a member of.
- [[Tag#^label|Label]] is case-insensitive
- [[Tag#^label|Label]] is unique within group
- [[Tag#^label|Label]] is immutable
- Tag deletion leads to cascade update of all linked [[Entry - WIP|entries]], unlinking them.

Example workflows:
- Entry search by tag autocompletion
- Tag listing within group
- Tag search by description
- Displaying tags of entries