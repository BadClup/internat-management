part of "user_bloc.dart";

class UserState extends Equatable {

  final User user;
  final String? bearerToken;
  final String? error;
  final bool isLoading;

  const UserState({required this.user, this.bearerToken, this.error, this.isLoading = false});

  @override
  List get props => [user, bearerToken, error, isLoading];
}