# Что такое fh (find-here)?
Аналог программы grep написанный на Rust.

## Примеры
Найти все строки, где встречается значение `"some value"` с игнорированием регистра
```sh
cat file.txt | fh "some value"
```

Найти все строки, где встречается значение `"some value"` без игнорирования регистра
```sh
cat file.txt | fh -r "some value"
```

Найти все строки, где встречается значение `"some value"` и показать номер строки
```sh
cat file.txt | fh -i "some value"
```

## Конфиг
Если хотите изменить цвета, который использует программа, вы можете создать конфиг файл `/etc/fh-config/config.json`. Пример конфига:
```json
{
  "background": false,
  "underline": true,
  "color": "Green" // Blue, Green, Red, None
}
```
