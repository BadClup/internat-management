import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:internat_management/theme.dart';

import '../../blocs/theme/theme_bloc.dart';

List<ThemeData> themes = [lightTheme, darkTheme];

Future<void> themeDialogBuilder(BuildContext context) {
  return showDialog(
      context: context,
      builder: (BuildContext context) {
        return AlertDialog(
            title: const Text("Wybierz motyw"),
            content: BlocBuilder<ThemeBloc, ThemeState>(
              builder: (context, state) {
                return SizedBox(
                  height: 100.0,
                  child: Column(
                    children: [
                      RadioListTile(
                        value: themes[0],
                        groupValue: state.themeData,
                        onChanged: (value) {
                          if (value != null) {
                            context
                                .read<ThemeBloc>()
                                .add(ChangeTheme(themeData: value));
                          }
                        },
                        title: const Text("Jasny"),
                      ),
                      RadioListTile(
                        value: themes[1],
                        groupValue: state.themeData,
                        onChanged: (value) {
                          if (value != null) {
                            context
                                .read<ThemeBloc>()
                                .add(ChangeTheme(themeData: value));
                          }
                        },
                        title: const Text("Ciemny"),
                      )
                    ],
                  ),
                );
              },
            ));
      });
}
