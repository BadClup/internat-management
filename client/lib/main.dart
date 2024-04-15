import 'package:flutter/material.dart';
import 'package:internat_management/blocs/user/user_bloc.dart';
import 'package:internat_management/models/user.dart';
import 'package:internat_management/router.dart';
import 'package:internat_management/screens/login/login.dart';
import 'package:internat_management/theme.dart';
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
        BlocProvider(
          create: (context) => UserBloc(),
        )
      ],
      child: BlocBuilder<UserBloc, UserState>(
        builder: (BuildContext context, UserState state) {
          if (state.bearerToken != null &&
              state.user.role == UserRole.resident) {
            return MaterialApp.router(
              routerConfig: residentRouter,
              theme: lightTheme,
            );
          }

          if (state.bearerToken != null &&
              state.user.role == UserRole.supervisor) {
            return MaterialApp.router(
              routerConfig: supervisorRouter,
              theme: lightTheme,
            );
          }

          return MaterialApp(
            home: LoginScreen(),
            theme: lightTheme,
          );
        },
      ),
    );
  }
}
