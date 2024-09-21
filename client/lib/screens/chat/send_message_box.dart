import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../../blocs/chat/chat_bloc.dart';
import '../../blocs/theme/theme_bloc.dart';
import '../../blocs/user/user_bloc.dart';
import '../../models/theme.dart';

class SendMessagebox extends StatelessWidget {
  SendMessagebox({required this.residentId, super.key});

  final int residentId;
  final _messageTextFieldController = TextEditingController();

  @override
  Widget build(BuildContext context) {
    final isDarkMode =
        AppTheme.darkTheme == context.watch<ThemeBloc>().state.themeData;

    return BlocBuilder<UserBloc, UserState>(
      builder: (context, state) {
        return SizedBox(
          height: 70,
          child: Padding(
            padding: const EdgeInsets.symmetric(vertical: 10, horizontal: 20),
            child: Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              crossAxisAlignment: CrossAxisAlignment.center,
              children: [
                Expanded(
                    child: TextField(
                  controller: _messageTextFieldController,
                  decoration: InputDecoration(
                    hintText: 'Wpisz wiadomość',
                    filled: true,
                    fillColor: isDarkMode
                        ? AppColors.backgroundColor.dark
                        : AppColors.backgroundColor.light,
                    border: OutlineInputBorder(
                      borderRadius: BorderRadius.circular(15),
                      borderSide: BorderSide.none,
                    ),
                  ),
                )),
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
                      icon: Icon(Icons.send, color: isDarkMode ? AppColors.primaryColor.main : AppColors.primaryColor.dark)),
                ),
              ],
            ),
          ),
        );
      },
    );
  }
}
