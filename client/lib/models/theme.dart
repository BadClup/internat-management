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
    main: const Color.fromRGBO(103, 80, 164, 1), // purple
    light: const Color.fromRGBO(139, 116, 203, 1),
    dark: const Color.fromRGBO(79, 55, 139, 1),
  );

  static ColorTypes error = ColorTypes(
      main: const Color.fromRGBO(208, 68, 65, 1), // red
      light: const Color.fromRGBO(242, 132, 130, 1),
      dark: const Color.fromRGBO(172, 27, 24, 1));

  static ColorTypes grayColor = ColorTypes(
    light: const Color.fromRGBO(212, 212, 212, 1), // gray
    dark: const Color.fromRGBO(92, 92, 92, 1),
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
        surface: AppColors.backgroundColor.light),
    appBarTheme: AppBarTheme(
      backgroundColor: Colors.deepPurple[100],
    ),
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
      backgroundColor: Colors.deepPurple[800],
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
