import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:internat_management/blocs/chat/chat_bloc.dart';
import 'package:internat_management/screens/chat/message_box.dart';
import 'package:internat_management/screens/chat/send_message_box.dart';
import 'package:internat_management/shared/navbar.dart';

class ChatScreen extends StatelessWidget {
  const ChatScreen({required this.residentId, super.key});

  final int residentId;

  @override
  Widget build(BuildContext context) {
    return BlocListener<ChatBloc, ChatState>(
      listener: (context, state) {
        final channel = state.wsChannel;

        if (channel != null) {
          context.read<ChatBloc>().add(ListenWebsocket(channel));
        }
      },
      child: Scaffold(
        appBar: const SharedAppBar(),
        body: Column(
          crossAxisAlignment: CrossAxisAlignment.center,
          mainAxisAlignment: MainAxisAlignment.end,
          children: [
            BlocBuilder<ChatBloc, ChatState>(builder: (context, state) {
              final messages = state.messages;

              if (state.isLoading) {
                return const Expanded(
                    child: Center(child: CircularProgressIndicator()));
              }

              if (state.error != null) {
                return const Expanded(
                    child:
                        Center(child: Text("Nie udało sie pobrać wiadomośći")));
              }

              if (messages != null && messages.isNotEmpty) {
                final messagesList = messages.map((message) {
                  return MessageBox(message: message);
                }).toList();

                return Expanded(
                  child: Container(
                    padding: const EdgeInsets.all(20),
                    child: ListView(
                      reverse: true,
                      children: List.from(messagesList),
                    ),
                  ),
                );
              }

              return Expanded(
                child: Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    Icon(
                      Icons.sentiment_dissatisfied,
                      color: Colors.grey[600],
                      size: 64,
                    ),
                    const Padding(padding: EdgeInsets.all(5)),
                    Text("Brak wiadomości do wyświetlenia",
                        style: TextStyle(color: Colors.grey[600]))
                  ],
                ),
              );
            }),
            SendMessagebox(
              residentId: residentId,
            )
          ],
        ),
      ),
    );
  }
}
