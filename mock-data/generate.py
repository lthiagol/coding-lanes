#!/usr/bin/env python3
"""Generate the mock task fixtures for coding-lanes.

Every fixture satisfies the gates of the phase it is in, and the dependency
graph only points backwards, so the set is valid as a whole.

Run from the repo root:

    python3 mock-data/generate.py
"""

import json
import pathlib
import shutil

ROOT = pathlib.Path(__file__).resolve().parent
TASKS = ROOT / "coding-lanes" / "work"

TITLES = [
    "Lorem ipsum dolor sit amet",
    "Consectetur adipiscing elit sed do eiusmod",
    "Tempor incididunt ut labore et dolore",
    "Magna aliqua ut enim ad minim veniam",
    "Iusto odio dignissimos ducimus qui blanditiis",
    "Praesentium voluptatum deleniti atque corrupti",
    "Quaerat voluptatem consequatur sunt in culpa",
    "Officiis debitis aut rerum necessitatibus",
    "Sed ut perspiciatis unde omnis iste natus",
    "Quis autem vel eum iure reprehenderit",
    "Nam libero tempore cum soluta nobis est",
    "Eligendi optio cumque nihil impedit quo minus",
    "Et harum quidem rerum facilis est et expedita",
    "Nemo enim ipsam voluptatem quia voluptas sit",
    "At vero eos et accusamus et iusto odio",
    "Totam rem aperiam eaque ipsa quae ab illo",
    "Ut enim ad minim veniam quis nostrud",
    "Duis aute irure dolor in reprehenderit",
    "Nulla pariatur excepteur sint occaecat",
    "Temporibus autem quibusdam et aut officiis",
]

OUTCOMES = [
    "Tempor incididunt ut labore et dolore magna aliqua, ut enim ad minim veniam.",
    "Quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
    "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat.",
    "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque.",
    "Totam rem aperiam eaque ipsa quae ab illo inventore veritatis et quasi architecto.",
    "Praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi.",
    "Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit.",
    "Et harum quidem rerum facilis est et expedita distinctio nam libero tempore.",
    "Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe.",
    "Ut et voluptates repudiandae sint et molestiae non recusandae itaque earum rerum.",
]

PROBLEMS = [
    "Quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
    "Occaecati cupiditate non provident sed ut perspiciatis unde omnis iste natus error.",
    "At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium.",
    "Ea voluptate velit esse quam nihil molestiae consequatur vel illum qui dolorem eum.",
    "Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet.",
    "Soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat.",
    "Itaque earum rerum hic tenetur a sapiente delectus ut aut reiciendis voluptatibus.",
    "Nulla pariatur excepteur sint occaecat cupidatat non proident sunt in culpa qui officia.",
]

ACTIONS = [
    "Sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt.",
    "Neque porro quisquam est qui dolorem ipsum quia dolor sit amet consectetur.",
    "At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium.",
    "Itaque earum rerum hic tenetur a sapiente delectus ut aut reiciendis voluptatibus.",
    "Et harum quidem rerum facilis est et expedita distinctio nam libero tempore cum soluta.",
    "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat.",
    "Nulla pariatur excepteur sint occaecat cupidatat non proident sunt in culpa qui officia.",
    "Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet.",
]

DONE_WHEN = [
    "The lorem module is read and its behaviour is described in the notes.",
    "The lorem tests exist and pass.",
    "The lorem lorem lorem is covered by a lorem test.",
    "The lorem behaviour is verified by the lorem command.",
]

CRITERIA = [
    "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque.",
    "Totam rem aperiam eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae.",
    "Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit sed quia.",
    "Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet.",
    "Ut et voluptates repudiandae sint et molestiae non recusandae itaque earum rerum hic.",
    "Soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat.",
]

FINDINGS = [
    "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor.",
    "Incididunt ut labore et dolore magna aliqua, ut enim ad minim veniam, quis nostrud.",
    "Quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat.",
    "Consequat duis aute irure dolor in reprehenderit in voluptate velit esse cillum.",
    "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu.",
    "Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit sed.",
]

NOTES = [
    "Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe.",
    "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip.",
    "Et harum quidem rerum facilis est et expedita distinctio nam libero tempore cum.",
    "Quaerat voluptatem consequatur sunt in culpa qui officia deserunt mollit anim id.",
]

VERIFICATIONS = [
    "cargo test --workspace",
    "cargo test -p coding-lanes-core",
    "cargo clippy --all-targets -- -D warnings",
    "make check",
]

SEVERITIES = ["low", "medium", "high"]
PRIORITIES = ["urgent", "high", "normal", "low"]
EFFORTS = ["S", "M", "L"]
FOCI = ["spec-compliance", "edge-cases", "security", "empirical", "design"]


def slugify(title):
    out = []
    dash = False
    for ch in title.lower():
        if ch.isalnum():
            if dash and out:
                out.append("-")
            dash = False
            out.append(ch)
        else:
            dash = True
    return "".join(out)


def pick(seq, index):
    return seq[index % len(seq)]


def ts(day, hour):
    return f"2026-11-{day:02d}T{hour:02d}:00Z"


def evidence(index, by, session, day, hour, passed=True):
    return {
        "command": pick(VERIFICATIONS, index),
        "exit_code": 0 if passed else 1,
        "summary": f"{10 + index} passed; 0 failed" if passed else f"1 failed; {9 + index} passed",
        "at": ts(day, hour),
        "by": by,
        "session": session,
    }


def criterion(index, with_evidence=None, deferral=None, cid="ACCEPT-01"):
    item = {
        "id": cid,
        "description": pick(CRITERIA, index),
        "verification": pick(VERIFICATIONS, index),
        "status": "pending",
        "evidence": with_evidence or [],
        "deferral": deferral,
        "failure_reason": "",
    }
    return item


def step(index, order, status, covers, files, deviation="", depends_on=None):
    return {
        "id": f"STP-{order:02d}",
        "order": order,
        "action": pick(ACTIONS, index),
        "done_when": pick(DONE_WHEN, index),
        "files": files,
        "status": status,
        "covers": covers,
        "depends_on": depends_on or [],
        "deviation": deviation,
    }


def finding(index, status="open", resolution="", resolved=""):
    return {
        "id": f"FIND-{index:02d}",
        "severity": pick(SEVERITIES, index),
        "status": status,
        "description": pick(FINDINGS, index),
        "created": ts(8, 10 + index),
        "resolved": resolved,
        "resolution": resolution,
        "comments": [],
    }


def round_(n, kind, focus, reviewer, verdict, index, findings=None):
    return {
        "n": n,
        "kind": kind,
        "focus": focus,
        "reviewer": reviewer,
        "verdict": verdict,
        "at": ts(9, 9 + n),
        "notes": pick(NOTES, index),
        "findings": findings or [],
    }


def task(task_id, title, status, index, **overrides):
    item = {
        "schema_version": 1,
        "id": task_id,
        "title": title,
        "slug": slugify(title),
        "parent": None,
        "children": [],
        "created": ts(2 + index % 6, 9 + index % 8),
        "updated": ts(4 + index % 6, 10 + index % 8),
        "status": status,
        "overlays": {},
        "claim": None,
        "planned_by": None,
        "meta": {"priority": pick(PRIORITIES, index), "effort": pick(EFFORTS, index), "tags": ["mock"]},
        "intent": {"outcome": "", "problem": ""},
        "scope": {"in_scope": [], "out_of_scope": []},
        "acceptance_criteria": [],
        "steps": [],
        "review": {"max_rounds": 5, "rounds": [], "verdict": None, "findings": []},
        "questions": [],
        "notes": [],
        "depends_on": [],
    }
    item.update(overrides)
    return item


def build():
    tasks = []
    counter = 0

    def next_id():
        nonlocal counter
        counter += 1
        return f"TSK{counter:02d}"

    ids = {}

    # draft — title only, plus one child of an approved task.
    for i in range(3):
        tid = next_id()
        ids.setdefault("draft", []).append(tid)
        tasks.append(task(tid, pick(TITLES, i + 14), "draft", i))

    # groomed — intent, scope, criteria; one of them blocked.
    for i in range(3):
        tid = next_id()
        ids.setdefault("groomed", []).append(tid)
        item = task(
            tid,
            pick(TITLES, i + 8),
            "groomed",
            i + 3,
            planned_by=f"session-creator-{i + 1:02d}",
            intent={"outcome": pick(OUTCOMES, i), "problem": pick(PROBLEMS, i)},
            scope={
                "in_scope": [pick(ACTIONS, i)],
                "out_of_scope": [pick(PROBLEMS, i + 4)],
            },
            acceptance_criteria=[criterion(i)],
        )
        if i == 1:
            item["overlays"] = {
                "blocked": {
                    "reason": pick(FINDINGS, i),
                    "by": f"creator-{i + 1:02d}",
                    "at": ts(8, 14),
                }
            }
        tasks.append(item)

    # approved — criteria covered by steps with done_when.
    for i in range(3):
        tid = next_id()
        ids.setdefault("approved", []).append(tid)
        item = task(
            tid,
            pick(TITLES, i + 2),
            "approved",
            i + 6,
            planned_by=f"session-creator-{i + 1:02d}",
            intent={"outcome": pick(OUTCOMES, i + 3), "problem": pick(PROBLEMS, i + 2)},
            scope={
                "in_scope": [pick(ACTIONS, i + 1)],
                "out_of_scope": [pick(PROBLEMS, i + 5)],
            },
            acceptance_criteria=[criterion(i, cid="ACCEPT-01"), criterion(i + 3, cid="ACCEPT-02")],
            steps=[
                step(i, 1, "pending", ["ACCEPT-01"], ["crates/core/src/task.rs"]),
                step(i + 1, 2, "pending", ["ACCEPT-02"], ["crates/core/tests/task_model.rs"], depends_on=["STP-01"]),
            ],
            depends_on=[ids["groomed"][i], ids["groomed"][(i + 1) % 3]],
        )
        tasks.append(item)

    # executing — claimed, evidence on the first criterion; one defers a criterion.
    for i in range(3):
        tid = next_id()
        ids.setdefault("executing", []).append(tid)
        criteria = [
            criterion(i, with_evidence=[evidence(i, f"runner-{i + 1:02d}", f"session-runner-{i + 1:02d}", 10, 12)]),
            criterion(i + 3),
        ]
        if i == 1:
            criteria[1]["status"] = "deferred"
            criteria[1]["deferral"] = {
                "reason": pick(PROBLEMS, i + 1),
                "follow_up": ids["draft"][2],
            }
        item = task(
            tid,
            pick(TITLES, i + 11),
            "executing",
            i + 9,
            planned_by=f"session-creator-{i + 1:02d}",
            claim={
                "by": f"runner-{i + 1:02d}",
                "session": f"session-runner-{i + 1:02d}",
                "at": ts(10, 9),
                "lease_expires_at": ts(10, 17),
            },
            intent={"outcome": pick(OUTCOMES, i + 5), "problem": pick(PROBLEMS, i + 3)},
            scope={
                "in_scope": [pick(ACTIONS, i + 2)],
                "out_of_scope": [pick(PROBLEMS, i + 6)],
            },
            acceptance_criteria=criteria,
            steps=[
                step(
                    i,
                    1,
                    "in-progress" if i == 0 else "done",
                    ["ACCEPT-01"],
                    ["crates/core/tests/task_model.rs"],
                    deviation=pick(NOTES, i) if i == 0 else "",
                )
            ],
            review={"max_rounds": 5, "rounds": [], "verdict": None, "findings": []},
            notes=[{"id": "NOTE-01", "author": f"runner-{i + 1:02d}", "body": pick(NOTES, i), "at": ts(10, 13)}],
            depends_on=[ids["approved"][i], ids["approved"][(i + 2) % 3]],
        )
        tasks.append(item)

    # code-review — steps settled, evidence, self round, one open finding.
    for i in range(3):
        tid = next_id()
        ids.setdefault("code-review", []).append(tid)
        findings = [finding(i + 1)]
        item = task(
            tid,
            pick(TITLES, i + 5),
            "code-review",
            i + 12,
            planned_by=f"session-creator-{i + 1:02d}",
            claim={
                "by": f"runner-{i + 1:02d}",
                "session": f"session-runner-{i + 1:02d}",
                "at": ts(11, 9),
                "lease_expires_at": ts(11, 17),
            },
            intent={"outcome": pick(OUTCOMES, i + 7), "problem": pick(PROBLEMS, i + 4)},
            scope={
                "in_scope": [pick(ACTIONS, i + 3)],
                "out_of_scope": [pick(PROBLEMS, i)],
            },
            acceptance_criteria=[criterion(i, with_evidence=[evidence(i, f"runner-{i + 1:02d}", f"session-runner-{i + 1:02d}", 11, 16)])],
            steps=[step(i, 1, "done", ["ACCEPT-01"], ["crates/core/tests/task_model.rs"])],
            review={
                "max_rounds": 5,
                "rounds": [round_(1, "self", "spec-compliance", f"runner-{i + 1:02d}", "changes-needed", i, ["FIND-01"])],
                "verdict": None,
                "findings": findings,
            },
            depends_on=[ids["executing"][i], ids["executing"][(i + 1) % 3]],
        )
        tasks.append(item)

    # remediating — external round with changes-needed, findings open.
    for i in range(3):
        tid = next_id()
        ids.setdefault("remediating", []).append(tid)
        item = task(
            tid,
            pick(TITLES, i + 9),
            "remediating",
            i + 15,
            planned_by=f"session-creator-{i + 2:02d}",
            claim={
                "by": f"runner-{i + 2:02d}",
                "session": f"session-runner-{i + 2:02d}",
                "at": ts(12, 9),
                "lease_expires_at": ts(12, 17),
            },
            intent={"outcome": pick(OUTCOMES, i + 2), "problem": pick(PROBLEMS, i + 5)},
            scope={
                "in_scope": [pick(ACTIONS, i + 4)],
                "out_of_scope": [pick(PROBLEMS, i + 1)],
            },
            acceptance_criteria=[criterion(i, with_evidence=[evidence(i, f"runner-{i + 2:02d}", f"session-runner-{i + 2:02d}", 12, 13)])],
            steps=[step(i, 1, "done", ["ACCEPT-01"], ["crates/core/tests/task_model.rs"])],
            review={
                "max_rounds": 5,
                "rounds": [
                    round_(1, "self", "spec-compliance", f"runner-{i + 2:02d}", "ok", i),
                    round_(2, "external", pick(FOCI, i + 1), f"reviewer-{i + 1:02d}", "changes-needed", i, ["FIND-01"]),
                ],
                "verdict": "changes-needed",
                "findings": [finding(i + 1)],
            },
            depends_on=[ids["code-review"][i], ids["code-review"][(i + 2) % 3]],
        )
        tasks.append(item)

    # needs-human — five external rounds exhausted.
    for i in range(3):
        tid = next_id()
        ids.setdefault("needs-human", []).append(tid)
        rounds = [round_(1, "self", "spec-compliance", f"runner-{i + 3:02d}", "ok", i)]
        findings = []
        for n in range(2, 7):
            findings.append(f"FIND-{n - 1:02d}")
            rounds.append(
                round_(n, "external", pick(FOCI, n), f"reviewer-{n - 1:02d}", "changes-needed", n, [f"FIND-{n - 1:02d}"])
            )
        item = task(
            tid,
            pick(TITLES, i + 17),
            "needs-human",
            i + 18,
            planned_by=f"session-creator-{i + 3:02d}",
            claim={
                "by": f"runner-{i + 3:02d}",
                "session": f"session-runner-{i + 3:02d}",
                "at": ts(13, 9),
                "lease_expires_at": ts(13, 17),
            },
            intent={"outcome": pick(OUTCOMES, i + 4), "problem": pick(PROBLEMS, i + 6)},
            scope={
                "in_scope": [pick(ACTIONS, i + 5)],
                "out_of_scope": [pick(PROBLEMS, i + 2)],
            },
            acceptance_criteria=[criterion(i, with_evidence=[evidence(i, f"runner-{i + 3:02d}", f"session-runner-{i + 3:02d}", 13, 17)])],
            steps=[step(i, 1, "done", ["ACCEPT-01"], ["crates/core/tests/task_model.rs"])],
            review={
                "max_rounds": 5,
                "rounds": rounds,
                "verdict": "changes-needed",
                "findings": [finding(n) for n in range(1, 6)],
            },
            notes=[{"id": "NOTE-01", "author": f"reviewer-{5:02d}", "body": pick(NOTES, i), "at": ts(14, 18)}],
            depends_on=[ids["remediating"][i], ids["remediating"][(i + 1) % 3]],
        )
        tasks.append(item)

    # done — clean verdict, no open findings. Five of them, chained.
    done_dependencies = [
        [ids["needs-human"][0], ids["needs-human"][1]],
        [ids["needs-human"][2], None],  # None: filled below with done[0]
        [None, None],  # filled below with done[0], done[1]
        [ids["needs-human"][0], None],  # None: filled below with done[2]
        [None, None, None],  # filled below with done[1..3]
    ]
    for i in range(5):
        tid = next_id()
        ids.setdefault("done", []).append(tid)
        item = task(
            tid,
            pick(TITLES, i + 12),
            "done",
            i + 21,
            planned_by=f"session-creator-{i + 4:02d}",
            claim={
                "by": f"runner-{i + 4:02d}",
                "session": f"session-runner-{i + 4:02d}",
                "at": ts(15, 9),
                "lease_expires_at": ts(15, 17),
            },
            intent={"outcome": pick(OUTCOMES, i + 6), "problem": pick(PROBLEMS, i)},
            scope={
                "in_scope": [pick(ACTIONS, i + 6)],
                "out_of_scope": [pick(PROBLEMS, i + 3)],
            },
            acceptance_criteria=[criterion(i, with_evidence=[evidence(i, f"runner-{i + 4:02d}", f"session-runner-{i + 4:02d}", 15, 16)])],
            steps=[step(i, 1, "done", ["ACCEPT-01"], ["crates/core/tests/task_model.rs"])],
            review={
                "max_rounds": 5,
                "rounds": [
                    round_(1, "self", "spec-compliance", f"runner-{i + 4:02d}", "ok", i),
                    round_(2, "external", pick(FOCI, i), f"reviewer-{i + 5:02d}", "ok", i),
                ],
                "verdict": "ok",
                "findings": [],
            },
            depends_on=[],
        )
        item["depends_on"] = [
            dependency
            for dependency in done_dependencies[i]
            if dependency is not None
        ]
        tasks.append(item)

    # The done chain: each task waits on the previous one, the last one on
    # three at once.
    done_ids = ids["done"]
    done_dependencies[1] = [ids["needs-human"][2], done_ids[0]]
    done_dependencies[2] = [done_ids[0], done_ids[1]]
    done_dependencies[3] = [ids["needs-human"][0], done_ids[2]]
    done_dependencies[4] = [done_ids[1], done_ids[2], done_ids[3]]
    for index, item in enumerate(tasks):
        if item["id"] in done_ids:
            position = done_ids.index(item["id"])
            item["depends_on"] = done_dependencies[position]

    # smaller tasks — ideas, bugs and small features, plus the trail of one
    # that became a task.
    def item(item_id, title, kind, status, body="", scope=None, promoted_to=None,
             dismissed_reason=""):
        return {
            "schema_version": 1,
            "id": item_id,
            "title": title,
            "slug": slugify(title),
            "kind": kind,
            "status": status,
            "body": body,
            "scope": scope or [],
            "tags": ["mock"],
            "created": ts(2, 8),
            "updated": ts(6, 15),
            "promoted_to": promoted_to,
            "dismissed_reason": dismissed_reason,
        }

    items = [
        item("ITM01", "Lorem ipsum dolor sit amet", "idea", "open",
             body="Consectetur adipiscing elit, sed do eiusmod tempor incididunt."),
        item("ITM02", "Tempor incididunt ut labore et dolore", "feature", "open",
             body="Magna aliqua, ut enim ad minim veniam.",
             scope=["the lorem pane"]),
        item("ITM03", "Quis nostrud exercitation ullamco laboris", "bug", "open",
             body="Nisi ut aliquip ex ea commodo consequat.",
             scope=["the lorem list"]),
        item("ITM04", "Sed do eiusmod tempor incididunt", "feature", "dismissed",
             body="Ut labore et dolore magna aliqua.",
             dismissed_reason="Lorem ipsum dolor sit amet, consectetur."),
    ]

    # A bug that was promoted: the task it became is a small task.
    promoted_title = "Duis aute irure dolor in reprehenderit"
    promoted_id = next_id()
    promoted_task = task(promoted_id, promoted_title, "draft", 4)
    promoted_task["kind"] = "bug"
    promoted_task["promoted_from"] = "ITM05"
    promoted_task["intent"] = {
        "outcome": promoted_title,
        "problem": "Velit esse cillum dolore eu fugiat.",
    }
    promoted_task["scope"] = {"in_scope": ["the lorem button"], "out_of_scope": []}
    promoted_task["acceptance_criteria"] = [
        {
            "id": "ACCEPT-01",
            "description": promoted_title,
            "verification": "",
            "status": "pending",
            "evidence": [],
            "deferral": None,
            "failure_reason": "",
        }
    ]
    tasks.append(promoted_task)
    items.append(
        item("ITM05", promoted_title, "bug", "promoted",
             body="Velit esse cillum dolore eu fugiat.",
             scope=["the lorem button"],
             promoted_to=promoted_id)
    )

    # parent / child: the first approved task owns a draft child.
    child = next_id()
    ids.setdefault("draft", []).append(child)
    child_task = task(child, pick(TITLES, 19), "draft", 2)
    child_task["parent"] = ids["approved"][0]
    tasks.append(child_task)
    for item in tasks:
        if item["id"] == ids["approved"][0]:
            item["children"] = [child]

    # approved after the last cycle finished: its dependencies are done, so
    # an agent can claim it right now. It is generated last, so it takes the
    # next id and the ids of the older fixtures keep their meaning.
    tid = next_id()
    ids.setdefault("approved", []).append(tid)
    tasks.append(
        task(
            tid,
            pick(TITLES, 20),
            "approved",
            6,
            planned_by="session-creator-05",
            intent={"outcome": pick(OUTCOMES, 8), "problem": pick(PROBLEMS, 7)},
            scope={
                "in_scope": [pick(ACTIONS, 7)],
                "out_of_scope": [pick(PROBLEMS, 3)],
            },
            acceptance_criteria=[
                criterion(2, cid="ACCEPT-01"),
                criterion(5, cid="ACCEPT-02"),
            ],
            steps=[
                step(2, 1, "pending", ["ACCEPT-01"], ["crates/core/src/task.rs"]),
                step(
                    5,
                    2,
                    "pending",
                    ["ACCEPT-02"],
                    ["crates/core/tests/task_model.rs"],
                    depends_on=["STP-01"],
                ),
            ],
            depends_on=[done_ids[0]],
        )
    )

    return tasks, items


def main():
    if TASKS.exists():
        shutil.rmtree(TASKS)
    TASKS.mkdir(parents=True)
    tasks, items = build()
    for record in tasks + items:
        path = TASKS / f"{record['id']}-{record['slug']}.json"
        path.write_text(json.dumps(record, indent=2, ensure_ascii=False) + "\n")
        print(path.relative_to(ROOT))


if __name__ == "__main__":
    main()
