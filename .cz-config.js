module.exports = {
  types: [
    { value: 'feat', name: 'feat:     Новая функциональность' },
    { value: 'fix', name: 'fix:      Исправление ошибки' },
    { value: 'docs', name: 'docs:     Обновление документации' },
    { value: 'style', name: 'style:    Изменения, не влияющие на логику' },
    { value: 'refactor', name: 'refactor: Рефакторинг кода' },
    { value: 'perf', name: 'perf:     Улучшение производительности' },
    { value: 'test', name: 'test:     Тесты' },
    { value: 'build', name: 'build:    Сборка и зависимости' },
    { value: 'ci', name: 'ci:       CI/CD настройки' },
    { value: 'chore', name: 'chore:    Прочие изменения' },
    { value: 'revert', name: 'revert:   Откат изменений' }
  ],

  scopes: [
    { name: 'parser' },
    { name: 'lexer' },
    { name: 'core' },
    { name: 'cli' },
    { name: 'config' },
    { name: 'ast' },
    { name: 'types' },
    { name: 'utils' },
    { name: 'custom' }
  ],

  allowCustomScopes: true,
  allowBreakingChanges: ['feat', 'fix'],
  subjectLimit: 100,

  messages: {
    type: 'Выберите тип изменения:',
    scope: 'Выберите область (scope):',
    customScope: 'Введите кастомную область:',
    subject: 'Краткое описание в повелительном наклонении:\n',
    body: 'Подробное описание (опционально):\n',
    breaking: 'BREAKING CHANGES (опционально):\n',
    footer: 'Issues (опционально, например: #123):\n',
    confirmCommit: 'Подтвердить коммит?'
  },

  formatSubject: function (subject, answers) {
    // ТОЧНО ТАК - с квадратными скобками для type
    if (answers.scope && answers.scope !== 'custom') {
      return `[${answers.type}](${answers.scope}): ${subject}`;
    }
    if (answers.customScope) {
      return `[${answers.type}](${answers.customScope}): ${subject}`;
    }
    return `[${answers.type}]: ${subject}`;
  }
};