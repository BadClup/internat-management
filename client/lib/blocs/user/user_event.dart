part of "user_bloc.dart";

abstract class UserEvent extends Equatable {
  const UserEvent();

  @override
  List<Object> get props => [];
}


class LoginUser extends UserEvent {

  final String username;
  final String password;

  const LoginUser({required this.username, required this.password});

  @override
  List<Object> get props => [username, password];
}

class LogoutUser extends UserEvent {}
