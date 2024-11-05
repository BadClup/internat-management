import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:go_router/go_router.dart';

import '../blocs/user/user_bloc.dart';
import '../models/navigation_option.dart';

class ScaffoldWithBottomSupervisorNavbar extends StatelessWidget {
  const ScaffoldWithBottomSupervisorNavbar(
      {required this.navigationShell, super.key});

  final StatefulNavigationShell navigationShell;

  static List<NavOption> supervisorRoutes = [
    NavOption(
        route: "/supervisor", label: "Start", icon: const Icon(Icons.hexagon)),
    NavOption(
        route: "/supervisor/conversations",
        label: "Czat",
        icon: const Icon(Icons.chat_bubble)),
    NavOption(
        route: "/supervisor/room",
        label: "Pokój",
        icon: const Icon(Icons.bookmark)),
    NavOption(
        route: "/supervisor/announcements",
        label: "Ogłoszenia",
        icon: const Icon(Icons.announcement)),
    NavOption(
        route: "/supervisor/settings",
        label: "Ustawienia",
        icon: const Icon(Icons.settings)),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: navigationShell,
      bottomNavigationBar: BlocBuilder<UserBloc, UserState>(
        builder: (context, state) {
          return NavigationBar(
            destinations: supervisorRoutes.map((option) {
              return NavigationDestination(
                  icon: option.icon, label: option.label);
            }).toList(),
            selectedIndex: navigationShell.currentIndex,
            onDestinationSelected: (int index) {
              navigationShell.goBranch(
                index,
                initialLocation: index == navigationShell.currentIndex,
              );
            },
          );
        },
      ),
    );
  }
}
