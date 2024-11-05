import 'package:flutter/material.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

class ColorTypes {
  Color? main;
  Color? light;
  Color? dark;

  ColorTypes({this.main, this.light, this.dark});
}

class AppColors {
  static ColorTypes primaryColor = ColorTypes(
    light: Colors.deepPurple[300],
    main: Colors.deepPurple[500], // purple
    dark: Colors.deepPurple[800],
  );

  static ColorTypes secondaryColor = ColorTypes(
    light: Colors.orange[300],
    main: Colors.orange[500], // orange
    dark: Colors.orange[800],
  );

  static ColorTypes success = ColorTypes(
      light: Colors.green[300],
      main: Colors.green[500], // green
      dark: Colors.green[800],
  );

  static ColorTypes error = ColorTypes(
      light: const Color.fromRGBO(242, 132, 130, 1),
      main: const Color.fromRGBO(208, 68, 65, 1), // red
      dark: const Color.fromRGBO(172, 27, 24, 1));

  static ColorTypes grayColor = ColorTypes(
    light: Colors.grey[300],
    main: Colors.grey[500],
    dark: Colors.grey[700],
  );

  static ColorTypes backgroundColor = ColorTypes(
    light: const Color.fromRGBO(240, 239, 244, 1),
    dark: const Color.fromRGBO(0, 0, 0, 1),
  );

  static Color white = const Color.fromRGBO(255, 255, 255, 1);
  static Color black = const Color.fromRGBO(0, 0, 0, 1);
}

class AppTheme {
  static final ThemeData lightTheme = ThemeData(
    colorScheme: ColorScheme.fromSeed(
        seedColor: AppColors.primaryColor.main!,
        surface: AppColors.white),
    inputDecorationTheme: InputDecorationTheme(
        border: const OutlineInputBorder(
            borderRadius: BorderRadius.all(Radius.circular(15))),
        fillColor: AppColors.primaryColor.main,
        prefixIconColor: AppColors.primaryColor.main),
  );
  static final ThemeData darkTheme = ThemeData(
    colorScheme: ColorScheme.fromSeed(
        seedColor: AppColors.primaryColor.main!,
        surface: AppColors.backgroundColor.dark,
        brightness: Brightness.dark),
    appBarTheme: AppBarTheme(
      backgroundColor: AppColors.primaryColor.dark,
    ),
    inputDecorationTheme: InputDecorationTheme(
        border: const OutlineInputBorder(
            borderRadius: BorderRadius.all(Radius.circular(15))),
        fillColor: AppColors.primaryColor.main,
        prefixIconColor: AppColors.primaryColor.main),
  );

  static Future<ThemeData?> getTheme() async {
    const storage = FlutterSecureStorage();
    final theme = await storage.read(key: "theme");

    if (theme == "dark") {
      return darkTheme;
    }

    if (theme == "light") {
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
