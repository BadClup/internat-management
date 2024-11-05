import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:internat_management/models/chat.dart';
import 'package:intl/intl.dart';

import '../../blocs/theme/theme_bloc.dart';
import '../../blocs/user/user_bloc.dart';
import '../../models/theme.dart';
import '../../utils/convert_to_utf_8.dart';

class MessageBox extends StatelessWidget {
  const MessageBox({required this.message, super.key});

  final Message message;

  @override
  Widget build(BuildContext context) {
    final userId = context.watch<UserBloc>().state.user.id!;
    final selectedTheme = context.watch<ThemeBloc>().state.themeData;
    final content = convertToUtf8(message.content);

    final isDarkMode = selectedTheme == AppTheme.darkTheme;

    final messageMainColor = isDarkMode
        ? AppColors.primaryColor.dark
        : AppColors.primaryColor.main;

    final messageSecondaryColor = isDarkMode
        ? AppColors.grayColor.dark
        : AppColors.grayColor.light;

    DateTime createdAt = DateTime.parse(message.createdAt);
    final hour = DateFormat.Hm('pl').format(createdAt);
    final month = DateFormat.MMMd('pl').format(createdAt);
    final formattedDate = "$hour | $month";

    return Container(
      padding: const EdgeInsets.symmetric(vertical: 10),
      width: double.maxFinite,
      child: Row(
        mainAxisAlignment: userId == message.sender.id
            ? MainAxisAlignment.end
            : MainAxisAlignment.start,
        children: [
          SizedBox(
            width: 200,
            child: Column(
              crossAxisAlignment: userId == message.sender.id
                  ? CrossAxisAlignment.end
                  : CrossAxisAlignment.start,
              children: [
                Container(
                  padding: const EdgeInsets.symmetric(vertical: 6, horizontal: 16),
                  decoration: BoxDecoration(
                      color: userId == message.sender.id
                          ? messageMainColor
                          : messageSecondaryColor,
                      borderRadius: BorderRadius.circular(20)),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(content),
                      const Padding(padding: EdgeInsets.fromLTRB(0,5,0,0)),
                      Text(formattedDate,
                        style: TextStyle(color: isDarkMode ? AppColors.backgroundColor.light : AppColors.backgroundColor.dark,)),
                    ],
                  ),
                ),
                // Text(formattedDate),
              ],
            ),
          ),
        ],
      ),
    );
  }
}
