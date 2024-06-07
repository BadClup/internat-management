import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:go_router/go_router.dart';
import 'package:internat_management/blocs/chat/chat_bloc.dart';
import 'package:internat_management/screens/chat/chat.dart';
import 'package:internat_management/screens/chat/supervisor/chat_groups.dart';
import 'package:internat_management/screens/home/home.dart';
import 'package:internat_management/screens/login/login.dart';
import 'package:internat_management/screens/settings/settings.dart';
import 'package:internat_management/shared/scaffold_with_bottom_navbar.dart';
import 'package:internat_management/shared/scaffold_with_bottom_supervisor_navbar.dart';

import 'blocs/user/user_bloc.dart';

final router = GoRouter(initialLocation: "/login", routes: [
  GoRoute(path: "/login", builder: (context, state) => LoginScreen()),
  StatefulShellRoute.indexedStack(
      builder: (context, state, navigationShell) {
        return BlocListener<UserBloc, UserState>(
          listener: (context, state) {
            if (state.bearerToken == null) {
              context.go("/login");
            }
          },
          child: ScaffoldWithBottomNavBar(navigationShell: navigationShell),
        );
      },
      branches: [
        StatefulShellBranch(routes: [
          GoRoute(
              path: "/resident",
              builder: (context, state) => const HomeScreen()),
        ]),
        StatefulShellBranch(
          routes: [
            GoRoute(
                path: "/resident/chat",
                builder: (context, state) {
                  final userState = context.watch<UserBloc>().state;
                  final userId = userState.user.id;
                  final bearerToken = userState.bearerToken;

                  if (userId != null && bearerToken != null) {
                    context.read<ChatBloc>().add(
                        GetMessages(userId: userId, bearerToken: bearerToken));
                  }

                  return ChatScreen(residentId: userId!,);
                }),
          ],
        ),
        StatefulShellBranch(routes: [
          GoRoute(
              path: "/resident/room",
              builder: (context, state) => const HomeScreen()),
        ]),
        StatefulShellBranch(routes: [
          GoRoute(
              path: "/resident/announcements",
              builder: (context, state) => const HomeScreen()),
        ]),
        StatefulShellBranch(routes: [
          GoRoute(
              path: "/resident/settings",
              builder: (context, state) => const ProfileSettingsScreen()),
        ]),
      ]),
  StatefulShellRoute.indexedStack(
      builder: (context, state, navigationShell) {
        return BlocListener<UserBloc, UserState>(
          listener: (context, state) {
            if (state.bearerToken == null) {
              context.go("/login");
            }
          },
          child: ScaffoldWithBottomSupervisorNavbar(navigationShell: navigationShell),
        );
      },
      branches: [
        StatefulShellBranch(routes: [
          GoRoute(
              path: "/supervisor",
              builder: (context, state) => const HomeScreen()),
        ]),
        StatefulShellBranch(
          routes: [
            GoRoute(
                path: "/supervisor/chat",
                builder: (context, state) {

                  final bearerToken = context.watch<UserBloc>().state.bearerToken;
                  context.read<ChatBloc>().add(GetConversations(bearerToken: bearerToken!));

                  return const ChatGroups();
                }),
            GoRoute(
                path: "/supervisor/chat/:userId",
                builder: (context, state) {
                  final userState = context.watch<UserBloc>().state;
                  final parameter = state.pathParameters["userId"]!;

                  final userId = int.parse(parameter);
                  final bearerToken = userState.bearerToken;

                  if (bearerToken != null) {
                    context.read<ChatBloc>().add(
                        GetMessages(userId: userId, bearerToken: bearerToken));
                  }

                  return ChatScreen(residentId: userId);
                }),
          ],
        ),
        StatefulShellBranch(routes: [
          GoRoute(
              path: "/supervisor/room",
              builder: (context, state) => const HomeScreen()),
        ]),
        StatefulShellBranch(routes: [
          GoRoute(
              path: "/supervisor/announcements",
              builder: (context, state) => const HomeScreen()),
        ]),
        StatefulShellBranch(routes: [
          GoRoute(
              path: "/supervisor/settings",
              builder: (context, state) => const ProfileSettingsScreen()),
        ]),
      ])
]);
