import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';

import '../blocs/user/user_bloc.dart';

class SharedAppBar extends StatelessWidget implements PreferredSizeWidget {
  const SharedAppBar({super.key});

  @override
  Widget build(BuildContext context) {
    return AppBar(title: BlocBuilder<UserBloc, UserState>(
      builder: (BuildContext context, UserState state) {
        return SizedBox(
            width: double.infinity,
            child: Text("${state.user.firstName} ${state.user.lastName}",
                textAlign: TextAlign.right,
                style: const TextStyle(
                  fontSize: 18,
                  fontWeight: FontWeight.w500
                ),
            ),
        );
      },
    ));
  }

  @override
  Size get preferredSize => const Size.fromHeight(kToolbarHeight);
}
