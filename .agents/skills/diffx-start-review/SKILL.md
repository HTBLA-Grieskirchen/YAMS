---
name: diffx-start-review
description: "Start a code review session by launching the diffx server and opening the browser UI. Use when the user invokes /diffx-start-review."
user_invocable: true
---

# Start diffx Review

Launch the diffx server so the user can review their git changes in a browser-based UI and leave inline comments.

## What to do

### 1. Launch diffx

Run `diffx` in the background. By default it shows all working tree changes (staged + unstaged + untracked).

**Always exclude `graphify-out/`** from the served diff unless the user explicitly asks to include it. Do **not** change git config. Append a pathspec after `--`:

```bash
diffx -- -- . ':!graphify-out'
```

When the user already supplied git diff args, keep them and still exclude `graphify-out`:

```bash
diffx -- --staged -- . ':!graphify-out'
diffx -- HEAD~3 -- . ':!graphify-out'
diffx -- main..HEAD -- . ':!graphify-out'
diffx -- 54cf7fc61..d5f95d4e6 -- . ':!graphify-out'
```

Common variations:

```bash
diffx -- --staged -- . ':!graphify-out'   # Only staged changes
diffx -- HEAD~3 -- . ':!graphify-out'     # Last 3 commits
diffx -- main..HEAD -- . ':!graphify-out' # Current branch vs main
diffx -p 8080 -- -- . ':!graphify-out'    # Custom port (default: random available port)
```

Anything after `--` is passed directly to `git diff`, so any valid git diff arguments work. The `. ':!graphify-out'` pathspec is required on every launch.

**Important:** Run diffx in the background using the Bash tool with `run_in_background: true`, so the server stays alive while the user reviews.

### 2. Tell the user

After launching, tell the user:

> diffx is running. Review your changes in the browser and leave inline comments. When you're done, come back here and run `/diffx-finish-review`.

Keep it brief.
