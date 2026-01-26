## Быстрый путеводитель по сборке

- `npm run build:native` — собирает native-часть (`compiler/index.node`). Использует внутренний `compiler` пакет.
- `npm run build:native:debug` — то же, но с дебаг-инфой.
- `npm run demo` — компилирует тестовый JS (`tsc -p tsconfig.json`, затем `node test.js`).
- `npm run demo:ts` — запускает `test.ts` напрямую через `ts-node/esm` с подключённым трансформером `heavy.cjs`.
- `npm run patch` — ставит `ts-patch`, чтобы TypeScript использовал трансформер.

### Как добавить новый сценарий
1. Создайте профиль в `config/` (например, `tsconfig.experiment.json`, расширяющий `tsconfig.base.json`).
2. Добавьте npm-скрипт, который ссылается на ваш профиль:  
   `\"demo:exp\": \"npx tsc -p config/tsconfig.experiment.json\"`.
3. Используйте его так же, как стандартные команды.

### Где менять общие опции
- Базовые настройки: `config/tsconfig.base.json`.
- Опции для heavy: `config/tsconfig.heavy.json`.
Меняйте их, не трогая корневой `tsconfig.json` — он нужен только как «указатель» по умолчанию.
