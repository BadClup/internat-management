import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:internat_management/screens/settings/theme_dialog.dart';
import 'package:internat_management/shared/navbar.dart';
import 'package:internat_management/models/theme.dart';

import '../../blocs/user/user_bloc.dart';

class ProfileSettingsScreen extends StatelessWidget {
  const ProfileSettingsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: const SharedAppBar(),
      body: Column(
        children: [
          ListTile(
            leading: const Icon(Icons.format_paint),
            title: const Text("Motyw"),
            subtitle: const Text("Zmień wygląd aplikacji"),
            onTap: () {
              themeDialogBuilder(context);
            },
          ),
          SizedBox(
            width: double.infinity,
            child: Padding(
              padding: const EdgeInsets.all(30.0),
              child: TextButton(
                onPressed: () {
                  context.read<UserBloc>().add(const LogoutUser());
                },
                style: ButtonStyle(
                  backgroundColor: WidgetStateProperty.all(AppColors.error),
                ),
                child: const Text(
                  "Wyloguj",
                  style: TextStyle(color: Colors.white),
                ),
              ),
            ),
          )
        ],
      ),
    );
  }
}
