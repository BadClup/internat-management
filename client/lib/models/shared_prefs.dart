import 'package:shared_preferences/shared_preferences.dart';

class SharedPrefs {
  static SharedPreferences? _sharedPrefs;

  static Future<SharedPreferences> getInstance() async {
    _sharedPrefs ??= await SharedPreferences.getInstance();

    return _sharedPrefs!;
  }
}