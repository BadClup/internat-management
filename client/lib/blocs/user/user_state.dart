part of "user_bloc.dart";

class UserState extends Equatable {

  final User user;
  final bool isAuth;
  final String? error;

  const UserState({required this.user, required this.isAuth, this.error});

  @override
  List<Object> get props => [];
}