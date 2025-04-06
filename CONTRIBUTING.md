# Style

Please run `cargo fmt` before sending pull requests.

# Output tests

The CI verifies that the PNG outputs do not unexpectedly change for existing
examples. This is done by checking out the repository before your PR and after
the PR and comparing the outputs. If you changed some examples deliberately,
you can indicate this in the PR description by adding a line like:

```
CHANGED_OUTPUTS=image1.png,image2.png
```

where `image1` etc is derived from the string you pass to the `c.show(&mut fg,
"image1");` line in the example. To run the tests manually you run these two
commands (requires [uv](https://github.com/astral-sh/uv) to be installed):

```bash
source setup_venv.sh  # Do this once
. venv/bin/activate   # Do this if you already set up venv
./cargo_util.py --make_golden_outputs  # On the base commit (typically master)
./cargo_util.py --test_outputs --ignore_new_outputs --changed_outputs=image1.png,image2.png  # With your changes applied
```

We don't check in the golden outputs because gnuplot does not guarantee
cross-platform pixel-perfect outputs, so the outputs end up being specific to
the platform they're generated on. Thus, we only compare two commits instead on
the same platform (i.e. your local machine, or the CI runner).

