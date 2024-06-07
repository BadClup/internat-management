import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:go_router/go_router.dart';
import 'package:internat_management/blocs/user/user_bloc.dart';
import 'package:internat_management/models/user.dart';
import 'package:internat_management/models/theme.dart';

class LoginScreen extends StatelessWidget {
  LoginScreen({super.key});

  final _usernameController = TextEditingController();
  final _passwordController = TextEditingController();

  void dispose() {
    _usernameController.dispose();
    _passwordController.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return BlocListener<UserBloc, UserState>(
      listener: (context, state) {
        if (state.user.role == UserRole.resident) {
          context.go('/resident');
        }

        if (state.user.role == UserRole.supervisor) {
          context.go('/supervisor');
        }
      },
      child: Scaffold(
        body: Padding(
          padding: const EdgeInsets.all(16),
          child: Container(
            padding: const EdgeInsets.all(16),
            child: BlocBuilder<UserBloc, UserState>(
              builder: (context, state) {
                return Column(
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: [
                    TextField(
                      controller: _usernameController,
                      decoration: InputDecoration(
                          prefixIcon: const Icon(Icons.person),
                          label: const Text("Nazwa użytkownika"),
                          errorText: state.error.loginInputError),
                    ),
                    const SizedBox(
                      height: 20,
                    ),
                    TextField(
                      controller: _passwordController,
                      obscureText: true,
                      decoration: InputDecoration(
                          prefixIcon: const Icon(Icons.key),
                          label: const Text("Hasło"),
                          errorText: state.error.passwordInputError),
                    ),
                    state.error.mainError != null
                        ? Padding(
                            padding: const EdgeInsets.only(top: 15),
                            child: Text(
                              state.error.mainError!,
                              style: TextStyle(color: AppColors.error),
                            ),
                          )
                        : const SizedBox.shrink(),
                    const SizedBox(
                      height: 20,
                    ),
                    Row(
                      children: [
                        Expanded(
                            child: state.isLoading
                                ? const Padding(
                                    padding: EdgeInsets.symmetric(vertical: 10),
                                    child: CircularProgressIndicator(),
                                  )
                                : FilledButton(
                                    onPressed: () {
                                      final username =
                                          _usernameController.text.trim();
                                      final password =
                                          _passwordController.text.trim();

                                      context.read<UserBloc>().add(LoginUser(
                                          username: username,
                                          password: password));
                                    },
                                    child: const Text("Zaloguj się"))),
                      ],
                    )
                  ],
                );
              },
            ),
          ),
        ),
      ),
    );
  }
}
