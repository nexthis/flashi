const words: any = {}

function define(style: any, dict: any) {
    for (let i = 0; i < dict.length; i++) {
        words[dict[i]] = style
    }
}

const commonAtoms = ["true", "false"]
const commonKeywords = [
    "if",
    "do",
    "else",
    "elseif",
    "while",
    "for",
    "in",
    "fn",
    "fin",
    "array",
    "done",
    "exit",
    "set",
    "unset",
    "function",
    "goto",
]
const commonCommands = [
    "echo",
    "eval",
    "is_command_defined",
    "noop",
    "not",
    "print",
    "println",
    "read",
    "release",
    "man",
    "array_clear",
    "array_concat",
    "array_contains",
    "array_get",
    "array_is_empty",
    "array_join",
    "arrlen",
    "array_size",
    "array_length",
    "array_pop",
    "array_add",
    "array_put",
    "array_push,",
    "array_add,",
    "array_remove",
    "array_set",
    "is_array",
    "is_map",
    "is_set",
    "map",
    "map_clear",
    "map_contains_key",
    "map_cevalontains_valuemap_get",
    "map_is_empty",
    "map_keys",
    "map_load_properties",
    "map_add",
    "map_put",
    "map_remove",
    "map_size",
    "map_to_properties",
    "range",
    "read_properties",
    "set_new",
    "set_clear",
    "set_contains",
    "set_from_array",
    "set_is_empty",
    "set_add",
    "set_put",
    "set_remove",
    "set_size",
    "set_to_array",
    "write_properties",
    "duckscript_sdk_version",
    "duckscript_version",
    "dump_instructions",
    "dump_state",
    "dump_variables",
    "env_to_map",
    "which",
    "get_cpu_count",
    "cpu_count",
    "get_home_dir",
    "os_family",
    "os_name",
    "os_release",
    "os_version",
    "get_user_name",
    "whoami",
    "get_env",
    "is_windows",
    "print_current_directory",
    "pwd",
    "printenv",
    "print_env",
    "set_current_dir",
    "set_current_directory",
    "cd",
    "set_env",
    "uname",
    "unset_env",
    "get_last_error",
    "get_last_error_line",
    "get_last_error_source",
    "set_error",
    "set_exit_on_error",
    "exit_on_error",
    "trigger_error",
    "appendfile",
    "cp_glob",
    "glob_cp",
    "cp",
    "mkdir",
    "touch",
    "rmdir",
    "rm",
    "is_path_exists",
    "canonicalize",
    "basename",
    "filesize",
    "get_file_size",
    "get_last_modified_time",
    "dirname",
    "gitignore_path_array",
    "globarray",
    "glob_array",
    "is_dir",
    "is_directory",
    "is_file",
    "is_path_newer",
    "is_readonly",
    "join_path",
    "ls",
    "mv",
    "cat",
    "read_binary_file",
    "readbinfile",
    "read_text_file",
    "readfile",
    "chmod",
    "chmod_glob",
    "glob_chmod",
    "temp_dir",
    "temp_file",
    "write_binary_file",
    "writebinfile",
    "write_text_file",
    "writefile",
    "digest",
    "sha256sum",
    "sha512sum",
    "json_encode",
    "json_parse",
    "alias",
    "unalias",
    "remove_command",
    "calc",
    "greater_than",
    "hex_decode",
    "hex_encode",
    "less_than",
    "hostname",
    "http_client",
    "wget",
    "ftp_get",
    "ftp_get_in_memory",
    "ftp_list",
    "ftp_nlst",
    "ftp_put",
    "ftp_put_in_memory",
    "exec",
    "quit",
    "process_id",
    "pid",
    "spawn",
    "watchdog",
    "rand_range",
    "random_range",
    "random_text",
    "rand_text",
    "clear_scope",
    "scope_pop_stack",
    "scope_push_stack",
    "semver_is_equal",
    "semver_is_newer",
    "semver_parse",
    "base64",
    "base64_decode",
    "base64_encode",
    "bytes_to_string",
    "camelcase",
    "concat",
    "contains",
    "ends_with",
    "equals",
    "indexof",
    "is_empty",
    "kebabcase",
    "last_indexof",
    "length",
    "strlen",
    "lowercase",
    "replace",
    "snakecase",
    "split",
    "starts_with",
    "string_to_bytes",
    "substring",
    "trim",
    "trim_end",
    "trim_start",
    "uppercase",
    "assert",
    "assert_eq",
    "assert_error",
    "assert_fail",
    "assert_false",
    "test_directory",
    "test_file",
    "sleep",
    "current_time",
    "get_all_var_names",
    "get_by_name",
    "is_defined",
    "set_by_name",
    "unset_all_vars",
]

define("atom", commonAtoms)
define("keyword", commonKeywords)
define("builtin", commonCommands)

function tokenBase(stream: any, state: any) {
    if (stream.eatSpace()) return null

    const sol = stream.sol()
    const ch = stream.next()

    if (ch === "\\") {
        stream.next()
        return null
    }
    if (ch === "'" || ch === '"' || ch === "`") {
        state.tokens.unshift(tokenString(ch, ch === "`" ? "quote" : "string"))
        return tokenize(stream, state)
    }
    if (ch === "#") {
        if (sol && stream.eat("!")) {
            stream.skipToEnd()
            return "meta" // 'comment'?
        }
        stream.skipToEnd()
        return "comment"
    }
    if (ch === "$") {
        state.tokens.unshift(tokenDollar)
        return tokenize(stream, state)
    }
    if (ch === "+" || ch === "=") {
        return "operator"
    }
    if (ch === "-") {
        stream.eat("-")
        stream.eatWhile(/\w/)
        return "attribute"
    }
    if (ch == "<") {
        if (stream.match("<<")) return "operator"
        const heredoc = stream.match(/^<-?\s*['"]?([^'"]*)['"]?/)
        if (heredoc) {
            state.tokens.unshift(tokenHeredoc(heredoc[1]))
            return "string.special"
        }
    }
    if (/\d/.test(ch)) {
        stream.eatWhile(/\d/)
        if (stream.eol() || !/\w/.test(stream.peek())) {
            return "number"
        }
    }
    stream.eatWhile(/[\w-]/)
    const cur = stream.current()
    if (stream.peek() === "=" && /\w+/.test(cur)) return "def"
    // eslint-disable-next-line no-prototype-builtins
    return words.hasOwnProperty(cur) ? words[cur] : null
}

function tokenString(quote: any, style: any) {
    const close = quote == "(" ? ")" : quote == "{" ? "}" : quote
    return function (stream: any, state: any) {
        let next,
            escaped = false
        while ((next = stream.next()) != null) {
            if (next === close && !escaped) {
                state.tokens.shift()
                break
            } else if (next === "$" && !escaped && quote !== "'" && stream.peek() != close) {
                escaped = true
                stream.backUp(1)
                state.tokens.unshift(tokenDollar)
                break
            } else if (!escaped && quote !== close && next === quote) {
                state.tokens.unshift(tokenString(quote, style))
                return tokenize(stream, state)
            } else if (!escaped && /['"]/.test(next) && !/['"]/.test(quote)) {
                state.tokens.unshift(tokenStringStart(next, "string"))
                stream.backUp(1)
                break
            }
            escaped = !escaped && next === "\\"
        }
        return style
    }
}

function tokenStringStart(quote: any, style: any) {
    return function (stream: any, state: any) {
        state.tokens[0] = tokenString(quote, style)
        stream.next()
        return tokenize(stream, state)
    }
}

const tokenDollar = function (stream: any, state: any) {
    if (state.tokens.length > 1) stream.eat("$")
    const ch = stream.next()
    if (/['"({]/.test(ch)) {
        state.tokens[0] = tokenString(ch, ch == "(" ? "quote" : ch == "{" ? "def" : "string")
        return tokenize(stream, state)
    }
    if (!/\d/.test(ch)) stream.eatWhile(/\w/)
    state.tokens.shift()
    return "def"
}

function tokenHeredoc(delim: any) {
    return function (stream: any, state: any) {
        if (stream.sol() && stream.string == delim) state.tokens.shift()
        stream.skipToEnd()
        return "string.special"
    }
}

function tokenize(stream: any, state: any) {
    return (state.tokens[0] || tokenBase)(stream, state)
}

export const flashi = {
    name: "flashi",
    startState: function () {
        return { tokens: [] }
    },
    token: function (stream: any, state: any) {
        return tokenize(stream, state)
    },
    languageData: {
        autocomplete: commonAtoms.concat(commonKeywords, commonCommands),
        closeBrackets: { brackets: ["(", "[", "{", "'", '"', "`"] },
        commentTokens: { line: "#" },
    },
}
