Describes a search structure for [[Entries|entries]] and [[Gallery - WIP|galleries]]. At this level these entities are considered the same.

Stores documents with the following contents:
```
{
	"id": "<id>",
	"group_id": "<id>",
	"tags": [text, text],
	"gallery_id": "<id?>",
	"created_by": "<id>",
	"is_gallery": "bool"
}
```

Search requests are expected to include filter `group_id=id`.

relevancy: attribute
searchableAttributes: tags
filterableAttributes: group_id, created_by, gallery_id, is_gallery

Enables following scenarios:
- Listing of entries
- Searching by tags
- Narrow down by gallery
- Narrow down by user
- Only search galleries/entries