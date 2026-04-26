module.exports = {
  types: [
    { value: "feat", name: "feat:     Добавление новой фичи" },
    { value: "fix", name: "fix:      Исправление бага" },
    { value: "docs", name: "docs:     Документация" },
    { value: "style", name: "style:    Форматирование кода" },
    { value: "refactor", name: "refactor: Рефакторинг" },
    { value: "test", name: "test:     Тесты" },
    { value: "chore", name: "chore:    Прочее" },
    { value: "wip", name: "wip:      Работа в процессе" }
  ],

  scopes: [
    { name: "runtime" },
    { name: "ast" },
    { name: "ir" },
    { name: "codegen" },
    { name: "compiler" },
    { name: "integration" },
    { name: "normalizer" },
    { name: "e2e" },
    { name: "parser" },
    { name: "tokenizer" },
    { name: "grammar" },
    { name: "error-handling" },
    { name: "optimizer" },
    { name: "cli" },
    { name: "gui" },
    { name: "docs" },
    { name: "tests" },
    { name: "infra" }
  ],

  allowCustomScopes: true,
  allowBreakingChanges: ["feat", "fix", "refactor"],
  skipQuestions: [],
  subjectLimit: 100,

  buildCommitMessage: (answers) => {
    const scope = answers.scope ? `(${answers.scope})` : "";
    return `${answers.type}${scope}: ${answers.subject}`;
  }
};
