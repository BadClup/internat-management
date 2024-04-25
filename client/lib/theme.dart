import 'package:flutter/material.dart';

class AppColors {
  static Color primaryColor = const Color.fromRGBO(103, 80, 164, 1);
  static Color primaryAccent = const Color.fromRGBO(79, 55, 139, 1);
  static Color textColor = const Color.fromRGBO(29, 27, 32, 1);
  static Color error = const Color.fromRGBO(179, 38, 30, 1);
}

ThemeData lightTheme = ThemeData(
  colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
  appBarTheme: AppBarTheme(
    backgroundColor: Colors.deepPurple[100],
  ),
  bottomAppBarTheme: BottomAppBarTheme(

  ),
  inputDecorationTheme: InputDecorationTheme(
      border: const OutlineInputBorder(
          borderRadius: BorderRadius.all(Radius.circular(15))),
      fillColor: AppColors.primaryColor,
      prefixIconColor: AppColors.primaryColor
  ),

);
