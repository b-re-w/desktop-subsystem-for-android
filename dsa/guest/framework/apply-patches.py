"""Applies DSA's patches for one Android version to a tree fetch-source.py made.

    uv run dsa/guest/framework/apply-patches.py --android android17 --top <tree>
    uv run dsa/guest/framework/apply-patches.py --android android17 --top <tree> --check
    uv run dsa/guest/framework/apply-patches.py --android android17 --top <tree> --native

Each patches/<project>/<android>/NNNN-*.patch applies with `git apply` in <tree>/<project>, in
order. A patch that does not apply exactly stops it: a patch applied in part, or with fuzz to
other source than it was made for, is not the change that was reviewed. With --check nothing
is changed: the patches apply in order as in a build, since a later patch may change lines an
earlier one wrote, and are then taken back in reverse. <tree>/.dsa-patches.json then lists
what was applied.
"""

import argparse
import hashlib
import json
import os
import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
import framework_lock  # noqa: E402


class PatchError(RuntimeError):
    pass


def git_apply(project_dir, patch, reverse=False):
    # The project folder is no repository of its own (fetch-source.py drops each .git). Git must
    # not find one around it either: inside another repository, git apply would skip every
    # path outside the folder it runs in and report success.
    env = dict(os.environ, GIT_CEILING_DIRECTORIES=str(project_dir.parent))
    args = ["git", "apply", "--verbose"] + (["--reverse"] if reverse else []) + [str(patch)]
    result = subprocess.run(args, cwd=project_dir, env=env, capture_output=True, text=True)
    if result.returncode != 0:
        verb = "be taken back from" if reverse else "apply to"
        raise PatchError(f"{patch.name} does not {verb} {project_dir}:\n{result.stderr.strip()}")
    applied = [line for line in result.stderr.splitlines() if line.startswith("Applied patch")]
    if not applied:
        raise PatchError(f"{patch.name} changed nothing in {project_dir}")
    return applied


def apply_patches(android, top, check=False, patches=framework_lock.PATCHES, native=False):
    """Applies (or with `check` only tries) every patch for `android` under `top`, and returns
    what it applied as (project, patch name, SHA-256). With `native`, for the reduced tree the
    native files are built in, it leaves out the patches that change Java sources only: that
    tree has not their files, and the build makes no Java."""
    top = Path(top)
    found = framework_lock.patch_files(android, patches)
    if not found:
        raise PatchError(f"there are no patches for {android}")
    done = []
    applied = []
    try:
        for project, patch in found:
            if native and framework_lock.java_only(patch):
                print(f"{project}: {patch.name} changes Java sources only, for the services build", flush=True)
                continue
            project_dir = top / project
            if not project_dir.is_dir():
                raise PatchError(f"{project} is not in {top}, so {patch.name} has nothing to apply to")
            for line in git_apply(project_dir, patch):
                print(f"{project}: {line}", flush=True)
            applied.append((project_dir, patch))
            done.append((project, patch.name, hashlib.sha256(patch.read_bytes()).hexdigest()))
    finally:
        # A check leaves the tree as it found it, also when a patch stopped it.
        if check:
            for project_dir, patch in reversed(applied):
                git_apply(project_dir, patch, reverse=True)
    return done


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--android", required=True)
    parser.add_argument("--top", type=Path, required=True)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--native", action="store_true",
                        help="the reduced tree of the native build: leave out the patches to Java sources")
    args = parser.parse_args()
    try:
        done = apply_patches(args.android, args.top, args.check, native=args.native)
    except PatchError as e:
        raise SystemExit(str(e))
    if not args.check:
        (args.top / ".dsa-patches.json").write_text(json.dumps(
            [{"project": p, "patch": n, "sha256": s} for p, n, s in done], indent=2) + "\n")
    print(f"{len(done)} patches {'apply' if args.check else 'applied'} for {args.android}")


if __name__ == "__main__":
    sys.exit(main())
