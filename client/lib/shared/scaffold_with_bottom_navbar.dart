import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:go_router/go_router.dart';

import '../blocs/user/user_bloc.dart';
import '../models/navigation_option.dart';
import '../models/user.dart';

class ScaffoldWithBottomNavBar extends StatelessWidget {
  const ScaffoldWithBottomNavBar({
    required this.navigationShell,
    Key? key,
  }) : super(key: key ?? const ValueKey<String>('ScaffoldWithNavBar'));

  /// The navigation shell and container for the branch Navigators.
  final StatefulNavigationShell navigationShell;

  static List<NavOption> residentRoutes = [
    NavOption(
        route: "/resident", label: "Start", icon: const Icon(Icons.hexagon)),
    NavOption(
        route: "/resident/chat",
        label: "Czat",
        icon: const Icon(Icons.chat_bubble)),
    NavOption(
        route: "/resident/room",
        label: "Pokój",
        icon: const Icon(Icons.bookmark)),
    NavOption(
        route: "/resident/announcements",
        label: "Ogłoszenia",
        icon: const Icon(Icons.announcement)),
    NavOption(
        route: "/settings",
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
            destinations: residentRoutes.map((option) {
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
