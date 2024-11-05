import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:go_router/go_router.dart';
import 'package:internat_management/shared/default_app_bar.dart';

import '../../../blocs/chat/chat_bloc.dart';

class ChatGroups extends StatelessWidget {
  const ChatGroups({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: const SharedAppBar(),
      body: Column(
        children: [
          BlocBuilder<ChatBloc, ChatState>(
            builder: (context, state) {
              final conversations = state.conversations;

              if (state.isLoading) {
                return const Expanded(
                  child: Row(
                    mainAxisAlignment: MainAxisAlignment.center,
                    crossAxisAlignment: CrossAxisAlignment.center,
                    children: [CircularProgressIndicator()],
                  ),
                );
              }

              if (conversations != null) {
                final conversationList = conversations.map((conversation) {
                  final recentMessage = conversation.recentMessage;
                  final lastMessage = recentMessage ?? "Brak";

                  final fullName = "${conversation.recipient.firstName} ${conversation.recipient.lastName}";

                  return ListTile(
                    leading: const Icon(Icons.person),
                    title: Text(fullName),
                    subtitle: Text("Ostatnia wiadomość: $lastMessage"),
                    onTap: () {
                      context.go("/supervisor/chat", extra: conversation.recipient);
                    },
                  );
                }).toList();

                return Expanded(
                    child: ListView(
                  children: conversationList,
                ));
              }

              return const SizedBox();
            },
          )
        ],
      ),
    );
  }
}
