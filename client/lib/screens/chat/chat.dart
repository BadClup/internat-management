import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:internat_management/blocs/chat/chat_bloc.dart';
import 'package:internat_management/models/theme.dart';
import 'package:internat_management/screens/chat/sendMessageBox.dart';
import 'package:internat_management/shared/navbar.dart';

import '../../blocs/user/user_bloc.dart';

class ChatScreen extends StatelessWidget {
  const ChatScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: const SharedAppBar(),
      body: Column(
        crossAxisAlignment: CrossAxisAlignment.center,
        mainAxisAlignment: MainAxisAlignment.end,
        children: [
          BlocBuilder<ChatBloc, ChatState>(builder: (context, state) {
            final messages = state.messages;
            print(messages);

            if (state.isLoading) {
              return const Expanded(
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    CircularProgressIndicator(),
                  ],
                ),
              );
            }

            if (state.error != null) {
              return const Text("Nie udalo sie nam pobrać wiadomości z chatu");
            }

            if (messages != null) {
              final userId = context.watch<UserBloc>().state.user.id!;

              final messagesList = messages.map((message) {
                return Padding(
                  padding: const EdgeInsets.symmetric(vertical: 10),
                  child: Expanded(
                    child: Row(
                      mainAxisAlignment: userId == message.senderId
                          ? MainAxisAlignment.end
                          : MainAxisAlignment.start,
                      children: [
                        Container(
                          padding: const EdgeInsets.symmetric(
                              vertical: 6, horizontal: 16),
                          decoration: BoxDecoration(
                              color: AppColors.primaryAccent,
                              borderRadius: BorderRadius.circular(20)),
                          child: Text(message.content),
                        )
                      ],
                    ),
                  ),
                );
              }).toList();

              return Expanded(
                child: Padding(
                  padding: const EdgeInsets.all(20.0),
                  child: ListView(
                    children: List.from(messagesList.reversed),
                  ),
                ),
              );
            }

            return const SizedBox();
          }),
          SendMessagebox()
        ],
      ),
    );
  }
}
