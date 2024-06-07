import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:internat_management/blocs/chat/chat_bloc.dart';
import 'package:internat_management/models/theme.dart';
import 'package:internat_management/screens/chat/send_message_box.dart';
import 'package:internat_management/shared/navbar.dart';
import 'package:internat_management/utils/convert_to_utf_8.dart';

import '../../blocs/user/user_bloc.dart';

class ChatScreen extends StatelessWidget {
  const ChatScreen({required this.residentId, super.key});

  final int residentId;

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

                final content = convertToUtf8(message.content);

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
                          child: Text(content),
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
          SendMessagebox(residentId: residentId,)
        ],
      ),
    );
  }
}
