# Smaller tasks

Ideas, bugs and small features — the work that is not a task yet.

## Storage

One folder holds every work unit:

```
coding-lanes/work/
  TSK01-<slug>.json     # a task
  ITM01-<slug>.json     # a smaller task: an idea, a bug, a small feature
```

The id prefix says which is which. raul's backlog tab lists both, so the
storage does not split what the human sees together.

## Item

```json
{
  "schema_version": 1,
  "id": "ITM01",
  "title": "",
  "slug": "",
  "kind": "idea",
  "status": "open",
  "body": "",
  "scope": [],
  "tags": [],
  "created": "",
  "updated": "",
  "promoted_to": null,
  "dismissed_reason": ""
}
```

| field | values | meaning |
|---|---|---|
| `kind` | `idea` `feature` `bug` | a hunch, a small task, a small task that is a defect |
| `status` | `open` `dismissed` `promoted` | live, not worth pursuing, or became a task |
| `body` | free text | why the item is worth doing |
| `scope` | list | the work, when it is known |

## Rules

- The **kind is the only truth** about what an item is. The groups the human
  sees — ideas and small tasks — are derived from it, never stored.
- The **id is kind-neutral**: `ITM01` stays `ITM01` when an idea is shaped into
  a small task, so the trail survives the shaping.
- **Shaping is an update**: `idea → feature|bug`, plus a scope. The id does not
  change.
- **An idea may not be promoted.** It carries no acceptance criteria, and a
  task without them can never be signed off. The refusal names the remedy:
  *shape it into a small task first*.
- **Promotion is one-way.** The item records the task it became in
  `promoted_to`. The item is written first, so a crash cannot produce two tasks
  from one item.
- **Dismissal keeps the reason** in `dismissed_reason`.

## Promotion

A promoted small task becomes a task in `draft`:

| task field | value |
|---|---|
| `title` | the item's title |
| `intent.outcome` | the item's title |
| `intent.problem` | the item's body |
| `scope` | the item's scope |
| `acceptance_criteria` | exactly one — description = the title, verification empty |
| `kind` | `feature` or `bug` |
| `promoted_from` | the item's id |

No gate is weakened. The promoted task is **checkable, not pre-groomed**: the
verification, the out-of-scope entries and the steps are the task's own life,
exactly as for any task.

## Small tasks

A task whose `kind` is `bug` or `feature` is small. It keeps the full
lifecycle and every gate, except the step rules:

- no acceptance criterion needs a covering step
- no step needs a `done_when`
- no step needs to be settled before code review

The review stays. A small task still needs a self-review round before it goes
to code review, and a verdict before it is done.

## Derived, never stored

- the groups: ideas and small tasks, from the kind
- whether an item may be promoted, and why not
- what the human can do with an item now: shape it, promote it, or dismiss it
- whether a small task is ready to run

## Not defined yet

- journal event kinds for item moves
