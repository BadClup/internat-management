import 'package:bloc/bloc.dart';
import 'package:equatable/equatable.dart';
import 'package:internat_management/models/user.dart';

part "user_event.dart";

part 'user_state.dart';

class UserBloc extends Bloc<UserEvent, UserState> {
  UserBloc() : super(const UserState(user: User())) {

    on<InitUser>((event, emit) async {
      StorageData data = await User.getFromStorage();

      if (data.bearerToken != null && data.user != null) {
        emit(UserState(user: data.user!, bearerToken: data.bearerToken));
      }
    });

    on<LoginUser>((event, emit) async {
      try {

        final data = await User.loginUser(event.username, event.password);
        final user = data.user;

        await user.writeToStorage(data.bearerToken);

        emit(UserState(user: user, error: "", bearerToken: data.bearerToken));
      } catch (e) {
        emit(const UserState(user: User(), error: "Could not login user", bearerToken: null));
      }
    });

    on<LogoutUser>((event, emit) async {
      await User.clearStorage();

      emit(const UserState(
        user: User(),
        bearerToken: null,
        error: null,
      ));
    });
  }
}
