# Design docs

| doc | what |
|---|---|
| [tasks/task-model.md](tasks/task-model.md) | the unit of work: file format, ids, statuses, transitions, review, evidence, steps |
| [roles/roles.md](roles/roles.md) | actors, roles, ownership, session boundaries |

The two cross-reference each other: the task model names the roles that act,
the roles doc names the transitions they act on.

Not defined yet: journal event kinds, schema validation, the CLI's operation
surface, raul screens and views, autopilot.
