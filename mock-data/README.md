# mock-data

Mocked task data — one task per lifecycle stage, plus edge cases. Text is
lorem ipsum; the shape is what matters.

```
mock-data/
  coding-lanes/
    tasks/
      TSK01-*.json   draft
      TSK02-*.json   groomed
      TSK03-*.json   approved
      TSK04-*.json   executing
      TSK05-*.json   code-review
      TSK06-*.json   remediating
      TSK07-*.json   needs-human (5 external rounds exhausted)
      TSK08-*.json   done
      TSK09-*.json   blocked overlay
      TSK10-*.json   deferred criterion (scope-down)
      TSK11-*.json   child of TSK03
      TSK12-*.json   follow-up of the deferral in TSK10
```

The folder mirrors a managed project's data folder, so it can be copied into
a project as-is.

## Use in tests

The fixtures are valid tasks for their stage: every one satisfies the gates
of the phase it is in. Tests read them from the repo root:

```rust
let store = Store::new(repo_root().join("mock-data/coding-lanes"));
let task = store.load("TSK07")?;
```

`repo_root()` is `CARGO_MANIFEST_DIR/../..` from a crate under `crates/`.

## Not included

- `activity.json` — the journal format is not defined yet
- `config.json` — not defined yet
