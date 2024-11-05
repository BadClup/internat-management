import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:go_router/go_router.dart';
import 'package:internat_management/models/user.dart';

import '../blocs/user/user_bloc.dart';

class ConversationAppBar extends StatelessWidget
    implements PreferredSizeWidget {
  const ConversationAppBar({required this.conversationName, super.key});

  final String conversationName;

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<UserBloc, UserState>(
        builder: (BuildContext context, UserState state) {
      final user = state.user;

      return AppBar(
        title: Text(conversationName, style: TextStyle(fontWeight: FontWeight.w600),),
        leading: user.role == UserRole.supervisor
            ? IconButton(
                onPressed: () {
                  context.go("/supervisor/conversations");
                },
                icon: const Icon(Icons.arrow_back),
              )
            : null,
      );
    });
  }

  @override
  Size get preferredSize => const Size.fromHeight(kToolbarHeight);
}
