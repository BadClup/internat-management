import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../../blocs/chat/chat_bloc.dart';
import '../../blocs/theme/theme_bloc.dart';
import '../../blocs/user/user_bloc.dart';
import '../../models/theme.dart';

class SendMessagebox extends StatelessWidget {
  SendMessagebox({required this.residentId ,super.key});

  final int residentId;
  final _messageTextFieldController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<UserBloc, UserState>(
      builder: (context, state) {
        return Container(
          height: 70,
          color:
              AppTheme.darkTheme == context.watch<ThemeBloc>().state.themeData
                  ? AppColors.primaryAccent
                  : Colors.deepPurple[200],
          child: Padding(
            padding: const EdgeInsets.symmetric(vertical: 10, horizontal: 20),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              crossAxisAlignment: CrossAxisAlignment.center,
              children: [
                Expanded(
                    child: TextField(controller: _messageTextFieldController)),
                Padding(
                  padding: const EdgeInsets.only(left: 20.0),
                  child: IconButton.filled(
                      onPressed: () {
                        final content = _messageTextFieldController.text.trim();

                        context.read<ChatBloc>().add(SendMessage(
                            content: content,
                            bearerToken: state.bearerToken!,
                            residentId: residentId));
                      },
                      icon: const Icon(Icons.send)),
                ),
              ],
            ),
          ),
        );
      },
    );
  }
}
