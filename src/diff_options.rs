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


static FLAGS: [&'static str; 59] = [
    "-p",
    "--no-stat",
    "-p",
    "-u",
    "--patch",
    "-s",
    "--no-patch",
    "--raw",
    "--patch-with-raw",
    "-t",
    "--indent-heuristic",
    "--no-indent-heuristic",
    "--minimal",
    "--patience",
    "--histogram",
    "--compact-summary",
    "--numstat",
    "--shortstat",
    "--cumulative",
    "--summary",
    "--patch-with-stat",
    "-z",
    "--name-only",
    "--name-status",
    "--no-color",
    "--no-color-moved",
    "--no-color-moved-ws",
    "--no-renames",
    "--rename-empty",
    "--no-rename-empty",
    "--check",
    "--full-index",
    "--binary",
    "--find-copies-harder",
    "-D",
    "--irreversible-delete",
    "--pickaxe-all",
    "--pickaxe-regex",
    "-R",
    "--no-relative",
    "-a",
    "--text",
    "--ignore-cr-at-eol",
    "--ignore-space-at-eol",
    "-b",
    "--ignore-space-change",
    "-w",
    "--ignore-all-space",
    "--ignore-blank-lines",
    "-W",
    "--function-context",
    "--exit-code",
    "--quiet",
    "--ext-diff",
    "--no-ext-diff",
    "--textconv",
    "--no-textconv",
    "--no-prefix",
    "--ita-invisible-in-index",
];

static ARG_OPTIONS: [&'static str; 20] = [
    "-U",
    "--unified",
    "--output",
    "--output-indicator-new",
    "--output-indicator-old",
    "--output-indicator-context",
    "--anchored",
    "--diff-algorithm",
    "--color-moved-ws",
    "--word-diff-regex",
    "--ws-error-highlight",
    "-l",
    "-S",
    "-G",
    "--find-object",
    "-O",
    "--inter-hunk-context",
    "--src-prefix",
    "--dst-prefix",
    "--line-prefix",
];

static OPT_OPTIONS: [&'static str; 19] = [
    "--stat",
    "-X",
    "--dirstat",
    "--dirstat-by-file",
    "--submodule",
    "--color",
    "--color-moved",
    "--word-diff",
    "--color-words",
    "--abbrev",
    "-B",
    "--break-rewrites",
    "-M",
    "--find-renames",
    "-C",
    "--find-copies",
    "--diff-filter",
    "--relative",
    "--ignore-submodules",
];


pub fn parse(args: &[String]) -> (Vec<&String>, Vec<&String>) {
    let mut program_args = Vec::new();
    let mut found_args = Vec::new();
    let mut add_next_arg = false;

    'args: for arg in args {
        let find_arg_prefix = arg.find('-');

        if add_next_arg
            && (
                find_arg_prefix.is_none()
                || find_arg_prefix != Some(0)
            )
        {
            found_args.push(arg);

            add_next_arg = false;

            continue;
        }

        for flag in FLAGS.iter() {
            if arg.starts_with(flag) {
                found_args.push(arg);

                continue 'args;
            }
        }

        for option in &ARG_OPTIONS {
            if arg.starts_with(option) {
                found_args.push(arg);

                continue 'args;
            }
        }

        for option in &OPT_OPTIONS {
            if arg.starts_with(option) {
                found_args.push(arg);

                continue 'args;
            }
        }

        program_args.push(arg)
    }

    (program_args, found_args)
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
