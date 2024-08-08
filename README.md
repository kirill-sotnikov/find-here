# Что такое fin (find-in)?
Аналог программы grep написанный на Rust.

## Примеры
Найти все строки, где встречается значение `"some value"` с игнорированием регистра
```sh
cat file.txt | fin "some value"
```

Найти все строки, где встречается значение `"some value"` без игнорирования регистра
```sh
cat file.txt | fin -r "some value"
```

Найти все строки, где встречается значение `"some value"` и показать номер строки
```sh
cat file.txt | fin -i "some value"
```

Найти все строки в файле, где встречается значение `"some value"` и показать номер строки
```sh
fin -f some_file.txt -i "some value"
```

## Конфиг
Если хотите изменить цвета, который использует программа, вы можете создать конфиг файл `/etc/fin-config/config.json`. Пример конфига:
```json
{
  "background": false,
  "underline": true,
  "color": "Green" // Blue, Green, Red, None
}
```
