import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:internat_management/models/theme.dart';
import 'package:internat_management/models/user.dart';

import '../blocs/theme/theme_bloc.dart';
import '../blocs/user/user_bloc.dart';

class SharedAppBar extends StatelessWidget implements PreferredSizeWidget {
  const SharedAppBar({super.key});

  @override
  Widget build(BuildContext context) {
    return AppBar(title: BlocBuilder<UserBloc, UserState>(
      builder: (BuildContext context, UserState state) {
        final selectedTheme = context.watch<ThemeBloc>().state.themeData;
        final isDarkMode = selectedTheme == AppTheme.darkTheme;
        final fullName = "${state.user.firstName} ${state.user.lastName}";
        final bottomText = state.user.role == UserRole.resident
            ? "Wychowanek. pokój nr. ${state.user.roomNumber}"
            : "Wychowawca";

        return Container(
          padding: const EdgeInsets.all(16),
          child: Row(
            children: [
              IconButton(
                  onPressed: () {},
                  color: isDarkMode ? AppColors.white : AppColors.primaryColor.main,
                  icon: const Icon(
                    Icons.account_circle,
                    size: 40,
                  )),
              const Padding(padding: EdgeInsets.only(left: 24)),
              Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Text(
                    fullName,
                    style: const TextStyle(
                        fontSize: 20, fontWeight: FontWeight.bold),
                  ),
                  Text(
                    bottomText,
                    style: const TextStyle(fontSize: 14),
                  ),
                ],
              ),
            ],
          ),
        );
      },
    ));
  }

  @override
  Size get preferredSize => const Size.fromHeight(kToolbarHeight);
}
