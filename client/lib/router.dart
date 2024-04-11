import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';
import 'package:internat_management/screens/chat/chat.dart';
import 'package:internat_management/screens/home/home.dart';
import 'package:internat_management/shared/scaffold_with_navbar.dart';

final residentRouter = GoRouter(routes: [
  StatefulShellRoute.indexedStack(
      builder: (context, state, navigationShell) =>
          ScaffoldWithNavBar(navigationShell: navigationShell),
      branches: [
        StatefulShellBranch(routes: [
          GoRoute(path: "/", builder: (context, state) => const HomeScreen()),
        ]),
        StatefulShellBranch(routes: [
          GoRoute(path: "/chat", builder: (context, state) => const ChatScreen()),
        ]),
        StatefulShellBranch(routes: [
          GoRoute(path: "/room", builder: (context, state) => const HomeScreen()),
        ]),
        StatefulShellBranch(routes: [
          GoRoute(path: "/announcements", builder: (context, state) => const HomeScreen()),
        ]),
      ]),
]);

final supervisorRouter = GoRouter(routes: [
  GoRoute(
      path: "/",
      builder: (context, state) => const Scaffold(
            body: Text("Admin panel"),
          ))
]);
