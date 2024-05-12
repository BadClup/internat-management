part of "user_bloc.dart";

class UserState extends Equatable {

  final User user;
  final String? bearerToken;
  final String? error;

  const UserState({required this.user, this.bearerToken, this.error});

  @override
  List get props => [user, bearerToken, error];
}