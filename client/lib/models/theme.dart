import 'package:flutter/material.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

class AppColors {
  static Color primaryColor = const Color.fromRGBO(103, 80, 164, 1);
  static Color primaryAccent = const Color.fromRGBO(79, 55, 139, 1);
  static Color textColor = const Color.fromRGBO(29, 27, 32, 1);
  static Color error = const Color.fromRGBO(179, 38, 30, 1);
}

class AppTheme {
  static final ThemeData lightTheme = ThemeData(
    colorScheme: ColorScheme.fromSeed(
        seedColor: Colors.deepPurple, surface: Colors.white),
    appBarTheme: AppBarTheme(
      backgroundColor: Colors.deepPurple[100],
    ),
    inputDecorationTheme: InputDecorationTheme(
        border: const OutlineInputBorder(
            borderRadius: BorderRadius.all(Radius.circular(15))),
        fillColor: AppColors.primaryColor,
        prefixIconColor: AppColors.primaryColor),
  );
  static final ThemeData darkTheme = ThemeData(
    colorScheme: ColorScheme.fromSeed(
        seedColor: Colors.purple,
        surface: Colors.grey[700],
        brightness: Brightness.dark),
    appBarTheme: AppBarTheme(
      backgroundColor: Colors.deepPurple[900],
    ),
    inputDecorationTheme: InputDecorationTheme(
        border: const OutlineInputBorder(
            borderRadius: BorderRadius.all(Radius.circular(15))),
        fillColor: AppColors.primaryColor,
        prefixIconColor: AppColors.primaryColor),
  );

  static Future<ThemeData?> getTheme() async {
    const storage = FlutterSecureStorage();
    final theme = await storage.read(key: "theme");

    if (theme == "dark") {
      return darkTheme;
    }

    if(theme == "light") {
      return lightTheme;
    }

    return null;
  }

  static Future<void> saveTheme(ThemeData theme) async {
    const storage = FlutterSecureStorage();

    if (theme.brightness == Brightness.dark) {
      await storage.write(key: "theme", value: "dark");
    } else {
      await storage.write(key: "theme", value: "light");
    }
  }
}

