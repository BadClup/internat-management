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
          ListTile(
            leading: Icon(Icons.logout, color: AppColors.error.main,),
            title: const Text("Wyloguj"),
            subtitle: const Text("Kliknij aby wylogować się z aplikacji"),
            onTap: () {
              context.read<UserBloc>().add(const LogoutUser());
            },
          ),
        ],
      ),
    );
  }
}
