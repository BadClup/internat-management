import 'dart:convert';

String convertToUtf8(String str) {
  var utf8runes = str.runes.toList();
  return utf8.decode(utf8runes);
}
