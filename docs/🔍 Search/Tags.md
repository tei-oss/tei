Describes a search structure for [[Tag|tags]].

Stores documents with the following contents:
```
{
	"id": "<id>",
	"group_id": "<id>",
	"label": "wallpaper",
	"description": "An image fitting to be set as a wallpaper",
	"created_by": "<id>"
}
```

Search requests are expected to include filter `group_id=id`.

searchableAttributes: label, description
filterableAttributes: group_id, created_by
FTS ranking: label > description

Enables following scenarios:
- Search tags by label prefix (incl. typos)
- Search tags by description
- Narrow to user
- Widen across multiple user's groups (for copy workflow)