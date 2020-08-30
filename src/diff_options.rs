// Copyright (c) 2020  Teddy Wing
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.


/// Git diff options.
static FLAGS: [&'static str; 97] = [
    "--abbrev",
    "--anchored",
    "--binary",
    "--break-rewrites",
    "--check",
    "--color",
    "--color-moved",
    "--color-moved-ws",
    "--color-words",
    "--compact-summary",
    "--cumulative",
    "--diff-algorithm",
    "--diff-filter",
    "--dirstat",
    "--dirstat-by-file",
    "--dst-prefix",
    "--exit-code",
    "--ext-diff",
    "--find-copies",
    "--find-copies-harder",
    "--find-object",
    "--find-renames",
    "--full-index",
    "--function-context",
    "--histogram",
    "--ignore-all-space",
    "--ignore-blank-lines",
    "--ignore-cr-at-eol",
    "--ignore-space-at-eol",
    "--ignore-space-change",
    "--ignore-submodules",
    "--indent-heuristic",
    "--inter-hunk-context",
    "--irreversible-delete",
    "--ita-invisible-in-index",
    "--line-prefix",
    "--minimal",
    "--name-only",
    "--name-status",
    "--no-color",
    "--no-color-moved",
    "--no-color-moved-ws",
    "--no-ext-diff",
    "--no-indent-heuristic",
    "--no-patch",
    "--no-prefix",
    "--no-relative",
    "--no-rename-empty",
    "--no-renames",
    "--no-stat",
    "--no-textconv",
    "--numstat",
    "--output",
    "--output-indicator-context",
    "--output-indicator-new",
    "--output-indicator-old",
    "--patch",
    "--patch-with-raw",
    "--patch-with-stat",
    "--patience",
    "--pickaxe-all",
    "--pickaxe-regex",
    "--quiet",
    "--raw",
    "--relative",
    "--rename-empty",
    "--shortstat",
    "--src-prefix",
    "--stat",
    "--submodule",
    "--summary",
    "--text",
    "--textconv",
    "--unified",
    "--word-diff",
    "--word-diff-regex",
    "--ws-error-highlight",
    "-B",
    "-C",
    "-D",
    "-G",
    "-M",
    "-O",
    "-R",
    "-S",
    "-U",
    "-W",
    "-X",
    "-a",
    "-b",
    "-l",
    "-p",
    "-s",
    "-t",
    "-u",
    "-w",
    "-z",
];


/// Parse Git diff options from `args`.
///
/// Returns a tuple containing `(args-that-are-not-diff-options, diff-options)`.
pub fn parse(args: &[String]) -> (Vec<&String>, Vec<&String>) {
    let mut program_args = Vec::new();
    let mut diff_args = Vec::new();

    'args: for arg in args {
        for flag in FLAGS.iter() {
            if arg.starts_with(flag) {
                diff_args.push(arg);

                continue 'args;
            }
        }

        program_args.push(arg)
    }

    (program_args, diff_args)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_extracts_diff_options() {
        let args = vec![
            "--github-token".to_owned(),
            "MY_TOKEN".to_owned(),
            "--diff-filter=A".to_owned(),
            "-D".to_owned(),
            "--color=always".to_owned(),
            "-U5".to_owned(),
            "--patience".to_owned(),
            "--ws-error-highlight=old,new".to_owned(),
            "--no-rename-empty".to_owned(),
            "--stat=50".to_owned(),
            "-M90%".to_owned(),
            "--relative".to_owned(),
        ];

        let (_, diff_opts) = parse(&args);

        assert_eq!(diff_opts, vec![
            "--diff-filter=A",
            "-D",
            "--color=always",
            "-U5",
            "--patience",
            "--ws-error-highlight=old,new",
            "--no-rename-empty",
            "--stat=50",
            "-M90%",
            "--relative",
        ]);
    }

    #[test]
    fn parse_does_not_consume_suggestion_args() {
        let args = vec![
            "--github-token".to_owned(),
            "MY_TOKEN".to_owned(),
            "--word-diff".to_owned(),
            "459692838".to_owned(),
            "https://github.com/teddywing/git-suggestion/pull/1#discussion_r459691747".to_owned(),
        ];

        let (options, diff_opts) = parse(&args);

        assert_eq!(diff_opts, vec!["--word-diff"]);
        assert_eq!(options, vec![
            "--github-token",
            "MY_TOKEN",
            "459692838",
            "https://github.com/teddywing/git-suggestion/pull/1#discussion_r459691747",
        ]);
    }
}
