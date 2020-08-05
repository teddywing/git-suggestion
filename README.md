git-suggestion
==============

Diffs and patches for GitHub pull request suggestions. Given a suggestion ID,
get a unified diff of the suggested change, or apply it directly to the working
copy in a Git repository.


## Examples

	$ git clone https://github.com/cli/cli.git
	Cloning into 'cli'...
	…
	$ cd cli/
	$ git checkout 74a39f3
	Note: checking out '74a39f3'.
	…
	$ git sugpatch https://github.com/cli/cli/pull/1150#discussion_r438352211
	diff --git a/pkg/cmd/api/api.go b/pkg/cmd/api/api.go
	index b4a8dbd..c081b50 100644
	--- a/pkg/cmd/api/api.go
	+++ b/pkg/cmd/api/api.go
	@@ -247,8 +247,7 @@ func readUserFile(fn string, stdin io.ReadCloser) ([]byte, error) {
		if fn == "-" {
			r = stdin
		} else {
	-		var err error
	-		r, err = os.Open(fn)
	+		r, err := os.Open(fn)
			if err != nil {
				return nil, err
			}
	$ git status
	HEAD detached at 74a39f3
	nothing to commit, working tree clean
	$ git sugapply 438352211
	$ git status
	HEAD detached at 74a39f3
	Changes not staged for commit:
	  (use "git add <file>..." to update what will be committed)
	  (use "git checkout -- <file>..." to discard changes in working directory)

		modified:   pkg/cmd/api/api.go

	no changes added to commit (use "git add" and/or "git commit -a")
	$ git diff
	diff --git a/pkg/cmd/api/api.go b/pkg/cmd/api/api.go
	index b4a8dbd..c081b50 100644
	--- a/pkg/cmd/api/api.go
	+++ b/pkg/cmd/api/api.go
	@@ -247,8 +247,7 @@ func readUserFile(fn string, stdin io.ReadCloser) ([]byte, error) {
		if fn == "-" {
			r = stdin
		} else {
	-		var err error
	-		r, err = os.Open(fn)
	+		r, err := os.Open(fn)
			if err != nil {
				return nil, err
			}


## Install
On Mac OS X, Git-Suggestion can be installed with Homebrew:

	$ brew install teddywing/formulae/git-suggestion

To compile from source or install on other platforms:

	$ cargo install --git https://github.com/teddywing/git-suggestion.git --root /usr/local


## Uninstall

	$ cargo uninstall --root /usr/local git-suggestion


## License
Copyright © 2020 Teddy Wing. Licensed under the GNU GPLv3+ (see the included
COPYING file).
