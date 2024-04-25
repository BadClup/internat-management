import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:internat_management/blocs/user/user_bloc.dart';
import 'package:internat_management/theme.dart';

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
    return Scaffold(
      body: Padding(
        padding: const EdgeInsets.all(16),
        child: Container(
          padding: const EdgeInsets.all(16),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              TextField(
                controller: _usernameController,
                decoration: const InputDecoration(
                    prefixIcon: Icon(Icons.person),
                    label: Text("Nazwa użytkownika")),
              ),
              const SizedBox(
                height: 20,
              ),
              TextField(
                controller: _passwordController,
                decoration: const InputDecoration(
                    prefixIcon: Icon(Icons.key), label: Text("Hasło")),
              ),
              if(context.read<UserBloc>().state.error != null)
                 Padding(padding: const EdgeInsets.only(top: 15),
                  child: Text("Nie udało się zalogować", style: TextStyle(
                    color: AppColors.error
                  ),),),
              const SizedBox(
                height: 20,
              ),
              Row(
                children: [
                  Expanded(
                    child: FilledButton(
                        onPressed: () {
                          final username = _usernameController.text;
                          final password = _passwordController.text;

                          if (username.isEmpty || password.isEmpty) {
                            // TODO: show user error
                            return;
                          }

                          context.read<UserBloc>().add(LoginUser(
                              username: username, password: password));
                        },
                        child: const Text("Zaloguj się")),
                  ),
                ],
              )
            ],
          ),
        ),
      ),
    );
  }
}
