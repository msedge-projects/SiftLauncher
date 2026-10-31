# TOOLING.md — what this environment can actually do

Written 2026-10-31 after a session in which the first hour was spent doing by hand what a
delegation tool and an image reader would have done in parallel. The lesson is one line:

**The visible tool list is not the tool list.** The harness exposes a runtime registry that is
reachable from inside a `run_code` program, and it is larger than the function signature the
model is handed. Enumerate it before concluding a capability is missing:

```ts
console.log(Object.keys(tools).join(", "));
```

## The catalogue, as observed on 2026-10-31

| tool | arguments (required) | what it is for |
| --- | --- | --- |
| `read_code`/`run_code` | `code`, `description` | the program that runs everything else |
| `bash` | `description`, `command` | a shell command with a description the UI shows |
| `read` | `file_path` | read a text file |
| `write` | `file_path`, `content` | write a text file |
| `edit` | `file_path`, `old_string`, `new_string` | anchored edit |
| `glob` | `pattern` | find files |
| `grep` | `pattern` | search contents |
| `read_image` | `file_path` | **look at a PNG** and get it back as an image the model can see |
| `present` | `files` | show files to the user |
| `todo_write` | `todos` | maintain a task list |
| `subagent` | `description`, `prompt`, optional `cwd` | spawn a fresh child agent, no shared context |
| `subagent_fork` | as above | spawn a child seeded with this conversation |
| `list_agents` | — | liveness of children |
| `send_message` | `agent_id`, `message` | steer or continue a child |
| `interrupt_agent` | `agent_id` | stop a child |
| `job_list` / `job_output` | `job_id` | background jobs started from `run_code` |
| `create_goal` / `get_goal` / `update_goal` | `objective`, `max_goal_rounds` / `revision` | the persisted goal |
| `schedule_create` / `schedule_list` / `schedule_delete` / `schedule_update` | — | timed work |
| `web_search` | `queries` | search the web |
| `web_fetch` | `url` | fetch a page |
| `skill` | `name` | load a named skill |
| `ask_user_question` | `questions` | ask the human |
| `working_directory` | — | current cwd |
| `exit_plan_mode` | — | leave planning |

## The three that mattered here

1. **`read_image`.** Every parity claim in this repository is supposed to rest on a capture.
   Before this was found, captures were measured with `tools/parity/probe.py` (row and column
   colour runs, OCR boxes, bounding boxes) and never *looked at*; the numbers were right and the
   picture was still a surprise. `probe.py` remains the instrument for exact values;
   `read_image` is what tells you which question to ask it.
2. **`subagent`.** Eight children run concurrently (`active child limit: 8`); a ninth call
   returns `subagent limit reached`. Children cannot spawn grandchildren. Each child needs a
   self-contained prompt: paths, constraints, deliverable, report format, word limit.
3. **`create_goal`.** A goal outlives a compaction. `get_goal` returns `{"goal":null}` when
   none is active for the session; `create_goal` then sets one, and `update_goal` needs the
   `revision` from `get_goal`.

## Gotchas that cost time

- `read_image` takes `file_path`, not `path`. Passing `path` fails validation.
- `bash` requires `description`; a missing one is rejected before the command runs.
- Template literals inside template literals, with escaped backticks, are a reliable syntax
  error. Build long prompt strings with `String.raw` or plain concatenation.
- Background processes started from `run_code` die when the program returns. Long-lived
  things (Xvfb, the reference client) belong inside a Docker container, which survives.
- The live tool result for an image is the *attachment*; do not print base64 into stdout, it
  spills and is unreadable.
