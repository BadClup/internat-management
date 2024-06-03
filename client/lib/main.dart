import 'package:flutter/material.dart';
import 'package:internat_management/blocs/chat/chat_bloc.dart';
import 'package:internat_management/blocs/theme/theme_bloc.dart';
import 'package:internat_management/blocs/user/user_bloc.dart';
import 'package:internat_management/router.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_dotenv/flutter_dotenv.dart';

Future main() async {
  await dotenv.load(fileName: ".env");

  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MultiBlocProvider(
        providers: [
          BlocProvider(create: (context) => UserBloc()..add(const InitUser())),
          BlocProvider(create: (context) => ThemeBloc()..add(InitTheme())),
          BlocProvider(create: (context) => ChatBloc()),
        ],
        child: BlocBuilder<ThemeBloc, ThemeState>(
          builder: (context, state) {
            return MaterialApp.router(
              routerConfig: router,
              theme: state.themeData,
            );
          },
        ));
  }
}
