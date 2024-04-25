import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

import '../models/navigation_option.dart';

class ScaffoldWithBottomNavBar extends StatelessWidget {
  const ScaffoldWithBottomNavBar({
    required this.navigationShell,
    Key? key,
  }) : super(key: key ?? const ValueKey<String>('ScaffoldWithNavBar'));

  /// The navigation shell and container for the branch Navigators.
  final StatefulNavigationShell navigationShell;

  static List<NavOption> navOptions = [
    NavOption(route: "/", label: "Start", icon: const Icon(Icons.hexagon)),
    NavOption(
        route: "/chat", label: "Czat", icon: const Icon(Icons.chat_bubble)),
    NavOption(route: "/room", label: "Pokój", icon: const Icon(Icons.bookmark)),
    NavOption(
        route: "/announcements",
        label: "Ogłoszenia",
        icon: const Icon(Icons.announcement)),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: navigationShell,
      bottomNavigationBar: NavigationBar(
        destinations: navOptions.map((option) {
          return NavigationDestination(icon: option.icon, label: option.label);
        }).toList(),
        selectedIndex: navigationShell.currentIndex,
        onDestinationSelected: (int index) {
          navigationShell.goBranch(
            index,
            initialLocation: index == navigationShell.currentIndex,
          );
        },
      ),
    );
  }
}
