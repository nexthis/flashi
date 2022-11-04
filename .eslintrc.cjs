module.exports = {
    root: true,
    parser: "vue-eslint-parser",
    parserOptions: {
        parser: "@typescript-eslint/parser",
    },
    extends: [
        "plugin:vue/strongly-recommended",
        "eslint:recommended",
        "@vue/typescript/recommended",
        "plugin:import/errors",
        "plugin:import/warnings",
        "plugin:import/typescript",
        "prettier",
    ],
    plugins: ["@typescript-eslint", "prettier", "import"],
    rules: {
        "prettier/prettier": "error",
        // not needed for vue 3
        "vue/no-multiple-template-root": "off",
        "import/no-unresolved": "error",
        "import/order": [
            "error",
            {
                groups: ["builtin", "external", "parent", "sibling", "index"],
            },
        ],
    },

    settings: {
        "import/parsers": {
            "@typescript-eslint/parser": [".ts", ".tsx"],
        },
        "import/resolver": {
            typescript: {
                alwaysTryTypes: true, // always try to resolve types under `<root>@types` directory even it doesn't contain any source code, like `@types/unist`

                // Choose from one of the "project" configs below or omit to use <root>/tsconfig.json by default

                // use <root>/path/to/folder/tsconfig.json
                project: "tsconfig.json",
            },
        },
    },
}
