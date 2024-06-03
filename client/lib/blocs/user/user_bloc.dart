import 'package:bloc/bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:internat_management/models/errors.dart';
import 'package:internat_management/models/user.dart';

part "user_event.dart";

part 'user_state.dart';

class UserBloc extends Bloc<UserEvent, UserState> {
  UserBloc() : super(UserState(user: const User(), error: UserErrors())) {

    on<InitUser>((event, emit) async {
      StorageData data = await User.getFromStorage();

      if (data.bearerToken != null && data.user != null) {
        emit(UserState(user: data.user!, bearerToken: data.bearerToken, error: UserErrors()));
      }
    });

    on<LoginUser>((event, emit) async {
      try {

        if(event.username.isEmpty) {
          emit(UserState(user: const User(), error: UserErrors(loginInputError: "Nazwa użytkownika nie może być pusta")));
          return;
        }

        if(event.password.isEmpty) {
          emit(UserState(user: const User(), error: UserErrors(passwordInputError: "Hasło nie może być puste")));
          return;
        }

        emit(UserState(user: const User(), isLoading: true, error: UserErrors()));

        final data = await User.loginUser(event.username, event.password);
        final user = data.user;

        await user.writeToStorage(data.bearerToken);

        emit(UserState(user: user, bearerToken: data.bearerToken, error: UserErrors()));
      } catch (e) {
        emit(UserState(user: const User(), error: UserErrors(mainError: "Nie udało nam się ciebie zalgować")));
      }
    });

    on<LogoutUser>((event, emit) async {
      await User.clearStorage();

      emit(UserState(
        user: const User(),
        error: UserErrors(),
      ));
    });
  }
}
