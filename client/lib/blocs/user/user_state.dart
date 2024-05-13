part of "user_bloc.dart";

class UserState extends Equatable {

  final User user;
  final String? bearerToken;
  final UserErrors error;
  final bool isLoading;

  const UserState({required this.user, this.bearerToken, required this.error, this.isLoading = false});

  @override
  List get props => [user, bearerToken, error, isLoading];
}